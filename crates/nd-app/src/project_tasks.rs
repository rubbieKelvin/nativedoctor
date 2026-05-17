use std::path::PathBuf;

use gpui::*;

use crate::state::AppState;

pub fn spawn_create_database(
    state: Entity<AppState>,
    name: String,
    location: PathBuf,
    cx: &mut App,
) {
    cx.spawn(|cx: &mut AsyncApp| {
        let cx = cx.clone();
        async move {
            let store = match crate::store::Store::init().await {
                Ok(s) => s,
                Err(_) => return,
            };
            match store.create_project(&name, &location).await {
                Ok(project) => {
                    let project_path = location.join(&name);
                    let _ = cx.update(|app| {
                        state.update(app, |state, _cx| {
                            state
                                .recent_projects
                                .push(crate::store::models::RecentProject {
                                    id: 0,
                                    name: project.name,
                                    path: project_path.to_string_lossy().to_string(),
                                    last_opened_at: String::new(),
                                });
                        });
                    });
                }
                Err(e) => {
                    tracing::error!("Failed to create project: {e}");
                }
            }
        }
    })
    .detach();
}

pub fn spawn_insert_skeleton_request(_state: Entity<AppState>, cx: &mut App) {
    cx.spawn(|_: &mut AsyncApp| async move {})
        .detach();
}
