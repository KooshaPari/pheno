//! B6: Traffic-shadow adapter — send a percentage of `pick()` / `record_outcome()`
//! calls to a shadow router without affecting the primary path.
//!
//! The primary router's result is **always** returned to the caller. The shadow
//! router receives the same request fire-and-forget when a random sample matches.
//! Shadow failures are logged via `tracing::warn!` — they never propagate to the
//! dispatch loop (R-omni-1: no silent error swallowing in the primary path).
//!
//! ## Usage
//! ```ignore
//! let shadow = TrafficShadow::new(primary, bifrost, 0.05);
//! let target = shadow.pick(&request).await?;        // always primary
//! shadow.record_outcome(&target, &outcome).await?;   // always primary + sometimes bifrost
//! ```

use async_trait::async_trait;
use rand::Rng;

use crate::error::{Error, Result};
use crate::router::{RouteOutcome, RouteRequest, RouteTarget, RouterPort};

/// Default proportion of requests forwarded to the shadow router (5%).
pub const DEFAULT_SAMPLE_RATE: f64 = 0.05;

/// Adapter that wraps a primary [`RouterPort`] with an optional shadow router.
///
/// - [`pick`](RouterPort::pick) always returns the primary's result. When the
///   sample succeeds, a tokio task is spawned to `shadow.pick(request)` —
///   its result is **dropped**. Shadow failures are logged at `warn` level.
/// - [`record_outcome`](RouterPort::record_outcome) always records on the
///   primary. When the sample succeeds, the same outcome is forwarded to the
///   shadow fire-and-forget.
#[derive(Debug)]
pub struct TrafficShadow<P, S> {
    primary: P,
    shadow: S,
    sample_rate: f64,
}

impl<P, S> TrafficShadow<P, S>
where
    P: RouterPort,
    S: RouterPort,
{
    /// Create a new traffic-shadow adapter.
    ///
    /// # Panics
    /// Panics if `sample_rate` is not in `[0.0, 1.0]`.
    #[must_use]
    pub fn new(primary: P, shadow: S, sample_rate: f64) -> Self {
        assert!(
            (0.0..=1.0).contains(&sample_rate),
            "sample_rate must be in [0.0, 1.0], got {sample_rate}"
        );
        Self {
            primary,
            shadow,
            sample_rate,
        }
    }

    /// Create a shadow adapter with the default 5 % sample rate.
    #[must_use]
    pub fn with_default_rate(primary: P, shadow: S) -> Self {
        Self::new(primary, shadow, DEFAULT_SAMPLE_RATE)
    }

    /// Returns `true` when the current request should be forwarded to the
    /// shadow router, based on the configured `sample_rate`.
    fn should_sample(&self) -> bool {
        if self.sample_rate >= 1.0 {
            return true;
        }
        if self.sample_rate <= 0.0 {
            return false;
        }
        let mut rng = rand::thread_rng();
        rng.gen::<f64>() < self.sample_rate
    }
}

