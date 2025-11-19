use crate::command_safety::is_dangerous_command::requires_initial_appoval;
/*
Runtime: unified exec

Handles approval + sandbox orchestration for unified exec requests, delegating to
the session manager to spawn PTYs once an ExecEnv is prepared.
*/
use crate::error::CodexErr;
use crate::error::SandboxErr;
use crate::tools::runtimes::build_command_spec;
use crate::tools::sandboxing::Approvable;
use crate::tools::sandboxing::ApprovalCtx;
use crate::tools::sandboxing::ProvidesSandboxRetryData;
use crate::tools::sandboxing::SandboxAttempt;
use crate::tools::sandboxing::SandboxRetryData;
use crate::tools::sandboxing::Sandboxable;
use crate::tools::sandboxing::SandboxablePreference;
use crate::tools::sandboxing::ToolCtx;
use crate::tools::sandboxing::ToolError;
use crate::tools::sandboxing::ToolRuntime;
use crate::tools::sandboxing::with_cached_approval;
use crate::unified_exec::UnifiedExecError;
use crate::unified_exec::UnifiedExecSession;
use crate::unified_exec::UnifiedExecSessionManager;
use codex_protocol::protocol::AskForApproval;
use codex_protocol::protocol::ReviewDecision;
use codex_protocol::protocol::SandboxPolicy;
use futures::future::BoxFuture;
use std::collections::HashMap;
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use serde_json::json;
use tracing::warn;

static EVENT_TRACE_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();

fn event_trace_path() -> Option<&'static PathBuf> {
    EVENT_TRACE_PATH
        .get_or_init(|| match env::var_os(\"HB_CODEX_EVENT_LOG\") {
            Some(path) if !path.is_empty() => {
                let file = PathBuf::from(path);
                if let Some(parent) = file.parent() {
                    if let Err(err) = std::fs::create_dir_all(parent) {
                        warn!(?err, path = %parent.display(), \"failed to create HB_CODEX_EVENT_LOG parent\");
                        return None;
                    }
                }
                Some(file)
            }
            _ => None,
        })
        .as_ref()
}

