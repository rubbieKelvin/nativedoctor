use dioxus::prelude::*;
use dioxus_desktop::use_window;

use crate::{
    components::{Dialog, WmDragArea},
    states::{
        ApplicationState, ProjectContentLoadingStatus, ToastCloseMethod, ToastConfig, ToastState,
        ToastTitle,
    },
};

const BUTTON_CLASS: &'static str =
    "w-full px-2 py-1 text-center hover:bg-gray-200 bg-gray-100 rounded-md";

#[component]
fn ProjectNameInputDialog(show: Signal<bool>) -> Element {
    let appstate = use_context::<ApplicationState>();
    let mut project_name = use_signal(|| String::new());

    // if the dialog is opened, focus to editor, else clear editor
    use_effect(move || {
        let show_state = show();
        if show_state {
            document::eval("document.getElementById('project-name-input').focus()");
        } else {
            *project_name.write() = String::new();
        }
    });

    rsx! {
        Dialog {
            show,
            div {
                class: "p-4 bg-white flex flex-col gap-2",
                h1 {
                    "Create project"
                }
                input {
                    id: "project-name-input",
                    value: "{project_name}",
                    autocomplete: "off",
                    autocapitalize: "off",
                    spellcheck: false,
                    placeholder: "Enter project name",
                    oninput: move |e| {
                        let text = e.value();
                        *project_name.write() = text;
                    }
                }
                button {
                    class: BUTTON_CLASS,
                    onclick: {
                        let appstate = appstate.clone();
                        move |_| {
                            let mut appstate = appstate.clone();
                            spawn(async move {
                                appstate.create_project_with_picker(&*project_name.read()).await;
                            });
                        }
                    },
                    "Create"

                }
            }
        }
    }
}

#[component]
pub fn EmptyPage() -> Element {
    let window = use_window();
    let mut toast = ToastState::inject();
    let appstate = ApplicationState::inject();
    let mut open_project = use_signal(|| false);

    
    // watch for creation errors
    use_effect(move || {
        let project_status = appstate.project.read();
        match &*project_status {
            ProjectContentLoadingStatus::Error(e) => {
                toast.push(ToastConfig::new(
                    ToastTitle::Error("An error occured".to_string()),
                    Some(e.to_string()),
                    ToastCloseMethod::Button,
                ));
            }
            _ => {}
        };
    });

    return rsx! {
        ProjectNameInputDialog {
            show: open_project
        },
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
                        onclick: move |_| {
                            *open_project.write() = true;
                        },
                        "Create"
                    }
                    button {
                        class: BUTTON_CLASS,
                        onclick: {
                            let appstate = appstate.clone();
                            move |_| {
                                let mut appstate = appstate.clone();
                                spawn(async move {
                                    appstate.open_project_with_picker().await;
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
