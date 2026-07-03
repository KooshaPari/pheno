//! Runtime AgentPort adapter selection for the AgilePlus CLI.
//!
//! AgilePlus owns planning, state, and governance. Substrate owns provider
//! dispatch and low-token cockpit events.

use std::collections::HashMap;
use std::path::Path;
use std::process::Command;
use std::sync::{Arc, Mutex};

use agileplus_domain::error::DomainError;
use agileplus_domain::ports::agent::{
    AgentConfig, AgentKind, AgentPort, AgentResult, AgentStatus, AgentTask,
};
use chrono::Utc;
use serde::Serialize;

use crate::agent_stub::StubAgentAdapter;

pub enum ConfiguredAgentAdapter {
    Substrate(SubstrateAgentAdapter),
    Stub(StubAgentAdapter),
}

impl ConfiguredAgentAdapter {
    pub fn from_env() -> Self {
        match std::env::var("AGILEPLUS_AGENT_BACKEND") {
            Ok(value) if value.eq_ignore_ascii_case("stub") => Self::Stub(StubAgentAdapter),
            _ => Self::Substrate(SubstrateAgentAdapter::from_env()),
        }
    }
}

impl AgentPort for ConfiguredAgentAdapter {
    async fn dispatch(
        &self,
        task: AgentTask,
        config: &AgentConfig,
    ) -> Result<AgentResult, DomainError> {
        match self {
            Self::Substrate(adapter) => adapter.dispatch(task, config).await,
            Self::Stub(adapter) => adapter.dispatch(task, config).await,
        }
    }

    async fn dispatch_async(
        &self,
        task: AgentTask,
        config: &AgentConfig,
    ) -> Result<String, DomainError> {
        match self {
            Self::Substrate(adapter) => adapter.dispatch_async(task, config).await,
            Self::Stub(adapter) => adapter.dispatch_async(task, config).await,
        }
    }

    async fn query_status(&self, job_id: &str) -> Result<AgentStatus, DomainError> {
        match self {
            Self::Substrate(adapter) => adapter.query_status(job_id).await,
            Self::Stub(adapter) => adapter.query_status(job_id).await,
        }
    }

    async fn cancel(&self, job_id: &str) -> Result<(), DomainError> {
        match self {
            Self::Substrate(adapter) => adapter.cancel(job_id).await,
            Self::Stub(adapter) => adapter.cancel(job_id).await,
        }
    }

    async fn send_instruction(&self, job_id: &str, instruction: &str) -> Result<(), DomainError> {
        match self {
            Self::Substrate(adapter) => adapter.send_instruction(job_id, instruction).await,
            Self::Stub(adapter) => adapter.send_instruction(job_id, instruction).await,
        }
    }
}

pub struct SubstrateAgentAdapter {
    bin: String,
    cockpit_url: Option<String>,
    running: Arc<Mutex<HashMap<String, RunningJob>>>,
    completed: Arc<Mutex<HashMap<String, AgentResult>>>,
}

#[derive(Debug, Clone)]
struct RunningJob {
    pid: u32,
    task: AgentTask,
}

