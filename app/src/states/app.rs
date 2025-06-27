use std::{collections::HashMap, path::{Path, PathBuf}};

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
pub enum RequestLoadingStatus {
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
    pub requests: Signal<RequestLoadingStatus>,
    pub creation_status: Signal<ProjectCreationStatus>,
}

impl ApplicationState {
    pub fn provide() -> ApplicationState {
        return use_context_provider(|| ApplicationState {
            project: Signal::new(ProjectContentLoadingStatus::None),
            requests: Signal::new(RequestLoadingStatus::None),
            creation_status: Signal::new(ProjectCreationStatus::None),
        });
    }

    pub fn inject() -> ApplicationState {
        return use_context::<ApplicationState>();
    }

    // returns the current file's filename and the project name (if available)
    pub fn current_project_title(&self) -> Option<(String, Option<String>)> {
        return match &*self.project.read() {
            ProjectContentLoadingStatus::Loaded(content) => {
                let filename = content.path.file_name().unwrap().to_str().unwrap();
                let projectname = content.schema.project.clone().map(|p| p.name);
                Some((filename.to_string(), projectname.clone()))
            }
            _ => None,
        };
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
                *requests = RequestLoadingStatus::Loading;

                let mut buff = Vec::<LoadedRequestObject>::new();
                content.dump_requests(&mut buff).await;

                *requests = RequestLoadingStatus::Loaded(buff);
            }
            _ => {
                *requests = RequestLoadingStatus::None;
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

    pub fn add_new_request(&mut self) -> uuid::Uuid {
        let request = LoadedRequestObject::empty();
        let id = request.id.clone();
        self.requests.with_mut(|status| match status {
            RequestLoadingStatus::Loaded(requests) => {
                requests.push(request);
            }
            _ => {
                tracing::error!("Attempting to add new request to unloaded request stack");
            }
        });
        return id;
    }

    pub fn computed_requests(&self) -> Vec<LoadedRequestObject> {
        return match &*self.requests.read() {
            RequestLoadingStatus::Loaded(requests) => requests.clone(),
            _ => vec![],
        };
    }

    pub async fn save_project(&mut self) {
        // group all the request files
        let request_objects_to_save = self.computed_requests();
        
        let saved_schemas: HashMap<PathBuf, LoadedRootObject> = HashMap::new();
        let mut request_map: HashMap<PathBuf, Vec<LoadedRequestObject>> = HashMap::new();

        for request in request_objects_to_save {
            if request_map.contains_key(&request.path) {
                if let Some(container) = request_map.get_mut(&request.path){
                    container.push(request.clone());
                };
            }else{
                request_map.insert(request.path.clone(), vec![request.clone()]);
            };
        };

        // create schema for each path to be save, then load them with all the requests that should be there
        for (path, requests) in request_map {
            // TODO: I stopped here. I'm tired
            // create the loaded root object, load the requests in the schema
            // if the object has only one request
                // if the pathbuf is not empty, delete the old file in path buf
                // change the path buf file name to the request name
            // let schema = LoadedRootObject
            // schema.save().await;
        }
    }
}
