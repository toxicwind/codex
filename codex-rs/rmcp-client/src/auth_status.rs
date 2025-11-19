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
use std::time::Duration;

use anyhow::Error;
use anyhow::Result;
use codex_protocol::protocol::McpAuthStatus;
use reqwest::Client;
use reqwest::StatusCode;
use reqwest::Url;
use reqwest::header::HeaderMap;
use serde::Deserialize;
use tracing::debug;

use crate::OAuthCredentialsStoreMode;
use crate::oauth::has_oauth_tokens;
use crate::utils::apply_default_headers;
use crate::utils::build_default_headers;

const DISCOVERY_TIMEOUT: Duration = Duration::from_secs(5);
const OAUTH_DISCOVERY_HEADER: &str = "MCP-Protocol-Version";
const OAUTH_DISCOVERY_VERSION: &str = "2024-11-05";

/// Determine the authentication status for a streamable HTTP MCP server.
pub async fn determine_streamable_http_auth_status(
    server_name: &str,
    url: &str,
    bearer_token_env_var: Option<&str>,
    http_headers: Option<HashMap<String, String>>,
    env_http_headers: Option<HashMap<String, String>>,
    store_mode: OAuthCredentialsStoreMode,
) -> Result<McpAuthStatus> {
    if bearer_token_env_var.is_some() {
        return Ok(McpAuthStatus::BearerToken);
    }

    if has_oauth_tokens(server_name, url, store_mode)? {
        return Ok(McpAuthStatus::OAuth);
    }

    let default_headers = build_default_headers(http_headers, env_http_headers)?;

    match supports_oauth_login_with_headers(url, &default_headers).await {
        Ok(true) => Ok(McpAuthStatus::NotLoggedIn),
        Ok(false) => Ok(McpAuthStatus::Unsupported),
        Err(error) => {
            debug!(
                "failed to detect OAuth support for MCP server `{server_name}` at {url}: {error:?}"
            );
            Ok(McpAuthStatus::Unsupported)
        }
    }
}

/// Attempt to determine whether a streamable HTTP MCP server advertises OAuth login.
pub async fn supports_oauth_login(url: &str) -> Result<bool> {
    supports_oauth_login_with_headers(url, &HeaderMap::new()).await
}

async fn supports_oauth_login_with_headers(url: &str, default_headers: &HeaderMap) -> Result<bool> {
    let base_url = Url::parse(url)?;
    let builder = Client::builder().timeout(DISCOVERY_TIMEOUT);
    let client = apply_default_headers(builder, default_headers).build()?;

    let mut last_error: Option<Error> = None;
    for candidate_path in discovery_paths(base_url.path()) {
        let mut discovery_url = base_url.clone();
        discovery_url.set_path(&candidate_path);

        let response = match client
            .get(discovery_url.clone())
            .header(OAUTH_DISCOVERY_HEADER, OAUTH_DISCOVERY_VERSION)
            .send()
            .await
        {
            Ok(response) => response,
            Err(err) => {
                last_error = Some(err.into());
                continue;
            }
        };

        if response.status() != StatusCode::OK {
            continue;
        }

        let metadata = match response.json::<OAuthDiscoveryMetadata>().await {
            Ok(metadata) => metadata,
            Err(err) => {
                last_error = Some(err.into());
                continue;
            }
        };

        if metadata.authorization_endpoint.is_some() && metadata.token_endpoint.is_some() {
            return Ok(true);
        }
    }

    if let Some(err) = last_error {
        debug!("OAuth discovery requests failed for {url}: {err:?}");
    }

    Ok(false)
}

#[derive(Debug, Deserialize)]
struct OAuthDiscoveryMetadata {
    #[serde(default)]
    authorization_endpoint: Option<String>,
    #[serde(default)]
    token_endpoint: Option<String>,
}

/// Implements RFC 8414 section 3.1 for discovering well-known oauth endpoints.
/// This is a requirement for MCP servers to support OAuth.
/// https://datatracker.ietf.org/doc/html/rfc8414#section-3.1
/// https://github.com/modelcontextprotocol/rust-sdk/blob/main/crates/rmcp/src/transport/auth.rs#L182
fn discovery_paths(base_path: &str) -> Vec<String> {
    let trimmed = base_path.trim_start_matches('/').trim_end_matches('/');
    let canonical = "/.well-known/oauth-authorization-server".to_string();

    if trimmed.is_empty() {
        return vec![canonical];
    }

    let mut candidates = Vec::new();
    let mut push_unique = |candidate: String| {
        if !candidates.contains(&candidate) {
            candidates.push(candidate);
        }
    };

    push_unique(format!("{canonical}/{trimmed}"));
    push_unique(format!("/{trimmed}/.well-known/oauth-authorization-server"));
    push_unique(canonical);

    candidates
}