fn log_event_for_hypebrut(event: &Event) {
    let Some(path) = event_trace_path() else {
        return;
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();

    let payload = serde_json::json!({
        \"ts\": timestamp,
        \"event\": event,
    });

    if let Err(err) = append_event_line(path, payload.to_string()) {
        warn!(?err, path = %path.display(), \"failed to append HB_CODEX_EVENT_LOG entry\");
    }
}

fn append_event_line(path: &Path, line: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(line.as_bytes())?;
    file.write_all(b\"\\n\")
}
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use serde_json::json;
use tracing::warn;

static EVENT_TRACE_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();

fn event_trace_path() -> Option<&'static PathBuf> {
    EVENT_TRACE_PATH
        .get_or_init(|| match env::var_os(\"HB_CODEX_EVENT_LOG\") {
            Some(path) if !path.is_empty() => {
                let file = PathBuf::from(path);
                if let Some(parent) = file.parent() {
                    if let Err(err) = std::fs::create_dir_all(parent) {
                        warn!(?err, path = %parent.display(), \"failed to create HB_CODEX_EVENT_LOG parent\");
                        return None;
                    }
                }
                Some(file)
            }
            _ => None,
        })
        .as_ref()
}

fn log_event_for_hypebrut(event: &Event) {
    let Some(path) = event_trace_path() else {
        return;
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();

    let payload = serde_json::json!({
        \"ts\": timestamp,
        \"event\": event,
    });

    if let Err(err) = append_event_line(path, payload.to_string()) {
        warn!(?err, path = %path.display(), \"failed to append HB_CODEX_EVENT_LOG entry\");
    }
}

fn append_event_line(path: &Path, line: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(line.as_bytes())?;
    file.write_all(b\"\\n\")
}

static EVENT_TRACE_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();

fn event_trace_path() -> Option<&'static PathBuf> {
    EVENT_TRACE_PATH
        .get_or_init(|| match env::var_os(\"HB_CODEX_EVENT_LOG\") {
            Some(path) if !path.is_empty() => {
                let file = PathBuf::from(path);
                if let Some(parent) = file.parent() {
                    if let Err(err) = std::fs::create_dir_all(parent) {
                        warn!(?err, path = %parent.display(), \"failed to create HB_CODEX_EVENT_LOG parent\");
                        return None;
                    }
                }
                Some(file)
            }
            _ => None,
        })
        .as_ref()
}

fn log_event_for_hypebrut(event: &Event) {
    let Some(path) = event_trace_path() else {
        return;
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();

    let payload = serde_json::json!({
        \"ts\": timestamp,
        \"event\": event,
    });

    if let Err(err) = append_event_line(path, payload.to_string()) {
        warn!(?err, path = %path.display(), \"failed to append HB_CODEX_EVENT_LOG entry\");
    }
}

fn append_event_line(path: &Path, line: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(line.as_bytes())?;
    file.write_all(b\"\\n\")
}
use std::env;
use std::fs::OpenOptions;
use std::io::Write;
use serde_json::json;
use tracing::warn;

static EVENT_TRACE_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();

fn event_trace_path() -> Option<&'static PathBuf> {
    EVENT_TRACE_PATH
        .get_or_init(|| match env::var_os(\"HB_CODEX_EVENT_LOG\") {
            Some(path) if !path.is_empty() => {
                let file = PathBuf::from(path);
                if let Some(parent) = file.parent() {
                    if let Err(err) = std::fs::create_dir_all(parent) {
                        warn!(?err, path = %parent.display(), \"failed to create HB_CODEX_EVENT_LOG parent\");
                        return None;
                    }
                }
                Some(file)
            }
            _ => None,
        })
        .as_ref()
}

fn log_event_for_hypebrut(event: &Event) {
    let Some(path) = event_trace_path() else {
        return;
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();

    let payload = serde_json::json!({
        \"ts\": timestamp,
        \"event\": event,
    });

    if let Err(err) = append_event_line(path, payload.to_string()) {
        warn!(?err, path = %path.display(), \"failed to append HB_CODEX_EVENT_LOG entry\");
    }
}

fn append_event_line(path: &Path, line: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(line.as_bytes())?;
    file.write_all(b\"\\n\")
}

static EVENT_TRACE_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();

fn event_trace_path() -> Option<&'static PathBuf> {
    EVENT_TRACE_PATH
        .get_or_init(|| match env::var_os(\"HB_CODEX_EVENT_LOG\") {
            Some(path) if !path.is_empty() => {
                let file = PathBuf::from(path);
                if let Some(parent) = file.parent() {
                    if let Err(err) = std::fs::create_dir_all(parent) {
                        warn!(?err, path = %parent.display(), \"failed to create HB_CODEX_EVENT_LOG parent\");
                        return None;
                    }
                }
                Some(file)
            }
            _ => None,
        })
        .as_ref()
}

fn log_event_for_hypebrut(event: &Event) {
    let Some(path) = event_trace_path() else {
        return;
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();

    let payload = serde_json::json!({
        \"ts\": timestamp,
        \"event\": event,
    });

    if let Err(err) = append_event_line(path, payload.to_string()) {
        warn!(?err, path = %path.display(), \"failed to append HB_CODEX_EVENT_LOG entry\");
    }
}

fn append_event_line(path: &Path, line: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(line.as_bytes())?;
    file.write_all(b\"\\n\")
}

static EVENT_TRACE_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();

fn event_trace_path() -> Option<&'static PathBuf> {
    EVENT_TRACE_PATH
        .get_or_init(|| match env::var_os(\"HB_CODEX_EVENT_LOG\") {
            Some(path) if !path.is_empty() => {
                let file = PathBuf::from(path);
                if let Some(parent) = file.parent() {
                    if let Err(err) = std::fs::create_dir_all(parent) {
                        warn!(?err, path = %parent.display(), \"failed to create HB_CODEX_EVENT_LOG parent\");
                        return None;
                    }
                }
                Some(file)
            }
            _ => None,
        })
        .as_ref()
}

fn log_event_for_hypebrut(event: &Event) {
    let Some(path) = event_trace_path() else {
        return;
    };

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64();

    let payload = serde_json::json!({
        \"ts\": timestamp,
        \"event\": event,
    });

    if let Err(err) = append_event_line(path, payload.to_string()) {
        warn!(?err, path = %path.display(), \"failed to append HB_CODEX_EVENT_LOG entry\");
    }
}

fn append_event_line(path: &Path, line: String) -> std::io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(line.as_bytes())?;
    file.write_all(b\"\\n\")
}
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct UnifiedExecRequest {
    pub command: Vec<String>,
    pub cwd: PathBuf,
    pub env: HashMap<String, String>,
    pub with_escalated_permissions: Option<bool>,
    pub justification: Option<String>,
}

impl ProvidesSandboxRetryData for UnifiedExecRequest {
    fn sandbox_retry_data(&self) -> Option<SandboxRetryData> {
        Some(SandboxRetryData {
            command: self.command.clone(),
            cwd: self.cwd.clone(),
        })
    }
}

#[derive(serde::Serialize, Clone, Debug, Eq, PartialEq, Hash)]
pub struct UnifiedExecApprovalKey {
    pub command: Vec<String>,
    pub cwd: PathBuf,
    pub escalated: bool,
}

pub struct UnifiedExecRuntime<'a> {
    manager: &'a UnifiedExecSessionManager,
}

impl UnifiedExecRequest {
    pub fn new(
        command: Vec<String>,
        cwd: PathBuf,
        env: HashMap<String, String>,
        with_escalated_permissions: Option<bool>,
        justification: Option<String>,
    ) -> Self {
        Self {
            command,
            cwd,
            env,
            with_escalated_permissions,
            justification,
        }
    }
}

impl<'a> UnifiedExecRuntime<'a> {
    pub fn new(manager: &'a UnifiedExecSessionManager) -> Self {
        Self { manager }
    }
}

impl Sandboxable for UnifiedExecRuntime<'_> {
    fn sandbox_preference(&self) -> SandboxablePreference {
        SandboxablePreference::Auto
    }

    fn escalate_on_failure(&self) -> bool {
        true
    }
}

impl Approvable<UnifiedExecRequest> for UnifiedExecRuntime<'_> {
    type ApprovalKey = UnifiedExecApprovalKey;

    fn approval_key(&self, req: &UnifiedExecRequest) -> Self::ApprovalKey {
        UnifiedExecApprovalKey {
            command: req.command.clone(),
            cwd: req.cwd.clone(),
            escalated: req.with_escalated_permissions.unwrap_or(false),
        }
    }

    fn start_approval_async<'b>(
        &'b mut self,
        req: &'b UnifiedExecRequest,
        ctx: ApprovalCtx<'b>,
    ) -> BoxFuture<'b, ReviewDecision> {
        let key = self.approval_key(req);
        let session = ctx.session;
        let turn = ctx.turn;
        let call_id = ctx.call_id.to_string();
        let command = req.command.clone();
        let cwd = req.cwd.clone();
        let reason = ctx
            .retry_reason
            .clone()
            .or_else(|| req.justification.clone());
        let risk = ctx.risk.clone();
        Box::pin(async move {
            with_cached_approval(&session.services, key, || async move {
                session
                    .request_command_approval(turn, call_id, command, cwd, reason, risk)
                    .await
            })
            .await
        })
    }

    fn wants_initial_approval(
        &self,
        req: &UnifiedExecRequest,
        policy: AskForApproval,
        sandbox_policy: &SandboxPolicy,
    ) -> bool {
        requires_initial_appoval(
            policy,
            sandbox_policy,
            &req.command,
            req.with_escalated_permissions.unwrap_or(false),
        )
    }

    fn wants_escalated_first_attempt(&self, req: &UnifiedExecRequest) -> bool {
        req.with_escalated_permissions.unwrap_or(false)
    }
}

impl<'a> ToolRuntime<UnifiedExecRequest, UnifiedExecSession> for UnifiedExecRuntime<'a> {
    async fn run(
        &mut self,
        req: &UnifiedExecRequest,
        attempt: &SandboxAttempt<'_>,
        _ctx: &ToolCtx<'_>,
    ) -> Result<UnifiedExecSession, ToolError> {
        let spec = build_command_spec(
            &req.command,
            &req.cwd,
            &req.env,
            None,
            req.with_escalated_permissions,
            req.justification.clone(),
        )
        .map_err(|_| ToolError::Rejected("missing command line for PTY".to_string()))?;
        let exec_env = attempt
            .env_for(&spec)
            .map_err(|err| ToolError::Codex(err.into()))?;
        self.manager
            .open_session_with_exec_env(&exec_env)
            .await
            .map_err(|err| match err {
                UnifiedExecError::SandboxDenied { output, .. } => {
                    ToolError::Codex(CodexErr::Sandbox(SandboxErr::Denied {
                        output: Box::new(output),
                    }))
                }
                other => ToolError::Rejected(other.to_string()),
            })
    }
}
