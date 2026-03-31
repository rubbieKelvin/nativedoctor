//! Workspace dashboard: requests, sequences, and unrecognized files.

use dioxus::prelude::*;

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
    rsx! {}
}
