use dioxus::prelude::*;

use crate::states::{ToastConfig, ToastState};

#[component]
pub fn ToastProvider() -> Element {
    let toast_state = ToastState::inject();

    return rsx! {
        div {
            class: "w-96 p-2 fixed right-0 bottom-0 z-50",
            for item in toast_state.items() {
                ToastItem {key: item.id, toast: item}
            }
        }
    };
}

#[component]
pub fn ToastItem(toast: ToastConfig) -> Element {
    let mut toast_state = ToastState::inject();
    let title: String = toast.title.into();

    return rsx! {
        div {
            class: "p-4 bg-gray-100 border border-gray-200 rounded-md flex-col",
            div {
                class: "flex gap-4",
                span {
                    class: "flex-grow",
                    "{title}"
                }

                button {
                    class: "p-1 bg-gray-400 hover:bg-gray-500",
                    onclick: move |_| {
                        toast_state.remove(toast.id);
                    },
                    "X"
                }
            }

            if let Some(summary) = toast.summary {
                div {
                    "{summary}"
                }
            }
        }
    };
}
