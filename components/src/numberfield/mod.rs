use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons, Icon};

use crate::traits::Variant;

#[derive(Default, PartialEq, Clone, strum::EnumIter)]
pub enum NumberFieldStyleVariant {
    #[default]
    Default,
    Outline,
    Ghost
}

impl Variant for NumberFieldStyleVariant  {
    fn classes(&self) -> &'static str {
        return match self  {
            NumberFieldStyleVariant::Default => "bg-[#3d3d3d]",
            NumberFieldStyleVariant::Outline => "border-[#82857e] border",
            NumberFieldStyleVariant::Ghost => "",
        };
    }
}

#[derive(Default, PartialEq, Clone, strum::EnumIter)]
pub enum NumberFieldSizeVariant {
    #[default]
    Default,
    Small,
    Tiny,
}

impl Variant for NumberFieldSizeVariant {
    fn classes(&self) -> &'static str {
        return match self {
            NumberFieldSizeVariant::Default => "text-base rounded-md",
            NumberFieldSizeVariant::Small => "text-sm rounded",
            NumberFieldSizeVariant::Tiny => "text-xs rounded-sm",
        };
    }
}

#[component]
pub fn NumberField(
    value: Option<i32>,
    step: Option<u32>,
    min: Option<i32>,
    max: Option<i32>,
    class: Option<&'static str>,
    size: Option<NumberFieldSizeVariant>,
    style: Option<NumberFieldStyleVariant>,
    onchange: Option<EventHandler<i32>>,
) -> Element {
    let value = value.unwrap_or_default();
    let step = step.unwrap_or(1) as i32;
    let style = style.unwrap_or_default();
    let size = size.unwrap_or_default();
    let class = format!("{} {} {}", class.unwrap_or_default(), style.classes(), size.classes());

    let is_dec_disabled = if min.is_some() {
        value <= min.unwrap()
    } else {
        false
    };
    let is_inc_disabled = if max.is_some() {
        value >= max.unwrap()
    } else {
        false
    };
    let id = use_hook(|| uuid::Uuid::new_v4());

    let mut input_visible = use_signal(|| false);
    let mut input_value = use_signal(|| String::new());

    use_effect(move || {
        if !input_visible() {
            input_value.set(String::new());
        }
    });

    use_effect(move || {
        if input_visible() {
            let js = format!(
                r#"
                    let input = document.getElementById("number-input-{id}");
                    if (input) {{
                        input.focus();
                        input.select();
                    }}
                "#
            );
            // Execute the JavaScript
            document::eval(&js);
        }
    });

    let icon_size = match size {
        NumberFieldSizeVariant::Default => 20,
        NumberFieldSizeVariant::Small => 18,
        NumberFieldSizeVariant::Tiny => 16,
    };

    let button_classes = match style {
        NumberFieldStyleVariant::Default => "bg-[#2e2e2e] active:bg-[#3c3c3c]",
        NumberFieldStyleVariant::Outline => "hover:bg-[#3d3d3d] active:bg-[#2e2e2e]",
        NumberFieldStyleVariant::Ghost => "",
    };
    let button_class_from_size = match size {
        NumberFieldSizeVariant::Default => "px-2",
        NumberFieldSizeVariant::Small => "px-1",
        NumberFieldSizeVariant::Tiny => "px-0.5",
    };
    let button_classes = format!("{} {} h-full", button_classes, button_class_from_size);


    let set_value = move |new_val| {
        let is_valid = (max.is_none() || new_val <= max.unwrap())
            && (min.is_none() || new_val >= min.unwrap());

        if is_valid {
            // If the on_change handler exists, call it with the new value
            if let Some(handler) = &onchange {
                handler.call(new_val);
            }
        }
    };

    let submit_input = move || {
        if let Ok(v) = input_value().parse::<i32>() {
            set_value.clone()(v);
        }
        input_visible.set(false);
    };

    return rsx! {
        style {
            "input[type=number]::-webkit-inner-spin-button, input[type=number]::-webkit-outer-spin-button {{
                    -webkit-appearance: none;
                    margin: 0;
            }}"
        }

        // Main container with flex layout to align items
        div {
            class: "flex items-center gap-2 overflow-clip {class}",
            // Decrement button
            button {
                class: "{button_classes}",
                disabled: is_dec_disabled,
                onclick: move |_| {
                    let new_val = value - step;
                    set_value.clone()(new_val);
                },
                Icon{
                    width: icon_size,
                    height: icon_size,
                    icon: ld_icons::LdChevronLeft
                }
            }

            // The number display in the middle
            if input_visible() {
                input {
                    id: "number-input-{id}",
                    class: "flex-grow appearance-none focus:outline-0 w-0 h-full",
                    r#type: "number",
                    value: "{value}",
                    autocomplete: "off",
                    spellcheck: "false",
                    autocapitalize: "off",
                    onkeypress: move |e| {
                        if e.key() != Key::Enter {
                            return;
                        }
                        submit_input.clone()();
                    },
                    onblur: move |_| {
                        submit_input.clone()();
                    },
                    oninput: move |e| {
                        let v = e.value();
                        input_value.set(v);
                    },
                }
            } else {
                p {
                    class: "flex-grow text-center h-full w-0",
                    onclick: move |_| {
                        input_visible.set(true);
                    },
                    "{value}"
                }
            }

            // // Increment button
            button {
                class: "{button_classes}",
                disabled: is_inc_disabled,
                onclick: move |_| {
                    let new_val = value + step;
                    set_value.clone()(new_val);
                },
                Icon{
                    width: icon_size,
                    height: icon_size,
                    icon: ld_icons::LdChevronRight
                }
            }
        }
    };
}
