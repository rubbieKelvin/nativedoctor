//! Local web UI for previewing and running nativedoctor request files (Dioxus 0.7 fullstack, server only).

use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use dioxus::prelude::*;
use dioxus_fullstack::ServerFnError;

struct WebConfig {
    root: PathBuf,
    no_default_system_env: bool,
    env_files: Vec<PathBuf>,
    verbose: bool,
}

static WEB_CONFIG: OnceLock<WebConfig> = OnceLock::new();

/// Configure workspace root, runtime env options, bind address via `IP` / `PORT` env vars, then
/// start the Dioxus Axum server. Does not return while the server is running.
pub fn run_web(
    root: PathBuf,
    bind: SocketAddr,
    no_default_system_env: bool,
    env_files: Vec<PathBuf>,
    verbose: bool,
) -> Result<(), String> {
    WEB_CONFIG
        .set(WebConfig {
            root,
            no_default_system_env,
            env_files,
            verbose,
        })
        .map_err(|_| "internal error: web config already set".to_string())?;

    // Dioxus serves static assets from `DIOXUS_PUBLIC_PATH` (default: `public` next to the binary).
    // That directory must exist or the server panics; create an empty folder.
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let exe_dir = exe
        .parent()
        .ok_or_else(|| "could not resolve executable directory".to_string())?;
    let public_dir = exe_dir.join("public");
    std::fs::create_dir_all(&public_dir).map_err(|e| format!("create Dioxus public dir: {e}"))?;
    std::env::set_var("DIOXUS_PUBLIC_PATH", &public_dir);

    std::env::set_var(dioxus::cli_config::SERVER_IP_ENV, bind.ip().to_string());
    std::env::set_var(
        dioxus::cli_config::SERVER_PORT_ENV,
        bind.port().to_string(),
    );

    dioxus::LaunchBuilder::server().launch(App);
    #[allow(unreachable_code)]
    Ok(())
}

fn resolve_file(root: &Path, name: &str) -> Result<PathBuf, ServerFnError> {
    if name.contains('/') || name.contains('\\') || name.contains("..") {
        return Err(ServerFnError::new("invalid file name"));
    }
    let p = root.join(name);
    if !p.is_file() {
        return Err(ServerFnError::new("file not found"));
    }
    let root_canon = std::fs::canonicalize(root).map_err(|e| ServerFnError::new(e.to_string()))?;
    let file_canon = std::fs::canonicalize(&p).map_err(|e| ServerFnError::new(e.to_string()))?;
    if !file_canon.starts_with(&root_canon) {
        return Err(ServerFnError::new("path escapes workspace"));
    }
    Ok(file_canon)
}

#[server]
pub async fn list_request_files() -> Result<Vec<String>, ServerFnError> {
    let root = &WEB_CONFIG
        .get()
        .ok_or_else(|| ServerFnError::new("workspace not configured"))?
        .root;
    let paths =
        nd_core::list_request_paths(root.as_path()).map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(paths
        .into_iter()
        .filter_map(|p| p.file_name().map(|s| s.to_string_lossy().into_owned()))
        .collect())
}

#[server]
pub async fn run_request_file(name: String) -> Result<String, ServerFnError> {
    let cfg = WEB_CONFIG
        .get()
        .ok_or_else(|| ServerFnError::new("workspace not configured"))?;
    let path = resolve_file(&cfg.root, &name)?;

    let env = nd_core::RuntimeEnv::from_cli_options(cfg.no_default_system_env, &cfg.env_files)
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let opts = nd_core::RunOptions {
        verbose: cfg.verbose,
        no_post_script: false,
        dry_run: false,
        allow_error_status: true,
        outcome_policy: nd_core::OutcomePolicy::SingleRequest,
    };

    let output = nd_core::execute_request_with_env(&path, &opts, &env)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    nd_core::execute_request_post_script(&output, &opts, &env, None)
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let body_preview = String::from_utf8_lossy(&output.body);
    let body_short: String = body_preview.chars().take(4000).collect();
    let truncated = if body_preview.len() > 4000 {
        format!("{body_short}\n… (truncated)")
    } else {
        body_short.to_string()
    };

    Ok(format!(
        "HTTP {}\nURL: {}\n\n{}",
        output.status, output.final_url, truncated
    ))
}

#[component]
fn App() -> Element {
    let listed = use_resource(|| async move { list_request_files().await });

    rsx! {
        style { r#"
            body {{ font-family: system-ui, sans-serif; margin: 1.5rem; max-width: 56rem; }}
            h1 {{ font-size: 1.25rem; }}
            ul {{ list-style: none; padding: 0; }}
            li {{ display: flex; flex-wrap: wrap; align-items: center; gap: 0.75rem; margin: 0.35rem 0; }}
            pre {{ background: #f4f4f5; padding: 0.75rem; overflow: auto; white-space: pre-wrap; flex: 1 1 100%; }}
            button {{ cursor: pointer; }}
        "# }
        h1 { "nativedoctor — request files" }
        p { "Files in the configured workspace directory (non-recursive). Runs use the same runtime layering as the CLI: global `--env` / `--no-default-system-env`, cwd `runtime.nativedoctor.json`, then process env (unless disabled)." }
        match &*listed.value().read_unchecked() {
            None => rsx! { p { "Loading…" } },
            Some(Err(e)) => rsx! { p { "Error: {e}" } },
            Some(Ok(files)) if files.is_empty() => rsx! { p { "No request files found." } },
            Some(Ok(files)) => rsx! {
                ul {
                    for name in files {
                        FileRow { name: name.clone() }
                    }
                }
            },
        }
    }
}

#[component]
fn FileRow(name: String) -> Element {
    let mut output = use_signal(String::new);
    let mut pending = use_signal(|| false);
    let name_clone = name.clone();

    let run = move |_| {
        let n = name_clone.clone();
        spawn(async move {
            pending.set(true);
            match run_request_file(n).await {
                Ok(s) => output.set(s),
                Err(e) => output.set(format!("Error: {e}")),
            }
            pending.set(false);
        });
    };

    rsx! {
        li {
            code { "{name}" }
            button {
                disabled: *pending.read(),
                onclick: run,
                if *pending.read() { "Running…" } else { "Run" }
            }
        }
        if !output.read().is_empty() {
            pre { "{output}" }
        }
    }
}
