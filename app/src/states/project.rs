use std::path::PathBuf;

use dioxus::{
    hooks::use_context,
    signals::{Signal, Writable},
};
use nativedoctor_core::{
    fs::FileObject,
    schema::roots::{ProjectRootSchema, RequestRootSchema},
    EXTENSION_FOR_PROJECT, EXTENSION_FOR_REQUEST,
};
use rfd::AsyncFileDialog;

#[derive(Clone, PartialEq)]
pub struct ProjectState {
    pub open_request: Signal<Option<WritableRequest>>,
    pub project: Signal<FileObject<ProjectRootSchema>>,
    pub requests: Signal<Vec<WritableRequest>>,
}

impl ProjectState {
    pub fn new(
        projects: FileObject<ProjectRootSchema>,
        requests: Vec<FileObject<RequestRootSchema>>,
    ) -> ProjectState {
        return ProjectState {
            open_request: Signal::new(None),
            project: Signal::new(projects),
            requests: Signal::new(
                requests
                    .iter()
                    .map(|r| WritableRequest::from(r.clone()))
                    .collect(),
            ),
        };
    }

    pub fn inject() -> ProjectState {
        return use_context::<ProjectState>();
    }

    pub fn add_new_request(&mut self) {
        tracing::info!("Adding new reqeust");
        self.requests.with_mut(|request| {
            request.push(WritableRequest::new());
        });
    }

    /// Save the project to disk.
    async fn save(&self) -> Result<(), String> {
        tracing::info!("Starting save project");

        let mut project_signal = self.project.clone();
        let mut request_signal = self.requests.clone();

        let mut project = project_signal.write();

        // pick project folder name
        let name = project.object.project.name.clone();
        let name = name.trim();
        let name = if name.len() == 0 {
            "untitled_project"
        } else {
            name
        };

        // get base path
        let path = if project.path.try_exists().map_err(|e| {
            tracing::error!("{}", e);
            format!("Error checking path status: {}", e.to_string())
        })? {
            project.path.clone()
        } else {
            // open the filemanager to save
            let picker = AsyncFileDialog::new().set_title("Pick folder to save project");
            let path = match picker.pick_folder().await {
                Some(handler) => {
                    let path = handler.path();
                    let root = path.join(name);

                    // create the folder
                    tokio::fs::create_dir(&root).await.map_err(|e| {
                        tracing::error!("{}", e);
                        format!("Error creating directory: {}", e.to_string())
                    })?;
                    root.join(format!("nativedoctor.{}", EXTENSION_FOR_PROJECT))
                }
                None => return Err("Could not pick folder".to_string()),
            };
            path
        };

        // save project file
        project.path = path;
        tracing::info!("Writing project to path: {:?}", &project.path);

        project
            .save()
            .await
            .map_err(|e| format!("Erorr saving file: {e}"))?;

        // REQUESTS
        // create the requests directory if it doesn't exist
        let requests_dir = project.get_requests_dir();
        if !requests_dir.try_exists().map_err(|e| {
            tracing::error!("{}", e);
            format!(
                "Error checking requests directory status: {}",
                e.to_string()
            )
        })? {
            tokio::fs::create_dir(&requests_dir).await.map_err(|e| {
                tracing::error!("{}", e);
                format!("Error creating requests directory: {}", e.to_string())
            })?;
        }

        let mut requests_mut = request_signal.write();

        for request in requests_mut.iter_mut() {
            let name = request.name.trim();

            if name.len() == 0 {
                tracing::info!("Skipped request {}, as request has no name", request.id);
                continue;
            }

            let name = request.name.to_ascii_lowercase();
            let virtual_path = requests_dir.join(format!("{}.{}", name, EXTENSION_FOR_REQUEST));

            // check if the file exists, if it does, and it's not the same as the virtual path, then delete the file
            if request
                .path
                .try_exists()
                .map_err(|e| format!("Error checking request path status: {}", e))?
            {
                // if the file exists, and it's not the same as the virtual path, then delete the file
                if request.path != virtual_path {
                    tokio::fs::remove_file(&request.path).await.map_err(|e| {
                        tracing::error!("{}", e);
                        format!("Error deleting request file: {}", e.to_string())
                    })?;
                }
            }

            // now create the file
            request.path = virtual_path.clone();
            let file_object: FileObject<RequestRootSchema> = request.into();
            file_object.save().await.map_err(|e| {
                tracing::error!("{}", e);
                format!("Error saving request file: {}", e.to_string())
            })?;
        }

        return Ok(());
    }
}

pub enum WritableRequestProperty {
    Request(RequestRootSchema),
    Name(String),
    Path(PathBuf),
}

#[derive(Clone, PartialEq)]
pub struct WritableRequest {
    pub id: uuid::Uuid,
    pub request: RequestRootSchema,
    pub path: PathBuf,
    pub name: String,
    pub changes_made: bool,
}

impl WritableRequest {
    pub fn new() -> Self {
        return WritableRequest {
            id: uuid::Uuid::new_v4(),
            request: RequestRootSchema {
                url: String::new(),
                method: "GET".to_string(),
                ..Default::default()
            },
            name: "untitled".to_string(),
            path: PathBuf::new(),
            changes_made: true,
        };
    }

    pub fn set(&mut self, prop: WritableRequestProperty) {
        match prop {
            WritableRequestProperty::Name(name) => {
                if self.name == name {
                    return;
                }
                self.name = name;
            }
            WritableRequestProperty::Path(path) => {
                if self.path == path {
                    return;
                }
                self.path = path;
            }
            WritableRequestProperty::Request(request) => {
                if self.request == request {
                    return;
                }
                self.request = request;
            }
        }
        self.changes_made = true;
    }
}

impl Into<FileObject<RequestRootSchema>> for WritableRequest {
    fn into(self) -> FileObject<RequestRootSchema> {
        return FileObject {
            id: self.id,
            path: self.path,
            object: self.request,
        };
    }
}

impl Into<FileObject<RequestRootSchema>> for &mut WritableRequest {
    fn into(self) -> FileObject<RequestRootSchema> {
        return FileObject {
            id: self.id,
            path: self.path.clone(),
            object: self.request.clone(),
        };
    }
}

impl From<FileObject<RequestRootSchema>> for WritableRequest {
    fn from(value: FileObject<RequestRootSchema>) -> Self {
        let name = match value.path.file_stem() {
            Some(stem) => stem
                .to_str()
                .map(|s| s.to_string())
                .unwrap_or_else(|| "untitled".to_string()),
            None => "untitled".to_string(),
        };

        return WritableRequest {
            id: value.id,
            request: value.object,
            path: value.path,
            name,
            changes_made: false,
        };
    }
}
