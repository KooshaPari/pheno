//! B9 — Kill-switch for the Bifrost gateway.
//!
//! Monitors Bifrost health metrics (latency, error rate, success rate,
//! consecutive failures) and trips when configurable thresholds are
//! exceeded. Once tripped, the caller (e.g. `FallbackRouter`) falls back
//! to `omni-router` until the switch is manually reset or automatically
//! recovers after a cooldown period.
//!
//! Mirrors the TypeScript `open-sse/services/bifrostKillSwitch.ts` contract.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;

use crate::error::{Error, Result};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Default p99 latency threshold (ms) — 5 s, matching the TS side.
pub const DEFAULT_LATENCY_THRESHOLD_MS: u64 = 5_000;

/// Default error-rate threshold (fraction) — 2 %.
pub const DEFAULT_ERROR_RATE_THRESHOLD: f64 = 0.02;

/// Default consecutive-failure threshold — 10 in a row.
pub const DEFAULT_CONSECUTIVE_FAILURE_LIMIT: u64 = 10;

/// Default cooldown before auto-reset (seconds) — 5 minutes.
pub const DEFAULT_COOLDOWN_SECS: u64 = 300;

/// Number of recent observations to keep for the sliding window.
const WINDOW_SIZE: usize = 1_000;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// A kill-switch that monitors Bifrost health and auto-trips on threshold
/// breaches. Thread-safe via atomics + a small RWLock-protected window.
pub struct BifrostKillSwitch {
    inner: Arc<BifrostKillSwitchInner>,
}

struct BifrostKillSwitchInner {
    tripped: AtomicBool,
    cooldown_secs: AtomicU64,
    latency_threshold_ms: AtomicU64,
    error_rate_threshold: AtomicU64,         // stored as f64 * 1e6
    consecutive_failure_limit: AtomicU64,
    consecutive_failures: AtomicU64,
    tripped_at: AtomicU64,                    // Instant-as-nanos snapshot
    recovery_after_ns: AtomicU64,
    // Sliding window: we store status as bool (true = success, false = failure).
    // A simple lock-free ring buffer would be ideal, but for simplicity we
    // use a Mutex<Vec<bool>>. The kill-switch is not on the hot path.
    window: std::sync::Mutex<Vec<bool>>,
}

