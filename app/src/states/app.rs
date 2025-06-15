use std::path::{Path, PathBuf};

use dioxus::prelude::*;
use nd_core::{
    init::initialize_project,
    schema::{
        file_object::{LoadedRequestObject, LoadedRootObject},
        root::RootSchema,
    },
};
use rfd::AsyncFileDialog;

use nd_core::constants::{APP_NAME, FILE_EXTENSIONS};

#[derive(Clone, PartialEq, Debug)]
pub enum ProjectContentLoadingStatus {
    None,
    Loading(PathBuf),
    Loaded(LoadedRootObject),
    Error(String),
}

#[derive(Clone, PartialEq)]
pub enum RequestLoadingState {
    None,
    Loading,
    Loaded(Vec<LoadedRequestObject>),
}

#[allow(unused)]
#[derive(Clone, PartialEq)]
pub enum ProjectCreationStatus {
    None,
    Creating(PathBuf, String),
    Error(String),
}

#[derive(Clone, PartialEq)]
pub struct ApplicationState {
    pub project: Signal<ProjectContentLoadingStatus>,
    pub requests: Signal<RequestLoadingState>,
    pub creation_status: Signal<ProjectCreationStatus>,
}

impl ApplicationState {
    pub fn provide() -> ApplicationState {
        return use_context_provider(|| ApplicationState {
            project: Signal::new(ProjectContentLoadingStatus::None),
            requests: Signal::new(RequestLoadingState::None),
            creation_status: Signal::new(ProjectCreationStatus::None),
        });
    }

    pub fn inject() -> ApplicationState {
        return use_context::<ApplicationState>();
    }

    pub async fn open_project_with_picker(&mut self) {
        let mut project = self.project.write();
        let picker = AsyncFileDialog::new()
            .set_title(format!("Select {} file", APP_NAME))
            .add_filter("Project", FILE_EXTENSIONS);

        // try to open project
        if let Some(path) = picker.pick_file().await {
            let path = path.path();
            ApplicationState::load_project_from_path(&mut *project, path).await;
        }

        // try to load objects
        let mut requests = self.requests.write();

        match &*project {
            ProjectContentLoadingStatus::Loaded(content) => {
                // load requests
                *requests = RequestLoadingState::Loading;

                let mut buff = Vec::<LoadedRequestObject>::new();
                content.dump_requests(&mut buff).await;

                *requests = RequestLoadingState::Loaded(buff);
            }
            _ => {
                *requests = RequestLoadingState::None;
            }
        }
    }

    pub async fn create_project_with_picker(&mut self, name: &str) {
        let mut project = self.project.write();
        let mut creation_status = self.creation_status.write();
        let picker = AsyncFileDialog::new().set_title("Select folder for project files");

        if let Some(path) = picker.pick_folder().await {
            let path = path.path();
            if !path.is_dir() {
                *creation_status = ProjectCreationStatus::Error(
                    "Selected path for project creation is not a folder".to_string(),
                );
            }

            ApplicationState::create_project_at_path(
                &mut *creation_status,
                &mut *project,
                path,
                name,
            )
            .await;
        }
    }

    async fn load_project_from_path(project_status: &mut ProjectContentLoadingStatus, path: &Path) {
        tracing::debug!("Attempting to load project from path: {:?}", &path);

        match path.try_exists() {
            Ok(exists) => {
                tracing::debug!("Path exists");
                if !exists && !path.is_file() {
                    *project_status =
                        ProjectContentLoadingStatus::Error("Could not open file path".to_string());
                    return;
                }

                // set loading
                *project_status = ProjectContentLoadingStatus::Loading(path.to_path_buf());

                // load content
                let content = RootSchema::load_recursive(path).await;
                
                
                match content {
                    Ok(content) => {
                        tracing::info!("Successfully loaded project");
                        *project_status = ProjectContentLoadingStatus::Loaded(content);
                    }
                    Err(err) => {
                        let err_string = err.to_string();
                        tracing::debug!("Error loading project: {}", &err_string);
                        *project_status = ProjectContentLoadingStatus::Error(err_string);
                    }
                };
            }
            Err(err) => {
                *project_status = ProjectContentLoadingStatus::Error(err.to_string());
            }
        };
    }

    async fn create_project_at_path(
        creation_status: &mut ProjectCreationStatus,
        project: &mut ProjectContentLoadingStatus,
        path: &Path,
        name: &str,
    ) {
        *creation_status = ProjectCreationStatus::Creating(path.to_path_buf(), name.to_string());

        match initialize_project(path, name).await {
            Ok(creation_path) => {
                *creation_status = ProjectCreationStatus::None;
                // load the project after we've created it
                ApplicationState::load_project_from_path(project, creation_path.as_path()).await;
            }
            Err(err) => {
                *creation_status = ProjectCreationStatus::Error(err.to_string());
            }
        };
    }
}