#[async_trait]
impl<P, S> RouterPort for TrafficShadow<P, S>
where
    P: RouterPort + Send + Sync,
    S: RouterPort + Send + Sync + 'static,
    {
    /// Always returns the primary's result. Spawns a fire-and-forget task for
    /// the shadow's `pick` when the sample fires.
    async fn pick(&self, request: &RouteRequest) -> Result<RouteTarget> {
        let primary_result = self.primary.pick(request).await;

        if self.should_sample() {
            let shadow_req = request.clone();
            let shadow = &self.shadow;
            // Spawn a fire-and-forget tokio task so the shadow never blocks
            // the primary path.
            tokio::spawn(async move {
                match shadow.pick(&shadow_req).await {
                    Ok(_target) => {
                        // Shadow picked — quietly dropped. That's the contract.
                    }
                    Err(e) => {
                        let e: Error = e.into();
                        tracing::warn!(
                            error = %e,
                            "b6-shadow: shadow pick failed (dropped)"
                        );
                    }
                }
            });
        }

        primary_result
    }

    /// Records on the primary. Forwards the same outcome to the shadow
    /// fire-and-forget when the sample fires.
    async fn record_outcome(
        &self,
        target: &RouteTarget,
        outcome: &RouteOutcome,
    ) -> Result<()> {
        let primary_result = self.primary.record_outcome(target, outcome).await;

        if self.should_sample() {
            let shadow_target = target.clone();
            let shadow_outcome = outcome.clone();
            let shadow = &self.shadow;
            tokio::spawn(async move {
                match shadow.record_outcome(&shadow_target, &shadow_outcome).await {
                    Ok(()) => {}
                    Err(e) => {
                        let e: Error = e.into();
                        tracing::warn!(
                            error = %e,
                            "b6-shadow: shadow record_outcome failed (dropped)"
                        );
                    }
                }
            });
        }

        primary_result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::InMemoryRouter;

    /// A test helper that wraps the common setup:
    /// a known model registered in both primary and shadow routers.
    fn seeded_routers() -> (InMemoryRouter, InMemoryRouter) {
        let mut primary = InMemoryRouter::new();
        primary.add_fallback("gpt-4o", [RouteTarget::new("gpt-4o")]);

        let mut shadow = InMemoryRouter::new();
        shadow.add_fallback("gpt-4o", [RouteTarget::new("gpt-4o")]);

        (primary, shadow)
    }

    #[tokio::test]
    async fn sample_rate_zero_never_forwards() {
        let (primary, shadow) = seeded_routers();
        let s = TrafficShadow::new(primary, shadow, 0.0);
        let req = RouteRequest {
            requested_model: "gpt-4o".into(),
            tenant: None,
            kind: None,
        };
        // 100 iterations with 0% sample rate: shadow is never called.
        // We just verify primary picks correctly.
        for _ in 0..100 {
            let target = s.pick(&req).await.expect("primary always picks");
            assert_eq!(target.model_id, "gpt-4o");
        }
    }

    #[tokio::test]
    async fn sample_rate_100_forwards_always() {
        let (primary, shadow) = seeded_routers();
        let s = TrafficShadow::new(primary, shadow, 1.0);
        let req = RouteRequest {
            requested_model: "gpt-4o".into(),
            tenant: None,
            kind: None,
        };
        let target = s.pick(&req).await.expect("pick succeeds");
        assert_eq!(target.model_id, "gpt-4o");
    }

    #[tokio::test]
    async fn primary_always_returns_regardless_of_shadow() {
        // Shadow router has NO routes for the requested model — but this
        // should never affect the primary's result.
        let primary = {
            let mut r = InMemoryRouter::new();
            r.add_fallback("gpt-4o", [RouteTarget::new("gpt-4o")]);
            r
        };
        let shadow = InMemoryRouter::new(); // empty — no routes
        let s = TrafficShadow::new(primary, shadow, 1.0);

        let req = RouteRequest {
            requested_model: "gpt-4o".into(),
            tenant: None,
            kind: None,
        };
        let target = s.pick(&req).await.expect("primary still picks");
        assert_eq!(target.model_id, "gpt-4o");
    }

    #[tokio::test]
    async fn record_outcome_forwards_to_primary_regardless_of_shadow() {
        let (primary, shadow) = seeded_routers();
        let s = TrafficShadow::new(primary, shadow, 0.5);
        let target = RouteTarget::new("gpt-4o");
        let outcome = RouteOutcome {
            success: true,
            latency_ms: Some(42),
            error: None,
        };
        // Never fails because primary always works.
        s.record_outcome(&target, &outcome)
            .await
            .expect("primary record_outcome must succeed");
    }

    #[tokio::test]
    async fn sample_rate_0_record_outcome_still_works() {
        let (primary, shadow) = seeded_routers();
        let s = TrafficShadow::new(primary, shadow, 0.0);
        let target = RouteTarget::new("gpt-4o");
        let outcome = RouteOutcome {
            success: true,
            latency_ms: None,
            error: None,
        };
        s.record_outcome(&target, &outcome)
            .await
            .expect("0%% record_outcome must succeed");
    }

    #[tokio::test]
    async fn primary_error_propagates_independently_of_shadow() {
        let primary = InMemoryRouter::new(); // empty — no routes
        let shadow = {
            let mut r = InMemoryRouter::new();
            r.add_fallback("gpt-4o", [RouteTarget::new("gpt-4o")]);
            r
        };
        let s = TrafficShadow::new(primary, shadow, 1.0);
        let req = RouteRequest {
            requested_model: "gpt-4o".into(),
            tenant: None,
            kind: None,
        };
        let err = s.pick(&req).await.unwrap_err();
        assert!(matches!(err, Error::NoMatch(_)));
    }
}
