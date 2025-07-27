use dioxus::prelude::*;

#[component]
pub fn CallPage() -> Element {
    return rsx! {
        div { class: "h-full flex flex-col gap-4 pt-2",
            "My call"
        }
    };
}
