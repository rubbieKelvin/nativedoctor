use dioxus::prelude::*;
use dioxus_desktop::use_window;

use crate::{components::WmDragArea, states::ApplicationState};

#[component]
pub fn EmptyPage() -> Element {
    let window = use_window();
    let appstate = ApplicationState::inject();

    const BUTTON_CLASS: &'static str =
        "w-full px-2 py-1 text-center hover:bg-gray-200 bg-gray-100 rounded-md";

    return rsx! {
        WmDragArea{
            class: "h-full flex items-center justify-center",
            div {
                class: "",

                // title
                h1 {
                    class: "w-full font-medium text-3xl text-center",
                    "Native Doctor"
                }

                // menu
                div {
                    class: "flex flex-col gap-2",
                    button {
                        class: BUTTON_CLASS,
                        onclick: {
                            let appstate = appstate.clone();
                            move |_| {
                                let mut appstate = appstate.clone();
                                spawn(async move {
                                    appstate.open_project().await;
                                });
                            }
                        },
                        "Open"
                    }
                    button {
                        class: BUTTON_CLASS,
                        onclick: move |_| {
                            window.close();
                        },
                        "Quit"
                    }
                }
            }
        }
    };
}
