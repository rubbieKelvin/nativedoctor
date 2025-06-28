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
                "bg-[#3d3d3d] border border-[#82857e] focus:border-[#ffffff]"
            }
            TextFieldStyleVariant::Ghost => "bg-transparent border border-transparent hover:bg-[#3d3d3d] focus:bg-[#3d3d3d]",
            TextFieldStyleVariant::Void => "bg-transparent border border-transparent",
        }
    }
}

/// Represents the different size variants for a text field.
#[derive(PartialEq, Clone, Default, strum::EnumIter)]
pub enum TextFieldSizeVariant {
    #[default]
    Default,
    Small,
    Tiny,
}

impl Variant for TextFieldSizeVariant {
    /// Returns the appropriate Tailwind CSS classes for the text field size.
    fn classes(&self) -> &'static str {
        match self {
            TextFieldSizeVariant::Default => "rounded-md px-2 py-1 text-base",
            TextFieldSizeVariant::Small => "rounded px-1 py-0.5 text-sm",
            TextFieldSizeVariant::Tiny => "rounded-sm p-0.5 text-xs",
        }
    }
}

/// A customizable text input component.
#[component]
pub fn TextField(
    value: Signal<String>,
    placeholder: Option<&'static str>,
    size: Option<TextFieldSizeVariant>,
    style: Option<TextFieldStyleVariant>,
    class: Option<&'static str>,
    oninput: Option<EventHandler<Event<FormData>>>,
    autocomplete: Option<bool>,
) -> Element {
    let style = style.unwrap_or_default();
    let size = size.unwrap_or_default();
    let placeholder = placeholder.unwrap_or("Enter text...");
    let autocomplete = autocomplete.unwrap_or(false);
    let class = format!(
        "{} {} {} focus:outline-none",
        class.unwrap_or_default(),
        style.classes(),
        size.classes()
    );

    rsx! {
        input {
            r#type: "text",
            class,
            value: "{value}",
            placeholder: placeholder,
            autocomplete: if autocomplete {None} else {"off"},
            spellcheck: if autocomplete {None} else {"false"},
            autocapitalize: if autocomplete {None} else {"off"},
            oninput: move |e| {
                value.set(e.value());
                if let Some(oninput_handler) = oninput {
                    oninput_handler.call(e);
                }
            },
        }
    }
}
