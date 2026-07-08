//! B8 — Bifrost MCP client.
//!
//! Connects [`RouterPort`] to the MCP server tool surface
//! (`open-sse/mcp-server/`). Exposes `route_pick` and `route_record`
//! as typed MCP tool definitions that the downstream TS MCP server can
//! call back into the Rust runtime.
//!
//! ## Design
//!
//! The TS MCP server (`open-sse/mcp-server/server.ts`) owns the
//! transport layer (stdio / SSE / Streamable HTTP). When a tool
//! invocation arrives that references a Bifrost-backed route, the
//! MCP server dispatches to this module's methods via a JSON-RPC
//! bridge (the `RouteTool` struct and its `execute` method).
//!
//! This module is the **Rust side** of that bridge — it does NOT own
//! the MCP transport. It provides:
//!
//! 1. [`RouteTool`] — a callable that wraps [`RouterPort`] and
//!    exposes a JSON-serializable interface the TS side can call.
//! 2. [`McpRouterClient`] — a convenience struct the TS MCP server
//!    uses to route invocation requests through the Bifrost backend.
//!
//! ## Status (B8, Q4 2026 target)
//!
//! This is a **scaffold**. The actual TS→Rust bridge needs the
//! `pheno-cdylib-bridge` crate or a sidecar process. Until that's
//! wired, the methods here are callable from Rust tests only.
//!
//! ## Refs
//!
//! - `PLAN.md` § 2.5.2 (B8)
//! - `docs/adr/0031-bifrost-tier1-router.md`
//! - `open-sse/mcp-server/server.ts` (TS MCP server)
//! - `pheno-cdylib-bridge` (bridge crate, when available)

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::router::{RouterPort, RouteTarget, RouteRequest, RouteOutcome};

/// A single MCP-compatible route-pick call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteTool {
    /// The requested model id (e.g. "gpt-4o-mini").
    pub requested_model: String,
    /// Optional tenant / API-key scoping.
    pub tenant: Option<String>,
    /// Optional kind hint (`chat`, `embedding`, ...).
    pub kind: Option<String>,
}

/// Outcome returned by a tool execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteToolOutcome {
    /// The selected route target (model_id + metadata), if any.
    pub target: Option<RouteTarget>,
    /// Error message, if execution failed.
    pub error: Option<String>,
}

impl RouteTool {
    /// Execute a single route-pick against the given router.
    ///
    /// Returns the selected target, or an error description.
    /// Errors are not fatal — they are returned as structured
    /// [`RouteToolOutcome`] so the MCP caller can decide next steps.
    pub async fn execute<R: RouterPort>(&self, router: &R) -> RouteToolOutcome {
        let req = RouteRequest {
            requested_model: self.requested_model.clone(),
            tenant: self.tenant.clone(),
            kind: self.kind.clone(),
        };
        match router.pick(&req).await {
            Ok(target) => RouteToolOutcome {
                target: Some(target),
                error: None,
            },
            Err(e) => RouteToolOutcome {
                target: None,
                error: Some(e.to_string()),
            },
        }
    }
}

/// Convenience client the MCP server uses to communicate with a
/// [`RouterPort`] implementation.
///
/// Holds a reference-counted router and provides typed methods
/// that the downstream TS side serializes to/from JSON-RPC.
pub struct McpRouterClient<R: RouterPort> {
    router: R,
}

impl<R: RouterPort> McpRouterClient<R> {
    pub fn new(router: R) -> Self {
        Self { router }
    }

    /// Route-pick followed by a no-op record_outcome.
    /// Used when the MCP caller only needs to know the target
    /// without providing outcome feedback.
    pub async fn probe(&self, tool: &RouteTool) -> RouteToolOutcome {
        tool.execute(&self.router).await
    }

    /// Full pick + record cycle.
    pub async fn route_and_record(
        &self,
        tool: &RouteTool,
        outcome: &RouteOutcome,
    ) -> RouteToolOutcome {
        let pick_result = self.router.pick(&{
            RouteRequest {
                requested_model: tool.requested_model.clone(),
                tenant: tool.tenant.clone(),
                kind: tool.kind.clone(),
            }
        }).await;

        match pick_result {
            Ok(target) => {
                // Record the outcome (best-effort; errors are logged but
                // not returned since the pick already succeeded).
                let _ = self.router.record_outcome(&target, outcome).await;
                RouteToolOutcome {
                    target: Some(target),
                    error: None,
                }
            }
            Err(e) => RouteToolOutcome {
                target: None,
                error: Some(e.to_string()),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::router::InMemoryRouter;

    #[tokio::test]
    async fn route_tool_execute_picks_correct_target() {
        let mut router = InMemoryRouter::new();
        router.add_fallback("gpt-4o", [RouteTarget::new("gpt-4o")]);

        let tool = RouteTool {
            requested_model: "gpt-4o".into(),
            tenant: None,
            kind: None,
        };

        let outcome = tool.execute(&router).await;
        assert!(outcome.error.is_none());
        assert_eq!(outcome.target.as_ref().map(|t| &t.model_id), Some(&"gpt-4o".to_string()));
    }

    #[tokio::test]
    async fn route_tool_execute_returns_error_for_no_match() {
        let router = InMemoryRouter::new();

        let tool = RouteTool {
            requested_model: "unknown-model".into(),
            tenant: None,
            kind: None,
        };

        let outcome = tool.execute(&router).await;
        assert!(outcome.target.is_none());
        assert!(outcome.error.is_some());
        assert!(outcome.error.unwrap().contains("no route"));
    }

    #[tokio::test]
    async fn mcp_client_probe_roundtrips() {
        let mut router = InMemoryRouter::new();
        router.add_fallback("claude-sonnet", [RouteTarget::new("claude-sonnet-4-5")]);

        let client = McpRouterClient::new(router);
        let tool = RouteTool {
            requested_model: "claude-sonnet".into(),
            tenant: Some("acme-corp".into()),
            kind: None,
        };

        let outcome = client.probe(&tool).await;
        assert_eq!(
            outcome.target.as_ref().map(|t| &t.model_id),
            Some(&"claude-sonnet-4-5".to_string())
        );
    }

    #[tokio::test]
    async fn mcp_client_route_and_record_full_cycle() {
        let mut router = InMemoryRouter::new();
        router.add_fallback("gpt-4o", [RouteTarget::new("gpt-4o-mini")]);

        let client = McpRouterClient::new(router);
        let tool = RouteTool {
            requested_model: "gpt-4o".into(),
            tenant: None,
            kind: None,
        };
        let outcome_record = RouteOutcome {
            success: true,
            latency_ms: Some(200),
            error: None,
        };

        let outcome = client.route_and_record(&tool, &outcome_record).await;
        assert!(outcome.error.is_none());
        assert_eq!(
            outcome.target.as_ref().map(|t| &t.model_id),
            Some(&"gpt-4o-mini".to_string())
        );
    }
}
