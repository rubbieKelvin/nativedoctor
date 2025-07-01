use components_lib::tabs::TabState;
use dioxus::prelude::*;

use crate::views::project::WorkspaceTab;

#[component]
pub fn RequestPage() -> Element {
    let state = use_context::<TabState<WorkspaceTab>>();

    return rsx! {
        div { class: "h-full flex items-center justify-center" }
    };
}
