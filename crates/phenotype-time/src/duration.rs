//! Duration utilities for working with std::time::Duration.

use chrono::Duration as ChronoDuration;
use std::time::Duration;

/// Extension trait for std::time::Duration.
pub trait DurationExt {
    /// Format as human-readable string (e.g. "1h 30m").
    fn format_human(&self) -> String;

    /// Format as compact string (e.g. "1h30m").
    fn format_compact(&self) -> String;

    /// Convert to chrono::Duration.
    fn to_chrono(&self) -> Option<ChronoDuration>;

    /// Convert from chrono::Duration.
    fn from_chrono(duration: ChronoDuration) -> Option<Duration>;

    /// Check if duration is zero.
    fn is_zero(&self) -> bool;

    /// Check if duration is positive.
    fn is_positive(&self) -> bool;

    /// Check if duration is negative.
    fn is_negative(&self) -> bool;

    /// Clamp duration between min and max.
    fn clamp(&self, min: Duration, max: Duration) -> Duration;
}

impl DurationExt for Duration {
    fn format_human(&self) -> String {
        let secs = self.as_secs();
        let hours = secs / 3600;
        let mins = (secs % 3600) / 60;
        let secs = secs % 60;

        if hours > 0 {
            format!("{}h {}m {}s", hours, mins, secs)
        } else if mins > 0 {
            format!("{}m {}s", mins, secs)
        } else {
            format!("{}s", secs)
        }
    }

    fn format_compact(&self) -> String {
        let secs = self.as_secs();
        let hours = secs / 3600;
        let mins = (secs % 3600) / 60;
        let secs = secs % 60;

        if hours > 0 {
            format!("{}h{}m{}s", hours, mins, secs)
        } else if mins > 0 {
            format!("{}m{}s", mins, secs)
        } else {
            format!("{}s", secs)
        }
    }

    fn to_chrono(&self) -> Option<ChronoDuration> {
        ChronoDuration::from_std(*self).ok()
    }

    fn from_chrono(duration: ChronoDuration) -> Option<Duration> {
        duration.to_std().ok()
    }

    fn is_zero(&self) -> bool {
        self.as_secs() == 0 && self.subsec_nanos() == 0
    }

    fn is_positive(&self) -> bool {
        !self.is_zero()
    }

    fn is_negative(&self) -> bool {
        false
    }

    fn clamp(&self, min: Duration, max: Duration) -> Duration {
        if *self < min {
            min
        } else if *self > max {
            max
        } else {
            *self
        }
    }
}

/// Duration-related constants.
pub mod constants {
    use std::time::Duration;

    // Cache TTLs
    /// 5 minutes - short-lived cache entries.
    pub const CACHE_TTL_SHORT: Duration = Duration::from_secs(300);

    /// 15 minutes - long-lived cache entries.
    pub const CACHE_TTL_LONG: Duration = Duration::from_secs(900);

    /// 1 hour - session cache entries.
    pub const CACHE_TTL_SESSION: Duration = Duration::from_secs(3600);

    // Timeouts
    /// 5 seconds - fast operations.
    pub const TIMEOUT_FAST: Duration = Duration::from_secs(5);

    /// 30 seconds - normal operations.
    pub const TIMEOUT_NORMAL: Duration = Duration::from_secs(30);

    /// 60 seconds - slow operations.
    pub const TIMEOUT_SLOW: Duration = Duration::from_secs(60);

    /// 5 minutes - batch operations.
    pub const TIMEOUT_BATCH: Duration = Duration::from_secs(300);

    // Retries
    /// 100 milliseconds - fast retry.
    pub const RETRY_FAST: Duration = Duration::from_millis(100);

    /// 500 milliseconds - normal retry.
    pub const RETRY_NORMAL: Duration = Duration::from_millis(500);

    /// 1 second - slow retry.
    pub const RETRY_SLOW: Duration = Duration::from_secs(1);

    /// Exponential backoff base (2 seconds).
    pub const BACKOFF_BASE: Duration = Duration::from_secs(2);

    /// Exponential backoff max (60 seconds).
    pub const BACKOFF_MAX: Duration = Duration::from_secs(60);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_compact_hours_minutes_seconds() {
        let duration = Duration::from_secs(3661);
        assert_eq!(duration.format_compact(), "1h1m1s");
    }

    #[test]
    fn format_compact_minutes_seconds() {
        let duration = Duration::from_secs(125);
        assert_eq!(duration.format_compact(), "2m5s");
    }

    #[test]
    fn format_compact_seconds_only() {
        let duration = Duration::from_secs(45);
        assert_eq!(duration.format_compact(), "45s");
    }

    #[test]
    fn format_compact_zero() {
        let duration = Duration::ZERO;
        assert_eq!(duration.format_compact(), "0s");
    }
}
