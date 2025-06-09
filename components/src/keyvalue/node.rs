use super::types::{KeyValuePairData, KeyValuePairObject};
use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons};
use rfd::AsyncFileDialog;
use std::path::PathBuf;

#[component]
pub fn KeyValueNode(
    // The current state of this pair
    pair: KeyValuePairObject,
    // Event handler for when the pair's data changes
    on_change: EventHandler<KeyValuePairObject>,
    // Event handler for when this pair should be removed (emits its ID)
    on_remove: EventHandler<String>,
    // Event handler for when the user presses ctrl+enter
    on_newline: EventHandler<()>,
    // Prop to allow showing the file browse option
    allow_file_values: bool,
) -> Element {
    // Access the key and value from the pair
    let (key, _value) = match &pair.data {
        KeyValuePairData::String(k, v) => (k, v),
        KeyValuePairData::File(k, v) => (k, v),
    };

    // Signal to store the selected file path temporarily (for file picker)
    let mut file_path_signal = use_signal(|| None::<PathBuf>);

    // Clone pair for use in the resource
    let pair_clone = pair.clone();

    let cb_pair_clone = pair.clone();
    let ki_pair_clone = pair.clone();
    let cvi_pair_clone = pair.clone();
    let tbs_pair_clone = pair.clone();

    let request_new_line = move |e: Event<KeyboardData>| {
        let modifiers = e.modifiers();
        if e.key() == Key::Enter
            && (modifiers.contains(Modifiers::META) || modifiers.contains(Modifiers::CONTROL))
        {
            on_newline.call(());
        }
    };

    rsx! {
        div {
            class: "flex items-center gap-2 p-1 group/node",

            input {
                type: "checkbox",
                checked: pair.enabled,
                oninput: move |e| {
                    let pair = cb_pair_clone.clone();
                    let new_enabled = e.value() == "true";
                    on_change.call(KeyValuePairObject {
                        id: pair.id.clone(),
                        enabled: new_enabled,
                        data: pair.data,
                    });
                }
            }

            // Key Input
            input {
                class: "w-[35%] px-2 py-1 border border-gray-300 rounded focus:outline-none",
                placeholder: "Key",
                value: "{key}",
                onkeydown: move |e| request_new_line.clone()(e),
                oninput: move |e| {
                    let pair = ki_pair_clone.clone();
                    // Create a new KeyValuePairObject with the updated key
                    let new_data = match &pair.data {
                        KeyValuePairData::String(_, val) => KeyValuePairData::String(e.value(), val.clone()),
                        KeyValuePairData::File(_, val) => KeyValuePairData::File(e.value(), val.clone()),
                    };

                    on_change.call(KeyValuePairObject {
                        id: pair.id.clone(),
                        enabled: pair.enabled,
                        data: new_data,
                    });
                }
            }

            // Conditional Value Input (String or File)
            match &pair.data {
                KeyValuePairData::String(_, val) => rsx! {
                    input {
                        class: "flex-grow px-2 py-1 border border-gray-300 rounded focus:outline-none",
                        placeholder: "Value",
                        value: "{val}",
                        onkeydown: move |e| request_new_line.clone()(e),
                        oninput: move |e| {
                            let pair = cvi_pair_clone.clone();
                            let new_data = KeyValuePairData::String(pair.key().to_string(), e.value());
                            on_change.call(KeyValuePairObject {
                                id: pair.id.clone(),
                                enabled: pair.enabled,
                                data: new_data,
                            });
                        }
                    }
                },
                KeyValuePairData::File(_, val) => rsx! {
                    div {
                        class: "flex flex-grow items-center gap-1",
                        input {
                            class: "flex-grow px-2 py-1 border border-gray-300 rounded bg-gray-100",
                            placeholder: "File Path",
                            value: "{val}",
                            readonly: true,
                            onkeydown: move |e| request_new_line.clone()(e),
                        }
                        button {
                            class: "px-3 py-1 bg-blue-500 text-white rounded hover:bg-blue-600 focus:outline-none",
                            onclick: move |_| {
                                // pick_file_resource.restart(); // Trigger the file picker
                            },
                            "Browse"
                        }
                    }
                }
            }

            // Optional: Toggle between String and File if `allow_file_values`
            if allow_file_values {
                button {
                    class: "px-2 py-1 text-xs text-gray-600 hover:text-gray-900",
                    onclick: move |_| {
                        let pair = tbs_pair_clone.clone();
                        // Toggle between String and File type
                        let new_data = match &pair.data {
                            KeyValuePairData::String(k, v) => KeyValuePairData::File(k.clone(), v.clone()),
                            KeyValuePairData::File(k, v) => KeyValuePairData::String(k.clone(), v.clone()),
                        };
                        on_change.call(KeyValuePairObject {
                            id: pair.id.clone(),
                            enabled: pair.enabled,
                            data: new_data,
                        });
                    },
                    match &pair.data {
                        KeyValuePairData::String(_,_) => "File",
                        KeyValuePairData::File(_,_) => "String",
                    }
                }
            }


            // Remove Button
            button {
                class: "p-1 rounded hover:bg-red-600 focus:outline-none group-hover/node:block hidden",
                onclick: move |_| {
                    on_remove.call(pair.id.clone());
                },
                Icon {
                    icon: ld_icons::LdTrash,
                    width: 14,
                    height: 14,
                }
            }
        }
    }
}