impl SubstrateAgentAdapter {
    pub fn from_env() -> Self {
        Self {
            bin: std::env::var("SUBSTRATE_BIN").unwrap_or_else(|_| "substrate".to_string()),
            cockpit_url: std::env::var("AGILEPLUS_COCKPIT_URL").ok(),
            running: Arc::new(Mutex::new(HashMap::new())),
            completed: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn run_id(task: &AgentTask) -> String {
        format!(
            "agileplus-{}-{}-{}",
            task.feature_slug,
            task.wp_id,
            Utc::now().timestamp_millis()
        )
    }

    fn build_prompt(task: &AgentTask) -> Result<String, DomainError> {
        let mut prompt = String::new();
        prompt.push_str("You are an AgilePlus implementation agent.\n");
        prompt.push_str("Implement the assigned work package in this worktree.\n");
        prompt.push_str(
            "Emit concise status in chat; detailed cockpit state belongs in Substrate events.\n\n",
        );
        prompt.push_str(&format!("Feature: {}\n", task.feature_slug));
        prompt.push_str(&format!("Work package: {}\n\n", task.wp_id));

        append_file(&mut prompt, "Prompt", &task.prompt_path)?;
        for path in &task.context_files {
            append_file(&mut prompt, "Context", path)?;
        }

        Ok(prompt)
    }

    fn engine(kind: AgentKind) -> &'static str {
        match kind {
            AgentKind::ClaudeCode => "claude",
            AgentKind::Codex => "codex",
        }
    }
}

impl AgentPort for SubstrateAgentAdapter {
    async fn dispatch(
        &self,
        task: AgentTask,
        config: &AgentConfig,
    ) -> Result<AgentResult, DomainError> {
        let job_id = Self::run_id(&task);
        emit_cockpit_update(
            self.cockpit_url.as_deref(),
            cockpit_update_for(&job_id, &task, "running", 0.0, "agent dispatched"),
        )
        .await;
        let prompt = Self::build_prompt(&task)?;
        let output = build_substrate_dispatch(&self.bin, &job_id, &task, config, &prompt, false)?
            .output()
            .map_err(|err| DomainError::Other(format!("executing substrate dispatch: {err}")))?;
        let result = AgentResult {
            success: output.status.success(),
            pr_url: None,
            commits: vec![],
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            exit_code: output.status.code().unwrap_or(-1),
        };
        emit_cockpit_update(
            self.cockpit_url.as_deref(),
            cockpit_update_for(
                &job_id,
                &task,
                if result.success {
                    "completed"
                } else {
                    "failed"
                },
                1.0,
                if result.success {
                    "agent completed"
                } else {
                    "agent failed"
                },
            ),
        )
        .await;
        Ok(result)
    }

    async fn dispatch_async(
        &self,
        task: AgentTask,
        config: &AgentConfig,
    ) -> Result<String, DomainError> {
        let job_id = Self::run_id(&task);
        emit_cockpit_update(
            self.cockpit_url.as_deref(),
            cockpit_update_for(&job_id, &task, "running", 0.0, "agent dispatched"),
        )
        .await;
        let prompt = Self::build_prompt(&task)?;
        let child = build_substrate_dispatch(&self.bin, &job_id, &task, config, &prompt, true)?
            .spawn()
            .map_err(|err| DomainError::Other(format!("spawning substrate dispatch: {err}")))?;
        let pid = child.id();
        self.running
            .lock()
            .map_err(|_| DomainError::Other("substrate running cache poisoned".to_string()))?
            .insert(
                job_id.clone(),
                RunningJob {
                    pid,
                    task: task.clone(),
                },
            );

        let running = Arc::clone(&self.running);
        let completed = Arc::clone(&self.completed);
        let cockpit_url = self.cockpit_url.clone();
        let task_for_update = task.clone();
        let waiter_job_id = job_id.clone();
        std::thread::spawn(move || {
            let result = match child.wait_with_output() {
                Ok(output) => AgentResult {
                    success: output.status.success(),
                    pr_url: None,
                    commits: vec![],
                    stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
                    stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
                    exit_code: output.status.code().unwrap_or(-1),
                },
                Err(err) => AgentResult {
                    success: false,
                    pr_url: None,
                    commits: vec![],
                    stdout: String::new(),
                    stderr: format!("waiting for substrate dispatch: {err}"),
                    exit_code: -1,
                },
            };
            emit_cockpit_update_blocking(
                cockpit_url.as_deref(),
                cockpit_update_for(
                    &waiter_job_id,
                    &task_for_update,
                    if result.success {
                        "completed"
                    } else {
                        "failed"
                    },
                    1.0,
                    if result.success {
                        "agent completed"
                    } else {
                        "agent failed"
                    },
                ),
            );

            if let Ok(mut running) = running.lock() {
                running.remove(&waiter_job_id);
            }
            if let Ok(mut completed) = completed.lock() {
                completed.insert(waiter_job_id, result);
            }
        });

        Ok(job_id)
    }

