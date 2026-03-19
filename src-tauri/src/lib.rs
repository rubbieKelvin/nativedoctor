// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::path::PathBuf;
use std::sync::Mutex;

use crate::db::init_db;
use tracing_subscriber::EnvFilter;
use tracing::{error, info};

mod app;
mod client;
mod constants;
mod db;
mod project;
mod resource;
mod schema;
mod scripting;

fn setup_db() -> Result<(), String> {
    let db_path = db::get_db_path()?;
    let db_path_display = db_path.to_string_lossy();
    info!(db_path = %db_path_display, "Opening database");
    let mut conn = rusqlite::Connection::open(&db_path).map_err(|e| {
        let msg = e.to_string();
        error!(db_path = %db_path_display, error = %msg, "Failed to open database");
        msg
    })?;
    if let Err(e) = init_db(&mut conn) {
        error!(
            db_path = %db_path_display,
            error = %e,
            "Failed to initialize DB schema/migrations"
        );
        return Err(e);
    }
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize structured logging early so we capture startup + migration issues.
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .try_init();

    let initial_path = std::env::args()
        .nth(1)
        .filter(|s| !s.is_empty())
        .and_then(|s| {
            let p = PathBuf::from(&s);
            if p.exists() && p.is_dir() {
                std::fs::canonicalize(&p).ok().or(Some(p))
            } else {
                None
            }
        });

    let http_client_http1 = reqwest::Client::builder()
        .http1_only()
        .build()
        .expect("HTTP/1.1 client");
    let http_client_http2 = reqwest::Client::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(app::AppState {
            initial_project_path: Mutex::new(initial_path),
        })
        .manage(client::HttpClientState {
            client_http1: http_client_http1,
            client_http2: http_client_http2,
        })
        .invoke_handler(tauri::generate_handler![
            client::send_http_request,
            project::get_initial_project_path,
            project::get_recent_projects,
            project::add_recent_project,
            project::project_has_nativedoctor,
            project::read_nativedoctor,
            project::write_nativedoctor,
            project::create_project,
            project::get_project_root_from_config_path,
            project::discover_resources,
            project::load_env_file,
            resource::read_resource_file,
            resource::write_resource_file,
        ])
        .setup(|_app| {
            setup_db()?;
            return Ok(());
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
