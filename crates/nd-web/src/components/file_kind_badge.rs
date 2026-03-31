//! Small label for [`nd_core::WorkspaceFileKind`].

use dioxus::prelude::*;
use nd_core::WorkspaceFileKind;

#[component]
pub fn FileKindBadge(kind: WorkspaceFileKind) -> Element {
    let (label, class) = match kind {
        WorkspaceFileKind::Request => ("request", "nd-badge nd-badge-request"),
        WorkspaceFileKind::Sequence => ("sequence", "nd-badge nd-badge-sequence"),
        WorkspaceFileKind::Unknown => ("other", "nd-badge nd-badge-unknown"),
    };
    rsx! {
        span { class: "{class}", "{label}" }
    }
}
