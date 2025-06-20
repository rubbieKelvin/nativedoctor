use std::path::PathBuf;

use dioxus::prelude::*;
use nativedoctor_core::{
    fs::FileObject,
    schema::roots::{ProjectRootSchema, RequestRootSchema},
};

use crate::{
    components::WmDragArea,
    views::project::{self, panel, side},
};

#[component]
pub fn ProjectView(path: PathBuf) -> Element {
    let project: Signal<Option<FileObject<ProjectRootSchema>>> =
        use_context_provider(|| Signal::new(None));
    let requests: Signal<Vec<FileObject<RequestRootSchema>>> =
        use_context_provider(|| Signal::new(vec![]));

    // load project in scope
    {
        let path = path.clone();
        let mut project = project.clone();

        use_effect(move || {
            tracing::info!("Loading project, {:?}", &path);

            let path = path.clone();
            spawn(async move {
                let p = ProjectRootSchema::load(&path).await;
                match p {
                    Ok(p) => {
                        let mut project = project.write();
                        *project = Some(p);
                    }
                    Err(e) => tracing::error!("{e}"),
                };
            });
        })
    };

    return rsx! {
        div { class: "flex flex-col h-full",
            WmDragArea { class: " bg-gray-300 h-10 flex items-center", "{path.to_str().unwrap()}" }

            match project() {
                Some(_) => rsx! {
                    div { class: "flex-grow flex",
                        side::SideBar { }
                        panel::RequestPanel {}
                    }
                },
                None => rsx! {
                    div { "loading sidebar" }
                },
            }
        }
    };
}
