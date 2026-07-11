use std::path::Path as FsPath;

use agileplus_domain::domain::event::Event;
use agileplus_events::EventStore;
use agileplus_sqlite::SqliteStorageAdapter;
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json, Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use tokio::sync::broadcast;

use crate::app_state::{CockpitEvent, CockpitSession, DashboardStore, SharedState};
use crate::templates::EventView;

use super::router;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CockpitUpdate {
    pub session_id: String,
    pub run_id: String,
    pub phase: String,
    pub summary: String,
    pub progress: f32,
    #[serde(default)]
    pub ownership_bracket: Option<String>,
    #[serde(default)]
    pub kind: Option<String>,
    #[serde(default)]
    pub agent: Option<String>,
    #[serde(default)]
    pub agents: Vec<String>,
    #[serde(default)]
    pub lanes: Vec<String>,
    #[serde(default)]
    pub notices: Vec<String>,
    #[serde(default)]
    pub trace_refs: Vec<String>,
    #[serde(default)]
    pub payload: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardSnapshot {
    pub sessions: Vec<CockpitSession>,
    pub cockpit_events: Vec<CockpitEvent>,
}

pub type DashboardEventTx = broadcast::Sender<serde_json::Value>;

pub(crate) const COCKPIT_ENTITY_TYPE: &str = "session";
pub(crate) const COCKPIT_EVENT_TYPE: &str = "substrate_cockpit_update";

pub async fn cockpit_post(
    State(state): State<SharedState>,
    event_tx: Option<Extension<DashboardEventTx>>,
    Json(update): Json<CockpitUpdate>,
) -> Response {
    tracing::info!(
        session = %update.session_id,
        run_id = %update.run_id,
        phase = %update.phase,
        progress = update.progress,
        summary = %update.summary,
        "dashboard cockpit update received"
    );
    if let Some(Extension(event_tx)) = event_tx {
        let _ = event_tx.send(serde_json::json!({
            "event_type": "substrate_cockpit_update",
            "data": update,
        }));
    }

    let db_path = {
        let store = state.read().await;
        store.cockpit_event_db_path.clone()
    };
    if let Some(db_path) = db_path {
        if let Err(err) = persist_cockpit_update(&db_path, &update).await {
            tracing::warn!(error = %err, db_path = %db_path.display(), "failed to persist cockpit update");
        }
    }

    let received_at = Utc::now();
    let event = CockpitEvent {
        event_type: COCKPIT_EVENT_TYPE.to_string(),
        session_id: update.session_id.clone(),
        run_id: update.run_id.clone(),
        phase: update.phase.clone(),
        summary: update.summary.clone(),
        progress: update.progress,
        ownership_bracket: update.ownership_bracket.clone(),
        kind: update.kind.clone(),
        agent: update.agent.clone(),
        agents: update.agents.clone(),
        lanes: update.lanes.clone(),
        notices: update.notices.clone(),
        trace_refs: update.trace_refs.clone(),
        payload: update.payload.clone(),
        received_at,
    };
    let mut store = state.write().await;
    store.apply_cockpit_event(event);

    StatusCode::OK.into_response()
}

pub async fn dashboard_snapshot(State(state): State<SharedState>) -> Json<DashboardSnapshot> {
    let store = state.read().await;
    Json(DashboardSnapshot {
        sessions: store.sessions.clone(),
        cockpit_events: store.cockpit_events.clone(),
    })
}

pub fn router_with_events(state: SharedState, event_tx: Option<DashboardEventTx>) -> Router {
    let router = router(state);
    match event_tx {
        Some(event_tx) => router.layer(Extension(event_tx)),
        None => router,
    }
}

pub async fn persist_cockpit_update(
    db_path: &FsPath,
    update: &CockpitUpdate,
) -> anyhow::Result<i64> {
    if let Some(parent) = db_path.parent() {
        if !parent.as_os_str().is_empty() {
            std::fs::create_dir_all(parent)?;
        }
    }
    let store = SqliteStorageAdapter::new(db_path)?;
    let entity_id = stable_session_entity_id(&update.session_id);
    let sequence = store
        .get_latest_sequence(COCKPIT_ENTITY_TYPE, entity_id)
        .await?
        + 1;
    let mut event = Event::new(
        COCKPIT_ENTITY_TYPE,
        entity_id,
        COCKPIT_EVENT_TYPE,
        serde_json::json!({
            "session_id": update.session_id,
            "run_id": update.run_id,
            "phase": update.phase,
            "summary": update.summary,
            "progress": update.progress,
            "ownership_bracket": update.ownership_bracket,
            "kind": update.kind,
            "agent": update.agent,
            "agents": update.agents,
            "lanes": update.lanes,
            "notices": update.notices,
            "trace_refs": update.trace_refs,
            "payload": update.payload,
        }),
        "substrate",
    );
    event.sequence = sequence;
    store.append(&event).await.map_err(Into::into)
}

pub fn stable_session_entity_id(input: &str) -> i64 {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    i64::from_ne_bytes(hash.to_ne_bytes()).wrapping_abs()
}

pub fn hydrate_cockpit_events_from_sqlite(
    store: &mut DashboardStore,
    limit: usize,
) -> anyhow::Result<usize> {
    let Some(db_path) = store.cockpit_event_db_path.clone() else {
        return Ok(0);
    };
    let sqlite = SqliteStorageAdapter::new(&db_path)?;
    let events = sqlite.get_events_by_type(COCKPIT_EVENT_TYPE, limit)?;
    let count = events.len();
    for event in events {
        if let Some(cockpit_event) = cockpit_event_from_domain_event(&event) {
            store.apply_cockpit_event(cockpit_event);
        }
    }
    Ok(count)
}

pub fn cockpit_events_to_event_views(events: &[CockpitEvent], limit: usize) -> Vec<EventView> {
    events
        .iter()
        .rev()
        .take(limit)
        .map(cockpit_event_to_event_view)
        .collect()
}

fn cockpit_event_to_event_view(event: &CockpitEvent) -> EventView {
    let agent_name = event
        .agent
        .clone()
        .or_else(|| event.agents.first().cloned());
    let wp_id = event
        .payload
        .as_ref()
        .and_then(|payload| payload.get("wp_id").or_else(|| payload.get("work_package")))
        .and_then(|value| value.as_str())
        .map(ToString::to_string);

    EventView {
        id: format!(
            "{}:{}:{}",
            event.session_id,
            event.run_id,
            event.received_at.timestamp_millis()
        ),
        kind: event
            .kind
            .clone()
            .unwrap_or_else(|| event.event_type.clone()),
        description: event.summary.clone(),
        timestamp: event.received_at.to_rfc3339(),
        agent_name,
        agent_link: None,
        wp_id,
        wp_link: None,
        commit_sha: None,
        commit_link: None,
        ci_run_id: None,
        ci_run_link: None,
    }
}

fn cockpit_event_from_domain_event(event: &Event) -> Option<CockpitEvent> {
    let payload = &event.payload;
    Some(CockpitEvent {
        event_type: event.event_type.clone(),
        session_id: payload.get("session_id")?.as_str()?.to_string(),
        run_id: payload.get("run_id")?.as_str()?.to_string(),
        phase: payload.get("phase")?.as_str()?.to_string(),
        summary: payload.get("summary")?.as_str()?.to_string(),
        progress: payload.get("progress")?.as_f64()? as f32,
        ownership_bracket: payload
            .get("ownership_bracket")
            .and_then(|value| value.as_str())
            .map(ToString::to_string),
        kind: payload
            .get("kind")
            .and_then(|value| value.as_str())
            .map(ToString::to_string),
        agent: payload
            .get("agent")
            .and_then(|value| value.as_str())
            .map(ToString::to_string),
        agents: string_array_payload(payload, "agents"),
        lanes: string_array_payload(payload, "lanes"),
        notices: string_array_payload(payload, "notices"),
        trace_refs: string_array_payload(payload, "trace_refs"),
        payload: payload
            .get("payload")
            .cloned()
            .filter(|value| !value.is_null()),
        received_at: event.timestamp,
    })
}

fn string_array_payload(payload: &serde_json::Value, key: &str) -> Vec<String> {
    payload
        .get(key)
        .and_then(|value| value.as_array())
        .map(|values| {
            values
                .iter()
                .filter_map(|value| value.as_str().map(ToString::to_string))
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use tempfile::tempdir;
    use tokio::sync::RwLock;

    fn update() -> CockpitUpdate {
        CockpitUpdate {
            session_id: "dogfood-flow".into(),
            run_id: "run-1".into(),
            phase: "running".into(),
            summary: "WP01 running".into(),
            progress: 0.25,
            ownership_bracket: Some("[OmniRoute:✓, AgilePlus:◐]".into()),
            kind: Some("agent".into()),
            agent: Some("codex".into()),
            agents: vec!["codex".into()],
            lanes: vec!["implement".into()],
            notices: vec![],
            trace_refs: vec![],
            payload: None,
        }
    }

    #[tokio::test]
    async fn post_preserves_ownership_bracket_in_snapshot() {
        let state = Arc::new(RwLock::new(DashboardStore::default()));
        let response = cockpit_post(State(state.clone()), None, Json(update())).await;
        assert_eq!(response.status(), StatusCode::OK);

        let snapshot = dashboard_snapshot(State(state)).await.0;
        assert_eq!(snapshot.sessions.len(), 1);
        assert_eq!(
            snapshot.sessions[0].ownership_bracket.as_deref(),
            Some("[OmniRoute:✓, AgilePlus:◐]")
        );
        assert_eq!(
            snapshot.cockpit_events[0].ownership_bracket,
            snapshot.sessions[0].ownership_bracket
        );
    }

    #[tokio::test]
    async fn sqlite_roundtrip_preserves_ownership_bracket() {
        let dir = tempdir().expect("temp dir");
        let db_path = dir.path().join("agileplus.db");
        persist_cockpit_update(&db_path, &update())
            .await
            .expect("persist cockpit event");

        let mut store = DashboardStore {
            cockpit_event_db_path: Some(db_path),
            ..DashboardStore::default()
        };
        assert_eq!(
            hydrate_cockpit_events_from_sqlite(&mut store, 10).unwrap(),
            1
        );
        assert_eq!(
            store.sessions[0].ownership_bracket.as_deref(),
            Some("[OmniRoute:✓, AgilePlus:◐]")
        );
    }
}
