use std::path::PathBuf;

use dioxus::{
    hooks::use_context,
    signals::{Signal, Writable},
};
use nativedoctor_core::{
    fs::FileObject,
    schema::roots::{ProjectRootSchema, RequestRootSchema},
};

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
}

#[derive(Clone, PartialEq)]
pub struct WritableRequest {
    pub id: uuid::Uuid,
    pub request: RequestRootSchema,
    pub path: PathBuf,
    pub name: String,
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
        };
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
        };
    }
}
