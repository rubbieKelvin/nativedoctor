// ui/key_value_editor.rs
use dioxus::prelude::*;

use super::node::KeyValueNode;
use super::types::KeyValuePairObject; // Make sure this path is correct

#[component]
pub fn KeyValueEditor(allow_multikeys: Option<bool>, allow_file_values: Option<bool>) -> Element {
    // Unpack options with defaults
    let allow_file_values = allow_file_values.unwrap_or(false);

    // Signal to store the list of key-value pairs
    let mut keyvalue_pairs: Signal<Vec<KeyValuePairObject>> = use_signal(|| {
        vec![
            // Start with one empty string pair by default
            KeyValuePairObject::new_string(),
        ]
    });

    // Handler for when a KeyValueNode emits a change
    let on_kv_change = move |updated_pair: KeyValuePairObject| {
        keyvalue_pairs.with_mut(|pairs| {
            if let Some(index) = pairs.iter().position(|p| p.id == updated_pair.id) {
                pairs[index] = updated_pair;
            }
        });
    };

    // Handler for when a KeyValueNode emits a remove event
    let on_kv_remove = move |id_to_remove: String| {
        keyvalue_pairs.with_mut(|pairs| {
            pairs.retain(|p| p.id != id_to_remove);
        });
    };

    rsx! {
        div {

            // Render each KeyValueNode
            div {
                class: "flex flex-col",
                for pair in keyvalue_pairs.read().iter() {
                    // KeyValueNode needs owned data, so clone the pair for the loop.
                    // The on_change and on_remove handlers capture references,
                    // but EventHandler::call needs owned data for the argument.
                    KeyValueNode {
                        pair: pair.clone(), // Clone the pair for each node
                        on_change: on_kv_change.clone(), // Clone the handler
                        on_remove: on_kv_remove.clone(), // Clone the handler
                        on_newline: move |_| {
                            keyvalue_pairs.with_mut(|pairs| {
                                pairs.push(KeyValuePairObject::new_string());
                            });
                        },
                        allow_file_values: allow_file_values,
                    }
                }
            }
        }
    }
}
