use std::path::PathBuf;

use components_lib::button::{Button, ButtonStyleVariant};
use dioxus::prelude::*;
use nativedoctor_core::{create_project_template, fs::FileObject};

use crate::components::WmDragArea;
use crate::PageScreen;
use components_lib::label::Label;
use components_lib::pane::{Pane, PaneStyleVariant};

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
        WmDragArea { class: "h-full",
            Pane {
                style: PaneStyleVariant::Darker,
                class: "flex items-center justify-center h-full",
                div { class: "flex flex-col gap-4",
                    Label { class: "text-3xl text-center", "Native Doctor" }
                    div { class: "flex gap-2",
                        Button { onclick: create_new_project,
                            Label { "Create project" }
                        }
                        Button {
                            Label { "Open project" }
                        }
                    }
                }
            }
        }
    };
}
