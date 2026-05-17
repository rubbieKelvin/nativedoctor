//! Async helpers that load SQLite projects and attach them to [`AppState`][crate::state::AppState].
//!
//! All database work stays off the synchronous render path via [`gpui::App::spawn`].

use std::path::PathBuf;

use gpui::{App, AppContext, AsyncApp, Entity};
use std::sync::Arc;

use crate::state::{ActiveProject, AppState, SidebarTab};

fn finish_open(
    cx: &mut AsyncApp,
    state: Entity<AppState>,
    db_path: PathBuf,
    result: anyhow::Result<ActiveProject>,
) {
    cx.update_entity(&state, |app_state: &mut AppState, _cx| match result {
        Ok(active) => {
            app_state.attach_open_project(active);
        }
        Err(err) => {
            tracing::error!("Failed to open project at {:?}: {err}", db_path.display());
        }
    });
}

/// Open a SQLite project file, hydrate [`ActiveProject`], and navigate to workspace.
///
/// Logs errors on failure instead of presenting a modal (fine for MVP shell UI).
pub fn spawn_open_database(state: Entity<AppState>, db_path: PathBuf, cx: &mut App) {
    let cloned_state = state.clone();
    cx.spawn(async move |async_ctx| {
        let result = ActiveProject::open_and_load(db_path.clone()).await;
        finish_open(async_ctx, cloned_state, db_path, result);
    })
    .detach()
}

/// Create a SQLite project at `db_path` and hydrate UI collections.
///
/// Used by the landing page when the operator chooses **Create Project**.
pub fn spawn_create_database(
    state: Entity<AppState>,
    name: String,
    db_path: PathBuf,
    cx: &mut App,
) {
    let cloned_state = state.clone();

    cx.spawn(async move |async_ctx| {
        let result = ActiveProject::bootstrap_new(name, db_path.clone()).await;
        finish_open(async_ctx, cloned_state, db_path, result);
    })
    .detach()
}

/// Insert a scaffold request so the navigator never feels dormant on day zero.
///
/// Persists [`nd_db::models::Request::new`] with GET defaults, reloads caches, selects the insertion.
pub fn spawn_insert_skeleton_request(state: Entity<AppState>, cx: &mut App) {
    let Some(baseline) = state.read(cx).active_project.clone() else {
        return;
    };

    let store_handle = Arc::clone(&baseline.store);
    let project_key = baseline.project.id.clone();

    let relay = state.clone();

    cx.spawn(async move |async_ctx| {
        let draft_row = nd_db::models::Request::new(
            project_key,
            "New request",
            "GET",
            "https://example.com",
        );

        let persisted = match store_handle.insert_request(&draft_row).await {
            Ok(row) => row,
            Err(err) => {
                tracing::error!("Failed to insert skeleton HTTP request: {err}");
                return;
            }
        };

        let mut synced = baseline;
        if let Err(err) = synced.reload_from_store().await {
            tracing::error!("Navigator reload failed post-insertion: {err}");
            return;
        }

        synced.selected_request_id = Some(persisted.id);
        synced.selected_test_id = None;
        synced.selected_tab = SidebarTab::Requests;

        async_ctx.update_entity(&relay, |snapshot, _| {
            snapshot.active_project = Some(synced);
        });
    })
    .detach();
}

