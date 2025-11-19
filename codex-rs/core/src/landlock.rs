use crate::protocol::SandboxPolicy;
use crate::spawn::StdioPolicy;
use crate::spawn::spawn_child_async;
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
use std::path::Path;
use std::path::{Path, PathBuf};
use tokio::process::Child;

/// Spawn a shell tool command under the Linux Landlock+seccomp sandbox helper
/// (codex-linux-sandbox).
///
/// Unlike macOS Seatbelt where we directly embed the policy text, the Linux
/// helper accepts a list of `--sandbox-permission`/`-s` flags mirroring the
/// public CLI. We convert the internal [`SandboxPolicy`] representation into
/// the equivalent CLI options.
pub async fn spawn_command_under_linux_sandbox<P>(
    codex_linux_sandbox_exe: P,
    command: Vec<String>,
    command_cwd: PathBuf,
    sandbox_policy: &SandboxPolicy,
    sandbox_policy_cwd: &Path,
    stdio_policy: StdioPolicy,
    env: HashMap<String, String>,
) -> std::io::Result<Child>
where
    P: AsRef<Path>,
{
    let args = create_linux_sandbox_command_args(command, sandbox_policy, sandbox_policy_cwd);
    let arg0 = Some("codex-linux-sandbox");
    spawn_child_async(
        codex_linux_sandbox_exe.as_ref().to_path_buf(),
        args,
        arg0,
        command_cwd,
        sandbox_policy,
        stdio_policy,
        env,
    )
    .await
}

/// Converts the sandbox policy into the CLI invocation for `codex-linux-sandbox`.
pub(crate) fn create_linux_sandbox_command_args(
    command: Vec<String>,
    sandbox_policy: &SandboxPolicy,
    sandbox_policy_cwd: &Path,
) -> Vec<String> {
    #[expect(clippy::expect_used)]
    let sandbox_policy_cwd = sandbox_policy_cwd
        .to_str()
        .expect("cwd must be valid UTF-8")
        .to_string();

    #[expect(clippy::expect_used)]
    let sandbox_policy_json =
        serde_json::to_string(sandbox_policy).expect("Failed to serialize SandboxPolicy to JSON");

    let mut linux_cmd: Vec<String> = vec![
        "--sandbox-policy-cwd".to_string(),
        sandbox_policy_cwd,
        "--sandbox-policy".to_string(),
        sandbox_policy_json,
        // Separator so that command arguments starting with `-` are not parsed as
        // options of the helper itself.
        "--".to_string(),
    ];

    // Append the original tool command.
    linux_cmd.extend(command);

    linux_cmd
}
