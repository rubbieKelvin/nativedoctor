//! Non-request / non-sequence file (listed for visibility; no Run).

use dioxus::prelude::*;

use crate::components::FileKindBadge;
use nd_core::WorkspaceFileKind;

#[component]
pub fn UnknownRow(name: String) -> Element {
    rsx! {
        li { class: "nd-row nd-row-muted",
            code { "{name}" }
            FileKindBadge { kind: WorkspaceFileKind::Unknown }
            span { class: "nd-hint", "Not recognized as a request or sequence document." }
        }
    }
}
