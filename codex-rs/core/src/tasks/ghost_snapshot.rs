use crate::codex::TurnContext;
use crate::state::TaskKind;
use crate::tasks::SessionTask;
use crate::tasks::SessionTaskContext;
use async_trait::async_trait;
use codex_git::CreateGhostCommitOptions;
use codex_git::GitToolingError;
use codex_git::create_ghost_commit;
use codex_protocol::models::ResponseItem;
use codex_protocol::user_input::UserInput;
use codex_utils_readiness::Readiness;
use codex_utils_readiness::Token;
use std::sync::{Arc, OnceLock};
use tokio_util::sync::CancellationToken;
use tracing::info;
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

pub(crate) struct GhostSnapshotTask {
    token: Token,
}

#[async_trait]
impl SessionTask for GhostSnapshotTask {
    fn kind(&self) -> TaskKind {
        TaskKind::Regular
    }

    async fn run(
        self: Arc<Self>,
        session: Arc<SessionTaskContext>,
        ctx: Arc<TurnContext>,
        _input: Vec<UserInput>,
        cancellation_token: CancellationToken,
    ) -> Option<String> {
        tokio::task::spawn(async move {
            let token = self.token;
            let ctx_for_task = Arc::clone(&ctx);
            let cancelled = tokio::select! {
                _ = cancellation_token.cancelled() => true,
                _ = async {
                    let repo_path = ctx_for_task.cwd.clone();
                    // Required to run in a dedicated blocking pool.
                    match tokio::task::spawn_blocking(move || {
                        let options = CreateGhostCommitOptions::new(&repo_path);
                        create_ghost_commit(&options)
                    })
                    .await
                    {
                        Ok(Ok(ghost_commit)) => {
                            info!("ghost snapshot blocking task finished");
                            session
                                .session
                                .record_conversation_items(&ctx, &[ResponseItem::GhostSnapshot {
                                    ghost_commit: ghost_commit.clone(),
                                }])
                                .await;
                            info!("ghost commit captured: {}", ghost_commit.id());
                        }
                        Ok(Err(err)) => {
                            warn!(
                                sub_id = ctx_for_task.sub_id.as_str(),
                                "failed to capture ghost snapshot: {err}"
                            );
                            let message = match err {
                                GitToolingError::NotAGitRepository { .. } => {
                                    "Snapshots disabled: current directory is not a Git repository."
                                        .to_string()
                                }
                                _ => format!("Snapshots disabled after ghost snapshot error: {err}."),
                            };
                            session
                                .session
                                .notify_background_event(&ctx_for_task, message)
                                .await;
                        }
                        Err(err) => {
                            warn!(
                                sub_id = ctx_for_task.sub_id.as_str(),
                                "ghost snapshot task panicked: {err}"
                            );
                            let message =
                                format!("Snapshots disabled after ghost snapshot panic: {err}.");
                            session
                                .session
                                .notify_background_event(&ctx_for_task, message)
                                .await;
                        }
                    }
                } => false,
            };

            if cancelled {
                info!("ghost snapshot task cancelled");
            }

            match ctx.tool_call_gate.mark_ready(token).await {
                Ok(true) => info!("ghost snapshot gate marked ready"),
                Ok(false) => warn!("ghost snapshot gate already ready"),
                Err(err) => warn!("failed to mark ghost snapshot ready: {err}"),
            }
        });
        None
    }
}

impl GhostSnapshotTask {
    pub(crate) fn new(token: Token) -> Self {
        Self { token }
    }
}
