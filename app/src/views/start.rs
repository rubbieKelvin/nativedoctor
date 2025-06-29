use components_lib::button::Button;
use dioxus::prelude::*;

use crate::components::WmDragArea;
use crate::session::Session;
use crate::PageScreen;
use components_lib::label::Label;
use components_lib::pane::{Pane, PaneStyleVariant};

#[component]
pub fn StartScreenView() -> Element {
    let mut screen_state = use_context::<Signal<PageScreen>>();

    let create_new_project = move |_: Event<MouseData>| {
        screen_state.set(PageScreen::ProjectScreen(Session::template()));
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