    async fn query_status(&self, job_id: &str) -> Result<AgentStatus, DomainError> {
        let running_pid = self
            .running
            .lock()
            .map_err(|_| DomainError::Other("substrate running cache poisoned".to_string()))?
            .get(job_id)
            .cloned();
        if let Some(job) = running_pid {
            return Ok(AgentStatus::Running { pid: job.pid });
        }

        let result = self
            .completed
            .lock()
            .map_err(|_| DomainError::Other("substrate result cache poisoned".to_string()))?
            .get(job_id)
            .cloned();
        Ok(match result {
            Some(result) if result.success => AgentStatus::Completed { result },
            Some(result) => AgentStatus::Failed {
                error: result.stderr,
            },
            None => AgentStatus::Pending,
        })
    }

    async fn cancel(&self, job_id: &str) -> Result<(), DomainError> {
        let running_job = self
            .running
            .lock()
            .map_err(|_| DomainError::Other("substrate running cache poisoned".to_string()))?
            .remove(job_id);
        let Some(job) = running_job else {
            return Ok(());
        };

        terminate_process(job.pid)?;
        let result = AgentResult {
            success: false,
            pr_url: None,
            commits: vec![],
            stdout: String::new(),
            stderr: "canceled by AgilePlus".to_string(),
            exit_code: -1,
        };
        self.completed
            .lock()
            .map_err(|_| DomainError::Other("substrate result cache poisoned".to_string()))?
            .insert(job_id.to_string(), result);
        emit_cockpit_update(
            self.cockpit_url.as_deref(),
            cockpit_update_for(job_id, &job.task, "canceled", 1.0, "agent canceled"),
        )
        .await;
        Ok(())
    }

