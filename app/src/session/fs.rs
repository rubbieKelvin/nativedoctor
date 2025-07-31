use nativedoctor_core::schema::roots::{
    EnvironmentRootSchema, ProjectRootSchema, RequestRootSchema,
};
use nativedoctor_core::{
    ENVIRONMENT_FOLDER_NAME, EXTENSION_FOR_ENVIRONMENT, EXTENSION_FOR_PROJECT,
    EXTENSION_FOR_REQUEST, REQUEST_FOLDER_NAME,
};

#[cfg(not(target_arch = "wasm32"))]
use rfd::AsyncFileDialog;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::meta::recents::RecentProjects;
use crate::session::Session;

impl Session {
    /// gets the path where the project was or should be stored
    async fn get_project_path(&self) -> Option<PathBuf> {
        // we have an actuall path (file has been saved before)
        if self.path.is_some() {
            return self.path.clone();
        }

        // file has not been saved before
        #[cfg(not(target_arch = "wasm32"))]
        {
            // open file dialog (desktop only)
            let picker = AsyncFileDialog::new().set_title("Pick folder to save project");
            return picker.pick_folder().await.map(|handler| {
                let path = handler.path();
                let root = path.join(format!("project.{EXTENSION_FOR_PROJECT}"));
                return root;
            });
        }

        #[cfg(target_arch = "wasm32")]
        {
            // File dialogs don't work in browsers
            // I'll need to check if rfd supports wasm
            return None;
        }
    }

    pub async fn save_to_fs(&self) -> Result<(), String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // TODO: Update return type
            // IDEA: we can keep track of what changed so we dont have to rewite the whole project
            let project_root = self.to_fs_schema();

            // Get project path and make sure it exists
            let project_path = self.get_project_path().await;

            if project_path.is_none() {
                tracing::error!("No file path available");
                return Err("Could not get file path".to_string());
            }

            let project_path = project_path.unwrap();

            // Get parent folder
            let parent_folder = project_path.parent().map(|p| p.to_path_buf());

            if parent_folder.is_none() {
                tracing::error!("Cannot get parent folder");
                return Err("No parent folder".to_string());
            }

            let parent_folder = parent_folder.unwrap();

            // Create requests and environment folders from parent folder
            let requests_path = parent_folder.join(REQUEST_FOLDER_NAME);
            let environments_path = parent_folder.join(ENVIRONMENT_FOLDER_NAME);

            // Create the request/enviroment folder if they do not exist
            if !requests_path.try_exists().unwrap_or_default() {
                fs::create_dir(&requests_path).map_err(|e| e.to_string())?;
            }

            if !environments_path.try_exists().unwrap_or_default() {
                fs::create_dir(&environments_path).map_err(|e| e.to_string())?;
            }

            // Build request files
            for request in self.requests.iter() {
                let filename = requests_path.clone().join(format!(
                    "{}.{}",
                    request.ref_id.clone(),
                    EXTENSION_FOR_REQUEST
                ));

                let schema = request.to_request_schema();

                let content = serde_yaml::to_string(&schema).map_err(|e| e.to_string())?;
                let mut file = fs::File::create(filename).map_err(|e| e.to_string())?;
                file.write_all(content.as_bytes())
                    .map_err(|e| e.to_string())?;
            }

            // Build Environment files
            for env in self.environments.iter() {
                let filename = environments_path.clone().join(format!(
                    "{}.{}",
                    env.ref_id.clone(),
                    EXTENSION_FOR_ENVIRONMENT
                ));

                let schema = env.to_environment_schema();

                let content = serde_yaml::to_string(&schema).map_err(|e| e.to_string())?;
                let mut file = fs::File::create(filename).map_err(|e| e.to_string())?;
                file.write_all(content.as_bytes())
                    .map_err(|e| e.to_string())?;
            }

            // TODO: handle these errors better
            let content = serde_yaml::to_string(&project_root).map_err(|e| e.to_string())?;
            let mut file = fs::File::create(project_path).map_err(|e| e.to_string())?;
            file.write_all(content.as_bytes())
                .map_err(|e| e.to_string())?;

