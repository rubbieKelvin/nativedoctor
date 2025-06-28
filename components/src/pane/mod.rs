use dioxus::prelude::*;

use crate::traits::Variant;

#[derive(PartialEq, Clone, strum::EnumIter, strum::Display)]
pub enum PaneStyleVariant {
    Lighter,
    Default,
    Dark,
    Evil,
}

impl Variant for PaneStyleVariant {
    fn classes(&self) -> &'static str {
        return match self {
            PaneStyleVariant::Lighter => "bg-[#202020]",
            PaneStyleVariant::Default => "bg-[#1b1b1b]",
            PaneStyleVariant::Dark => "bg-[#141414]",
            PaneStyleVariant::Evil => "bg-[#0a0a0a]",
        };
    }
}

#[component]
pub fn Pane(class: Option<&'static str>, style: Option<PaneStyleVariant>, children: Element) -> Element {
    let style = style.unwrap_or(PaneStyleVariant::Default);
    let class = format!("{} {}", class.unwrap_or(""), style.classes());

    return rsx!{
        div {
            class,
            {children}
        }
    };
}