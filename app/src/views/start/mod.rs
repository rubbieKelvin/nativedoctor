use std::path::PathBuf;

use dioxus::prelude::*;
use nativedoctor_core::{
    create_project_template, fs::FileObject, schema::roots::ProjectRootSchema,
};

use crate::PageScreen;

#[component]
pub fn StartScreenView() -> Element {
    let mut screen_state = use_context::<Signal<PageScreen>>();

    let create_new_project = move |_: Event<MouseData>| {
        let mut screen_write = screen_state.write();
        let (project, requests) = create_project_template("Untitled");
        *screen_write = PageScreen::ProjectScreen(
            FileObject::new(PathBuf::new(), project),
            requests
                .iter()
                .map(|r| FileObject::new(PathBuf::new(), r.clone()))
                .collect(),
        );
    };

    return rsx! {
        div{
            class: "flex items-center justify-center h-full",
            div {
                class: "flex flex-col gap-4",
                h1 {
                    class: "text-3xl text-center",
                    "Native Doctor"
                }
                div {
                    class: "flex gap-2",
                    button {
                        class: "flex-grow px-3 py-2 bg-gray-200 hover:bg-gray-300",
                        onclick: create_new_project,
                        "Create project",
                    }
                    button {
                        class: "flex-grow px-3 py-2 bg-gray-200 hover:bg-gray-300",
                        "Open project"
                    }
                }
            }
        }
    };
}
