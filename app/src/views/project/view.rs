use std::path::PathBuf;

use dioxus::prelude::*;
use nativedoctor_core::schema::roots::ProjectRootSchema;

use crate::{
    components::WmDragArea,
    states::ProjectState,
    views::project::{panel, side},
};

#[component]
pub fn ProjectView(path: PathBuf) -> Element {
    let project_state = use_context_provider(|| ProjectState::new());

    // load project in scope
    {
        // clone upper scope
        let path = path.clone();
        let mut project = project_state.project.clone();

        use_effect(move || {
            tracing::info!("Loading project, {:?}", &path);

            let path = path.clone();

            // Asynchronously load up the project from file
            spawn(async move {
                let p = ProjectRootSchema::load(&path).await;
                match p {
                    Ok(p) => {
                        let mut project = project.write();
                        *project = Some(p);
                    }
                    // TODO: we need to let the user know an error showed up somehow.
                    Err(e) => tracing::error!("{e}"),
                };
            });
        })
    };

    return rsx! {
        div { class: "flex flex-col h-full",
            WmDragArea { class: " bg-gray-300 h-10 flex items-center", "{path.to_str().unwrap()}" }

            match &*project_state.project.read() {
                Some(_) => rsx! {
                    div { class: "flex-grow flex",
                        side::SideBar { }
                        // panel::RequestPanel {}
                    }
                },
                None => rsx! {
                    div { "loading sidebar" }
                },
            }
        }
    };
}
