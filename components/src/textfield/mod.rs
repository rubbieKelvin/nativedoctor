use dioxus::prelude::*;

use crate::traits::Variant;

/// Represents the different style variants for a text field.
#[derive(PartialEq, Clone, Default, strum::EnumIter)]
pub enum TextFieldStyleVariant {
    #[default]
    Default,
    Ghost,
    Void
}

impl Variant for TextFieldStyleVariant {
    fn classes(&self) -> &'static str {
        match self {
            TextFieldStyleVariant::Default => {
                "bg-[#3d3d3d] border border-[#82857e] focus-within:border-[#ffffff]"
            }
            TextFieldStyleVariant::Ghost => "bg-transparent border border-transparent hover:bg-[#3d3d3d] focus-within:bg-[#3d3d3d]",
            TextFieldStyleVariant::Void => "bg-transparent border border-transparent",
        }
    }
}

/// Represents the different size variants for a text field.
#[derive(PartialEq, Clone, Default, strum::EnumIter)]
pub enum TextFieldSizeVariant {
    Large,
    #[default]
    Default,
    Small,
    Tiny,
}

impl Variant for TextFieldSizeVariant {
    /// Returns the appropriate Tailwind CSS classes for the text field size.
    fn classes(&self) -> &'static str {
        match self {
            TextFieldSizeVariant::Large => "rounded-md px-2 py-1 text-lg",
            TextFieldSizeVariant::Default => "rounded-md px-2 py-1 text-base",
            TextFieldSizeVariant::Small => "rounded px-1 py-0.5 text-sm",
            TextFieldSizeVariant::Tiny => "rounded-sm p-0.5 text-xs",
        }
    }
}

/// A customizable text input component.
#[component]
pub fn TextField(
    value: Option<String>,
    placeholder: Option<String>,
    size: Option<TextFieldSizeVariant>,
    style: Option<TextFieldStyleVariant>,
    class: Option<String>,
    oninput: Option<EventHandler<Event<FormData>>>,
    before: Option<Element>,
    after: Option<Element>,
    autocomplete: Option<bool>,
    onblur: Option<EventHandler<Event<FocusData>>>,
    onfocus: Option<EventHandler<Event<FocusData>>>,
    onkeydown: Option<EventHandler<Event<KeyboardData>>>,
    onreturn: Option<EventHandler<Event<KeyboardData>>>,
) -> Element {
    let value = value.unwrap_or_default();
    let style = style.unwrap_or_default();
    let size = size.unwrap_or_default();
    let placeholder = placeholder.unwrap_or("Enter text...".to_string());
    let autocomplete = autocomplete.unwrap_or(false);
    let class = format!(
        "{} {} {} flex gap-2",
        class.unwrap_or_default(),
        style.classes(),
        size.classes()
    );

    rsx! {
        div {
            class,
            
            if let Some(before) = before {
                {before}
            }

            input {
                r#type: "text",
                class: "focus:outline-none bg-transparent flex-grow",
                value: "{value}",
                placeholder: placeholder,
                autocomplete: if autocomplete {None} else {"off"},
                spellcheck: if autocomplete {None} else {"false"},
                autocapitalize: if autocomplete {None} else {"off"},
                onfocus: move |e| {
                    if let Some(onfocus_handler) = onfocus {
                        onfocus_handler.call(e);
                    }
                },
                onblur: move |e| {
                    if let Some(onblur_handler) = onblur {
                        onblur_handler.call(e);
                    }
                },
                oninput: move |e| {
                    if let Some(oninput_handler) = oninput {
                        oninput_handler.call(e);
                    }
                },
                onkeydown: move |e| {
                    if e.key() == Key::Enter && let Some(onreturn_handler) = onreturn {
                        onreturn_handler.call(e.clone());
                    }
                    if let Some(onkeydown_handler) = onkeydown {
                        onkeydown_handler.call(e);
                    }
                },
            }

            if let Some(after) = after {
                {after}
            }
        }
    }
}