impl BifrostKillSwitch {
    /// Create a new kill-switch with default thresholds.
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            inner: Arc::new(BifrostKillSwitchInner {
                tripped: AtomicBool::new(false),
                cooldown_secs: AtomicU64::new(DEFAULT_COOLDOWN_SECS),
                latency_threshold_ms: AtomicU64::new(DEFAULT_LATENCY_THRESHOLD_MS),
                error_rate_threshold: AtomicU64::new(
                    (DEFAULT_ERROR_RATE_THRESHOLD * 1_000_000.0) as u64,
                ),
                consecutive_failure_limit: AtomicU64::new(DEFAULT_CONSECUTIVE_FAILURE_LIMIT),
                consecutive_failures: AtomicU64::new(0),
                tripped_at: AtomicU64::new(0),
                recovery_after_ns: AtomicU64::new(0),
                window: std::sync::Mutex::new(Vec::with_capacity(WINDOW_SIZE)),
            }),
        })
    }

    /// Create a kill-switch with custom thresholds.
    #[must_use]
    pub fn with_thresholds(
        latency_ms: u64,
        error_rate: f64,
        consecutive_failures: u64,
        cooldown_secs: u64,
    ) -> Arc<Self> {
        Arc::new(Self {
            inner: Arc::new(BifrostKillSwitchInner {
                tripped: AtomicBool::new(false),
                cooldown_secs: AtomicU64::new(cooldown_secs),
                latency_threshold_ms: AtomicU64::new(latency_ms),
                error_rate_threshold: AtomicU64::new(
                    (error_rate * 1_000_000.0) as u64,
                ),
                consecutive_failure_limit: AtomicU64::new(consecutive_failures),
                consecutive_failures: AtomicU64::new(0),
                tripped_at: AtomicU64::new(0),
                recovery_after_ns: AtomicU64::new(0),
                window: std::sync::Mutex::new(Vec::with_capacity(WINDOW_SIZE)),
            }),
        })
    }

    // -- State queries --

    /// Whether the kill-switch is currently tripped.
    pub fn is_tripped(&self) -> bool {
        self.inner.tripped.load(Ordering::Relaxed)
    }

    /// Returns `Err(KillSwitchActive)` if tripped; otherwise `Ok(())`.
    /// Callers that want to short-circuit before dispatching to Bifrost
    /// should use `check()`.
    pub fn check(&self) -> Result<()> {
        if self.is_tripped() {
            return Err(Error::KillSwitchActive(
                "kill-switch is active; falling back to omni-router".to_string(),
            ));
        }
        Ok(())
    }

    // -- Metric ingestion --

    /// Record a successful call (latency in ms).
    pub fn record_success(&self, latency_ms: u64) {
        // Check latency threshold
        let threshold = self.inner.latency_threshold_ms.load(Ordering::Relaxed);
        if latency_ms > threshold {
            self.trip(&format!(
                "p99 latency {latency_ms}ms exceeded threshold {threshold}ms"
            ));
            return;
        }

        // Reset consecutive failures
        self.inner.consecutive_failures.store(0, Ordering::Relaxed);

        // Push to window
        if let Ok(mut w) = self.inner.window.lock() {
            if w.len() >= WINDOW_SIZE {
                w.remove(0);
            }
            w.push(true);
        }

        self.evaluate_window();
    }

    /// Record a failed call with the measured latency (ms).
    pub fn record_failure(&self, latency_ms: u64) {
        // Increment consecutive failures
        let limit = self.inner.consecutive_failure_limit.load(Ordering::Relaxed);
        let prev = self.inner.consecutive_failures.fetch_add(1, Ordering::Relaxed) + 1;
        if prev >= limit {
            self.trip(&format!("{prev} consecutive failures"));
            return;
        }

        // Push to window
        if let Ok(mut w) = self.inner.window.lock() {
            if w.len() >= WINDOW_SIZE {
                w.remove(0);
            }
            w.push(false);
        }

        // Also check latency on failure
        let threshold = self.inner.latency_threshold_ms.load(Ordering::Relaxed);
        if latency_ms > threshold * 3 {
            // A failure that took >3x the latency threshold is pathological
            self.trip(&format!(
                "failure with extreme latency {latency_ms}ms"
            ));
            return;
        }

        self.evaluate_window();
    }

    // -- Manual control --

    /// Manually trip the kill-switch.
    pub fn trip(&self, reason: &str) {
        self.inner.tripped.store(true, Ordering::Release);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos() as u64;
        self.inner.tripped_at.store(now, Ordering::Relaxed);
        let cooldown_ns =
            self.inner.cooldown_secs.load(Ordering::Relaxed) * 1_000_000_000;
        self.inner
            .recovery_after_ns
            .store(now + cooldown_ns, Ordering::Relaxed);
        tracing::warn!(
            "bifrost kill-switch tripped: {reason} (cooldown {}s)",
            self.inner.cooldown_secs.load(Ordering::Relaxed)
        );
    }

    /// Manually reset the kill-switch.
    pub fn reset(&self) {
        self.inner.tripped.store(false, Ordering::Release);
        self.inner.consecutive_failures.store(0, Ordering::Relaxed);
        self.inner.tripped_at.store(0, Ordering::Relaxed);
        self.inner.recovery_after_ns.store(0, Ordering::Relaxed);
        if let Ok(mut w) = self.inner.window.lock() {
            w.clear();
        }
        tracing::info!("bifrost kill-switch manually reset");
    }

    // -- Internal helpers --

    fn evaluate_window(&self) {
        if self.is_tripped() {
            // Check if cooldown has expired -> auto-reset
            let recovery = self.inner.recovery_after_ns.load(Ordering::Relaxed);
            if recovery > 0 {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos() as u64;
                if now >= recovery {
                    self.reset();
                    tracing::info!("bifrost kill-switch auto-reset after cooldown");
                }
            }
            return;
        }

        // Check sliding-window error rate
        let Ok(w) = self.inner.window.lock() else { return };
        if w.is_empty() {
            return;
        }
        let failures = w.iter().filter(|&&s| !s).count() as f64;
        let total = w.len() as f64;
        let rate = failures / total;
        let threshold_f64 =
            self.inner.error_rate_threshold.load(Ordering::Relaxed) as f64 / 1_000_000.0;
        if rate > threshold_f64 && total >= 10.0 {
            // Need at least 10 observations before tripping on rate
            drop(w); // release before trip logs
            self.trip(&format!(
                "error rate {:.1}% exceeded threshold {:.1}%",
                rate * 100.0,
                threshold_f64 * 100.0
            ));
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fresh_switch_is_not_tripped() {
        let ks = BifrostKillSwitch::new();
        assert!(!ks.is_tripped());
        assert!(ks.check().is_ok());
    }

    #[test]
    fn test_manual_trip_and_reset() {
        let ks = BifrostKillSwitch::new();
        ks.trip("test");
        assert!(ks.is_tripped());
        assert!(ks.check().is_err());

        ks.reset();
        assert!(!ks.is_tripped());
        assert!(ks.check().is_ok());
    }

    #[test]
    fn test_latency_threshold_trips_switch() {
        let ks = BifrostKillSwitch::with_thresholds(100, 0.5, 10, 300);
        // Record a success with latency above the 100ms threshold
        ks.record_success(200);
        assert!(ks.is_tripped(), "expected trip on latency exceed");
    }

    #[test]
    fn test_consecutive_failures_trip_switch() {
        let ks = BifrostKillSwitch::with_thresholds(5000, 0.5, 3, 300);
        for _ in 0..3 {
            ks.record_failure(100);
        }
        assert!(
            ks.is_tripped(),
            "expected trip after 3 consecutive failures"
        );
    }

    #[test]
    fn test_normal_latency_does_not_trip() {
        let ks = BifrostKillSwitch::with_thresholds(5000, 0.5, 10, 300);
        for _ in 0..20 {
            ks.record_success(50);
        }
        assert!(!ks.is_tripped(), "expected no trip with low latency");
    }

    #[test]
    fn test_error_rate_threshold_trips_switch() {
        let ks = BifrostKillSwitch::with_thresholds(5000, 0.3, 10, 300);
        // Record 7 failures out of 10 = 70% error rate (>30% threshold)
        for _ in 0..3 {
            ks.record_success(50);
        }
        for _ in 0..7 {
            ks.record_failure(50);
        }
        assert!(
            ks.is_tripped(),
            "expected trip on high error rate (70% > 30%)"
        );
    }

    #[test]
    fn test_low_error_rate_does_not_trip() {
        let ks = BifrostKillSwitch::with_thresholds(5000, 0.5, 10, 300);
        // 1 failure out of 10 = 10% (<50% threshold)
        for _ in 0..9 {
            ks.record_success(50);
        }
        ks.record_failure(50);
        assert!(
            !ks.is_tripped(),
            "expected no trip with 10% error rate (<50%)"
        );
    }
}
