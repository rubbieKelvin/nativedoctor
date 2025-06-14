use std::path::PathBuf;

use dioxus::prelude::*;
use nd_core::schema::{request::RequestSchema, root::{LoadedRootObject, RootSchema}};
use rfd::AsyncFileDialog;

use crate::constants::{APP_NAME, FILE_EXTENSION};

#[derive(Clone, PartialEq)]
pub enum ProjectContentLoadingState {
    None,
    Loading(PathBuf),
    Loaded(LoadedRootObject),
    Error(String),
}

#[derive(Clone, PartialEq)]
pub struct ApplicationState {
    pub project: Signal<ProjectContentLoadingState>,
    pub requests: Signal<Vec<RequestSchema>>,
}

impl ApplicationState {
    pub fn provide() -> ApplicationState {
        return use_context_provider(|| ApplicationState {
            project: Signal::new(ProjectContentLoadingState::None),
            requests: Signal::new(vec![]),
        });
    }

    pub fn inject() -> ApplicationState {
        return use_context::<ApplicationState>();
    }

    pub async fn open_project(&mut self) {
        let mut project = self.project.write();
        let picker = AsyncFileDialog::new()
            .set_title(format!("Select {} file", APP_NAME))
            .add_filter("Project", &[FILE_EXTENSION]);

        if let Some(path) = picker.pick_file().await {
            let path = path.path();
            match path.try_exists() {
                Ok(exists) => {
                    if !exists && !path.is_file() {
                        *project = ProjectContentLoadingState::Error(
                            "Could not open file path".to_string(),
                        );
                        return;
                    }

                    // set loading
                    *project = ProjectContentLoadingState::Loading(path.to_path_buf());

                    // load content
                    let content = RootSchema::load_recursive(path).await;

                    match content {
                        Ok(content) => {
                            *project = ProjectContentLoadingState::Loaded(content);
                        }
                        Err(err) => {
                            *project = ProjectContentLoadingState::Error(err.to_string());
                        }
                    };
                }
                Err(err) => {
                    *project = ProjectContentLoadingState::Error(err.to_string());
                }
            };
        }
    }
}
