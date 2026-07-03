//! Phenotype fleet observability substrate.
//!
//! Self-contained tracing helpers for local spans and cockpit emission.

use std::collections::HashMap;
use std::sync::Arc;

pub use tracing::{debug, error, info, instrument, span, trace, warn};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraceId(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpanId(pub String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpanKind {
    Internal,
    Client,
    Server,
    Producer,
    Consumer,
}

#[derive(Debug, Clone)]
pub struct TraceOperation {
    pub trace_id: TraceId,
    pub span_id: SpanId,
    pub parent_span_id: Option<SpanId>,
    pub kind: SpanKind,
    pub name: String,
    pub attributes: HashMap<String, String>,
}

#[async_trait::async_trait]
pub trait TracePort: Send + Sync {
    async fn submit(&self, op: TraceOperation);
    async fn flush(&self) -> Result<(), String>;
}

#[derive(Debug, Default, Clone, Copy)]
pub struct StdoutAdapter;

#[async_trait::async_trait]
impl TracePort for StdoutAdapter {
    async fn submit(&self, op: TraceOperation) {
        println!(
            "[TRACE] trace={} span={} kind={:?}",
            op.trace_id.0, op.name, op.kind
        );
    }

    async fn flush(&self) -> Result<(), String> {
        Ok(())
    }
}

pub const SERVICE_NAME: &str = "phenotype";
pub const DEFAULT_OTLP_ENDPOINT: &str = "http://localhost:4317";

pub fn otlp_endpoint() -> String {
    std::env::var("OTEL_EXPORTER_OTLP_ENDPOINT")
        .unwrap_or_else(|_| DEFAULT_OTLP_ENDPOINT.to_string())
}

pub fn build_span(
    trace_id: &str,
    span_id: &str,
    parent_span_id: Option<&str>,
    name: &str,
    kind: SpanKind,
    attributes: HashMap<String, String>,
) -> TraceOperation {
    TraceOperation {
        trace_id: TraceId(trace_id.to_string()),
        span_id: SpanId(span_id.to_string()),
        parent_span_id: parent_span_id.map(|s| SpanId(s.to_string())),
        kind,
        name: name.to_string(),
        attributes,
    }
}

pub fn next_trace_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    let n = COUNTER.fetch_add(1, Ordering::SeqCst);
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    format!("phenotype-trace-{}-{}", ts, n)
}

pub async fn submit_span(op: TraceOperation) {
    let port: Arc<dyn TracePort> = Arc::new(StdoutAdapter);
    port.submit(op).await;
    let _ = port.flush().await;
}

pub async fn emit_span(name: &str, attributes: HashMap<String, String>) {
    let trace_id = next_trace_id();
    let op = build_span(
        &trace_id,
        &format!("{}-{}", name, trace_id),
        None,
        name,
        SpanKind::Internal,
        attributes,
    );
    submit_span(op).await;
}