    async fn send_instruction(&self, _job_id: &str, instruction: &str) -> Result<(), DomainError> {
        eprintln!("Substrate instruction channel is not persistent yet:\n{instruction}");
        Ok(())
    }
}

#[derive(Debug, Serialize)]
struct CockpitUpdate {
    session_id: String,
    run_id: String,
    phase: String,
    summary: String,
    progress: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    ownership_bracket: Option<String>,
}

fn cockpit_update_for(
    job_id: &str,
    task: &AgentTask,
    phase: &str,
    progress: f32,
    summary: &str,
) -> CockpitUpdate {
    CockpitUpdate {
        session_id: task.feature_slug.clone(),
        run_id: job_id.to_string(),
        phase: phase.to_string(),
        summary: format!("{} {}: {summary}", task.feature_slug, task.wp_id),
        progress,
        ownership_bracket: ownership_bracket_from_env(),
    }
}

fn ownership_bracket_from_env() -> Option<String> {
    std::env::var("AGILEPLUS_OWNERSHIP_BRACKET")
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

async fn emit_cockpit_update(url: Option<&str>, update: CockpitUpdate) {
    let Some(url) = url else {
        return;
    };
    let result = reqwest::Client::new().post(url).json(&update).send().await;
    if let Err(err) = result {
        tracing::warn!(error = %err, "failed to emit AgilePlus cockpit update");
    }
}

fn emit_cockpit_update_blocking(url: Option<&str>, update: CockpitUpdate) {
    let Some(url) = url else {
        return;
    };
    let result = reqwest::blocking::Client::new()
        .post(url)
        .json(&update)
        .send();
    if let Err(err) = result {
        tracing::warn!(error = %err, "failed to emit AgilePlus cockpit update");
    }
}

fn append_file(prompt: &mut String, label: &str, path: &Path) -> Result<(), DomainError> {
    match std::fs::read_to_string(path) {
        Ok(content) => {
            prompt.push_str(&format!("## {label}: {}\n", path.display()));
            prompt.push_str(&content);
            prompt.push_str("\n\n");
        }
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {}
        Err(err) => {
            return Err(DomainError::Other(format!(
                "reading {}: {err}",
                path.display()
            )));
        }
    }
    Ok(())
}

fn build_substrate_dispatch(
    bin: &str,
    _job_id: &str,
    task: &AgentTask,
    config: &AgentConfig,
    prompt: &str,
    background: bool,
) -> Result<Command, DomainError> {
    let mut command = Command::new(bin);
    command
        .arg("dispatch")
        .arg("--engine")
        .arg(SubstrateAgentAdapter::engine(config.kind))
        .arg("--cwd")
        .arg(&task.worktree_path)
        .arg("--mode")
        .arg(if background {
            "background"
        } else {
            "foreground"
        })
        .arg("--agent")
        .arg("subagent");

    if std::env::var("AGILEPLUS_SUBSTRATE_DRY_RUN").is_ok() {
        command.arg("--dry-run");
    }
    if !config.extra_args.is_empty() {
        command.arg("--").args(&config.extra_args);
    }
    command.arg(prompt);

    Ok(command)
}

fn terminate_process(pid: u32) -> Result<(), DomainError> {
    let status = Command::new("kill")
        .arg("-TERM")
        .arg(pid.to_string())
        .status()
        .map_err(|err| DomainError::Other(format!("terminating substrate pid {pid}: {err}")))?;
    if status.success() {
        Ok(())
    } else {
        Err(DomainError::Other(format!(
            "terminating substrate pid {pid} exited with {status}"
        )))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::path::PathBuf;
    use std::sync::mpsc;
    use std::time::Duration;

    fn task() -> AgentTask {
        AgentTask {
            wp_id: "WP01".to_string(),
            feature_slug: "demo-feature".to_string(),
            prompt_path: PathBuf::from("docs/specs/demo-feature/tasks/WP01.md"),
            worktree_path: PathBuf::from("."),
            context_files: vec![],
        }
    }

    #[test]
    fn cockpit_update_uses_feature_session_and_run_id() {
        let update = cockpit_update_for("run-1", &task(), "running", 0.25, "agent dispatched");

        assert_eq!(update.session_id, "demo-feature");
        assert_eq!(update.run_id, "run-1");
        assert_eq!(update.phase, "running");
        assert_eq!(update.progress, 0.25);
        assert!(update.summary.contains("WP01"));
    }

    #[test]
    fn cockpit_update_serializes_dashboard_contract() {
        let value = serde_json::to_value(cockpit_update_for(
            "run-1",
            &task(),
            "completed",
            1.0,
            "agent completed",
        ))
        .unwrap();

        assert_eq!(value["session_id"], "demo-feature");
        assert_eq!(value["run_id"], "run-1");
        assert_eq!(value["phase"], "completed");
        assert_eq!(value["progress"], 1.0);
        assert!(value.get("ownership_bracket").is_none());
    }

    #[test]
    fn cockpit_update_serializes_ownership_bracket_when_present() {
        let value = serde_json::to_value(CockpitUpdate {
            session_id: "demo-feature".to_string(),
            run_id: "run-1".to_string(),
            phase: "running".to_string(),
            summary: "demo-feature WP01: agent dispatched".to_string(),
            progress: 0.5,
            ownership_bracket: Some(
                "[pheno:✓, AgilePlus:✓, Substrate:✓, Tracaera:◐, phenotype-registry:✓]".to_string(),
            ),
        })
        .unwrap();

        assert_eq!(
            value["ownership_bracket"],
            "[pheno:✓, AgilePlus:✓, Substrate:✓, Tracaera:◐, phenotype-registry:✓]"
        );
    }

    #[test]
    fn emit_cockpit_update_posts_json_payload() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!(
            "http://{}/api/dashboard/cockpit",
            listener.local_addr().unwrap()
        );
        let (tx, rx) = mpsc::channel();

        std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(2)))
                .unwrap();
            let mut request = Vec::new();
            let mut buf = [0_u8; 4096];
            loop {
                match stream.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        request.extend_from_slice(&buf[..n]);
                        if request.windows(4).any(|window| window == b"\r\n\r\n")
                            && request
                                .windows(br#""progress":0.5"#.len())
                                .any(|window| window == br#""progress":0.5"#)
                        {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
            tx.send(String::from_utf8_lossy(&request).into_owned())
                .unwrap();
            stream
                .write_all(b"HTTP/1.1 200 OK\r\ncontent-length: 0\r\n\r\n")
                .unwrap();
        });

        emit_cockpit_update_blocking(
            Some(&url),
            cockpit_update_for("run-1", &task(), "running", 0.5, "agent dispatched"),
        );

        let request = rx.recv_timeout(Duration::from_secs(2)).unwrap();
        assert!(request.starts_with("POST /api/dashboard/cockpit HTTP/1.1"));
        assert!(request.contains(r#""session_id":"demo-feature""#));
        assert!(request.contains(r#""run_id":"run-1""#));
        assert!(request.contains(r#""phase":"running""#));
    }

    #[tokio::test]
    async fn emit_cockpit_update_posts_json_payload_from_async_context() {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let url = format!(
            "http://{}/api/dashboard/cockpit",
            listener.local_addr().unwrap()
        );
        let (tx, rx) = mpsc::channel();

        std::thread::spawn(move || {
            let (mut stream, _) = listener.accept().unwrap();
            stream
                .set_read_timeout(Some(Duration::from_secs(2)))
                .unwrap();
            let mut request = Vec::new();
            let mut buf = [0_u8; 4096];
            loop {
                match stream.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        request.extend_from_slice(&buf[..n]);
                        if request.windows(4).any(|window| window == b"\r\n\r\n")
                            && request
                                .windows(br#""progress":0.5"#.len())
                                .any(|window| window == br#""progress":0.5"#)
                        {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
            tx.send(String::from_utf8_lossy(&request).into_owned())
                .unwrap();
            stream
                .write_all(b"HTTP/1.1 200 OK\r\ncontent-length: 0\r\n\r\n")
                .unwrap();
        });

        emit_cockpit_update(
            Some(&url),
            cockpit_update_for("run-1", &task(), "running", 0.5, "agent dispatched"),
        )
        .await;

        let request = rx.recv_timeout(Duration::from_secs(2)).unwrap();
        assert!(request.starts_with("POST /api/dashboard/cockpit HTTP/1.1"));
        assert!(request.contains(r#""session_id":"demo-feature""#));
        assert!(request.contains(r#""run_id":"run-1""#));
        assert!(request.contains(r#""phase":"running""#));
    }

    #[test]
    fn build_substrate_dispatch_matches_current_cli_contract() {
        let config = AgentConfig {
            kind: AgentKind::Codex,
            max_review_cycles: 3,
            timeout_secs: 3600,
            extra_args: vec![],
        };

        let command =
            build_substrate_dispatch("substrate", "run-1", &task(), &config, "do the work", false)
                .unwrap();
        let args: Vec<String> = command
            .get_args()
            .map(|arg| arg.to_string_lossy().into_owned())
            .collect();

        assert_eq!(args[0], "dispatch");
        assert!(args
            .windows(2)
            .any(|window| window == ["--engine", "codex"]));
        assert!(args
            .windows(2)
            .any(|window| window == ["--mode", "foreground"]));
        assert!(args
            .windows(2)
            .any(|window| window == ["--agent", "subagent"]));
        assert_eq!(args.last().map(String::as_str), Some("do the work"));
        assert!(!args.iter().any(|arg| arg == "--prompt"));
        assert!(!args.iter().any(|arg| arg == "--provider"));
        assert!(!args.iter().any(|arg| arg == "--run-id"));
        assert!(!args.iter().any(|arg| arg == "--store"));
        assert!(!args.iter().any(|arg| arg == "--timeout-s"));
        assert!(!args.iter().any(|arg| arg == "--tier"));
    }
}
