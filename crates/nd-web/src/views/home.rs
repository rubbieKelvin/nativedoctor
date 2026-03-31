//! Workspace dashboard: requests, sequences, and unrecognized files.

use dioxus::prelude::*;
use nd_core::WorkspaceFileKind;

use crate::components::{RequestRow, SequenceRow, UnknownRow};
use crate::server::{list_catalog, CatalogEntry};

#[component]
pub fn Home() -> Element {
    let catalog = use_server_future(|| async move { list_catalog().await })?;

    rsx! {
        main { class: "nd-main",
            h1 { "Workspace" }
            p { class: "nd-lead",
                "Top-level request and sequence files in the configured directory (non-recursive). Runs use the same runtime layering as the CLI (`--env`, `--no-default-system-env`, cwd persist)."
            }
            {match catalog().as_ref() {
                None => rsx! { p { "Loading" } },
                Some(Err(e)) => rsx! { p { class: "nd-err", "Error: {e}" } },
                Some(Ok(entries)) if entries.is_empty() => rsx! { p { "No matching files found." } },
                Some(Ok(entries)) => rsx! {
                    WorkspaceSections { entries: entries.clone() }
                },
            }}
        }
    }
}

#[component]
fn WorkspaceSections(entries: Vec<CatalogEntry>) -> Element {
    let requests: Vec<_> = entries
        .iter()
        .filter(|e| e.kind == WorkspaceFileKind::Request)
        .cloned()
        .collect();
    let sequences: Vec<_> = entries
        .iter()
        .filter(|e| e.kind == WorkspaceFileKind::Sequence)
        .cloned()
        .collect();
    let unknown: Vec<_> = entries
        .iter()
        .filter(|e| e.kind == WorkspaceFileKind::Unknown)
        .cloned()
        .collect();

    rsx! {
        section { class: "nd-section",
            h2 { "Requests" }
            if requests.is_empty() {
                p { class: "nd-muted", "No request files." }
            } else {
                ul { class: "nd-list",
                    for e in requests {
                        RequestRow { name: e.name.clone() }
                    }
                }
            }
        }
        section { class: "nd-section",
            h2 { "Sequences" }
            if sequences.is_empty() {
                p { class: "nd-muted", "No sequence files." }
            } else {
                ul { class: "nd-list",
                    for e in sequences {
                        SequenceRow { name: e.name.clone() }
                    }
                }
            }
        }
        section { class: "nd-section",
            h2 { "Unrecognized" }
            if unknown.is_empty() {
                p { class: "nd-muted", "None." }
            } else {
                ul { class: "nd-list",
                    for e in unknown {
                        UnknownRow { name: e.name.clone() }
                    }
                }
            }
        }
    }
}
