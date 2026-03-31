//! Single sequence file row with Run.

use dioxus::prelude::*;

use crate::server::run_sequence_file;

#[component]
pub fn SequenceRow(name: String) -> Element {
    let mut output = use_signal(String::new);
    let mut pending = use_signal(|| false);
    let name_clone = name.clone();

    let run = move |_| {
        let n = name_clone.clone();
        spawn(async move {
            pending.set(true);
            match run_sequence_file(n).await {
                Ok(s) => output.set(s),
                Err(e) => output.set(format!("Error: {e}")),
            }
            pending.set(false);
        });
    };

    rsx! {
        li { class: "nd-row",
            code { "{name}" }
            button {
                class: "nd-run",
                disabled: *pending.read(),
                onclick: run,
                if *pending.read() { "Running…" } else { "Run" }
            }
        }
        if !output.read().is_empty() {
            pre { class: "nd-output", "{output}" }
        }
    }
}
