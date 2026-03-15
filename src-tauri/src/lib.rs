// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::path::PathBuf;
use std::sync::Mutex;

use crate::db::init_db;

mod app;
mod client;
mod constants;
mod db;
mod project;
mod resource;
mod schema;

fn setup_db() -> Result<(), String> {
    let db_path = db::get_db_path()?;
    let mut conn = rusqlite::Connection::open(&db_path).map_err(|e| e.to_string())?;
    return init_db(&mut conn);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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

    let http_client = reqwest::Client::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_opener::init())
        .manage(app::AppState {
            initial_project_path: Mutex::new(initial_path),
        })
        .manage(client::HttpClientState {
            client: http_client,
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
