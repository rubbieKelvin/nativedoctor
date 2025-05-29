use super::dialog;
use dioxus::prelude::*;

#[component]
pub fn EnvEditDialog(open: Signal<bool>) -> Element {
    return rsx! {
        dialog::Dialog {
            show: open,
            title: "Edit Environment".to_string(),
            content: rsx! {
                div {
                    "Edit Environment"
                }
            }
        }
    };
}
