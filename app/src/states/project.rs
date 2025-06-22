use std::path::PathBuf;

use dioxus::{
    hooks::use_context,
    signals::{Readable, Signal, Writable},
};
use nativedoctor_core::{
    fs::FileObject,
    schema::roots::{ProjectRootSchema, RequestRootSchema},
};

#[derive(Clone, PartialEq)]
pub struct ProjectState {
    pub selected_request: Signal<Option<uuid::Uuid>>,
    pub project: Signal<Option<FileObject<ProjectRootSchema>>>,
    pub requests: Signal<Vec<FileObject<RequestRootSchema>>>,
}

impl ProjectState {
    pub fn new() -> ProjectState {
        return ProjectState {
            selected_request: Signal::new(None),
            project: Signal::new(None),
            requests: Signal::new(vec![]),
        };
    }

    pub fn inject() -> ProjectState {
        return use_context::<ProjectState>();
    }

    pub fn add_new_request(&mut self) {
        tracing::info!("Adding new reqeust");
        self.requests.with_mut(|request| {
            request.push(FileObject::new(
                PathBuf::new(),
                RequestRootSchema {
                    url: String::new(),
                    method: "GET".to_string(),
                    ..Default::default()
                },
            ));
        });
    }

    pub fn get_selected_request(&self) -> Option<FileObject<RequestRootSchema>> {
        let selected_id = &*self.selected_request.read();
        let request = &*self.requests.read();

        return match selected_id {
            Some(id) => match request.iter().find(|r| r.id == *id) {
                Some(request) => Some(request.clone()),
                None => None,
            },
            None => None,
        };
    }

    pub fn update_request(&mut self, request: FileObject<RequestRootSchema>) {
        self.requests.with_mut(|requests| {
            let item = requests.iter_mut().find(|i| i.id == request.id);
            if let Some(item) = item {
                item.copy_from(request);
            } 
            
        })
    }
}