            return Ok(());
        }

        #[cfg(target_arch = "wasm32")]
        {
            // File system operations not supported in browsers
            return Err("File system operations not supported in browsers".to_string());
        }
    }

    pub fn load_from_fs_from_path(path: PathBuf) -> Result<Self, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Paths and files
            let project_file = path.clone();
            let project_folder = project_file.parent().unwrap();
            let request_folder = project_folder.join(REQUEST_FOLDER_NAME);
            let environment_folder = project_folder.join(ENVIRONMENT_FOLDER_NAME);

            // recents
            let recents = RecentProjects::init();
            let recent_project = recents.get(path.to_string_lossy().to_string());

            // Request files
            let request_files = if request_folder.try_exists().unwrap_or(false) {
                fs::read_dir(request_folder)
                    .map_err(|e| e.to_string())?
                    .map(|e| e.unwrap().path())
                    .filter(|e| {
                        e.is_file() && e.extension().unwrap_or_default() == EXTENSION_FOR_REQUEST
                    })
                    .collect::<Vec<PathBuf>>()
            } else {
                vec![]
            };

            // Environment files
            let environment_files = if environment_folder.try_exists().unwrap_or(false) {
                fs::read_dir(environment_folder)
                    .map_err(|e| e.to_string())?
                    .map(|e| e.unwrap().path())
                    .filter(|e| {
                        e.is_file()
                            && e.extension().unwrap_or_default() == EXTENSION_FOR_ENVIRONMENT
                    })
                    .collect::<Vec<PathBuf>>()
            } else {
                vec![]
            };

            // project file content
            let project_root_content =
                fs::read_to_string(project_file).map_err(|e| e.to_string())?;

            // request file contents
            let request_root_contents: Vec<(PathBuf, String)> = request_files
                .iter()
                .map(|e| {
                    let content = fs::read_to_string(e).map_err(|e| e.to_string())?;
                    Ok((e.clone(), content))
                })
                .collect::<Result<Vec<(PathBuf, String)>, String>>()?;

            // environment file contents
            let environment_root_contents: Vec<(PathBuf, String)> = environment_files
                .iter()
                .map(|e| {
                    let content = fs::read_to_string(e).map_err(|e| e.to_string())?;
                    Ok((e.clone(), content))
                })
                .collect::<Result<Vec<(PathBuf, String)>, String>>()?;

            // load project schema
            let project_root: ProjectRootSchema =
                serde_yaml::from_str(&project_root_content).map_err(|e| e.to_string())?;

            // load request schemas
            let request_root_schemas: Vec<(PathBuf, RequestRootSchema)> = request_root_contents
                .iter()
                .map(|e| {
                    let request_root: RequestRootSchema =
                        serde_yaml::from_str(&e.1).map_err(|e| e.to_string())?;
                    Ok((e.0.clone(), request_root))
                })
                .collect::<Result<Vec<(PathBuf, RequestRootSchema)>, String>>()?;

            // load environment schemas
            let environment_root_schemas: Vec<(PathBuf, EnvironmentRootSchema)> =
                environment_root_contents
                    .iter()
                    .map(|e| {
                        let environment_root: EnvironmentRootSchema =
                            serde_yaml::from_str(&e.1).map_err(|e| e.to_string())?;
                        Ok((e.0.clone(), environment_root))
                    })
                    .collect::<Result<Vec<(PathBuf, EnvironmentRootSchema)>, String>>()?;

            // cast to session type
            return Ok(Session::from_fs_schema(
                path,
                recent_project.map(|e| e.active_environment).flatten(),
                project_root,
                request_root_schemas
                    .iter()
                    .map(|e| Session::cast_request_schema_to_session_type(e.0.clone(), e.1.clone()))
                    .collect(),
                environment_root_schemas
                    .iter()
                    .map(|e| {
                        Session::cast_environment_schema_to_session_type(e.0.clone(), e.1.clone())
                    })
                    .collect(),
            ));
        }

        #[cfg(target_arch = "wasm32")]
        {
            // File system operations not supported in browsers
            return Err("File system operations not supported in browsers".to_string());
        }
    }

    pub async fn load_from_fs_from_dialog() -> Result<Option<Self>, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let picker = AsyncFileDialog::new()
                .set_title("Pick project to open")
                .add_filter("Native Doctor Project", &[EXTENSION_FOR_PROJECT]);

            match picker.pick_file().await {
                Some(path) => {
                    return Ok(Some(Session::load_from_fs_from_path(
                        path.path().to_path_buf(),
                    )?));
                }
                None => {
                    return Ok(None);
                }
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            // File dialogs don't work in browsers
            return Ok(None);
        }
    }
}
