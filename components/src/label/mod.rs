use dioxus::prelude::*;

use crate::traits::Variant;

#[derive(Clone, PartialEq, strum::Display, strum::EnumIter, Default)]
pub enum LabelSizeVariant {
    Large,
    #[default]
    Default,
    Small,
    Tiny,
}

impl Variant for LabelSizeVariant {
    fn classes(&self) -> &'static str {
        return match self {
            LabelSizeVariant::Large => "text-xl",
            LabelSizeVariant::Default => "text-base",
            LabelSizeVariant::Small => "text-sm",
            LabelSizeVariant::Tiny => "text-xs",
        };
    }
}

#[derive(Clone, PartialEq, strum::Display, strum::EnumIter, Default)]
pub enum LabelStyleVariant {
    #[default]
    Default,
    Mild,
    Ghost,
    Success,
    Danger,
    Info,
    Warning,
    Debug
}

impl Variant for LabelStyleVariant {
    fn classes(&self) -> &'static str {
        return match self {
            LabelStyleVariant::Default => "text-[#ffffff]",
            LabelStyleVariant::Mild => "text-[#b4b4b4]",
            LabelStyleVariant::Ghost => "text-[#898989]",
            LabelStyleVariant::Success => "text-[#1e7069]",
            LabelStyleVariant::Danger => "text-[#f44560]",
            LabelStyleVariant::Info => "text-[#44d1df]",
            LabelStyleVariant::Warning => "text-[#f2c43d]",
            LabelStyleVariant::Debug => "text-[#f17c37]",
        };
    }
}

#[component]
pub fn Label(
    class: Option<&'static str>,
    style: Option<LabelStyleVariant>,
    size: Option<LabelSizeVariant>,
    children: Element,
) -> Element {
    let size = size.unwrap_or_default();
    let style = style.unwrap_or_default();
    let class = format!(
        "{} {} {}",
        class.unwrap_or(""),
        size.classes(),
        style.classes()
    );

    return rsx! {
        p {
            class,
            {children}
        }
    };
}
