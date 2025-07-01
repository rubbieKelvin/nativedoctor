use super::traits::Variant;
use crate::border::Border;
use dioxus::prelude::*;

#[derive(PartialEq, Clone, strum::EnumIter, strum::Display)]
pub enum PaneStyleVariant {
    Transparent,
    Lighter,
    Default,
    Dark,
    Darker,
    Evil,
}

impl Variant for PaneStyleVariant {
    fn classes(&self) -> &'static str {
        return match self {
            PaneStyleVariant::Transparent => "bg-transparent",
            PaneStyleVariant::Lighter => "bg-[#202020]",
            PaneStyleVariant::Default => "bg-[#1b1b1b]",
            PaneStyleVariant::Dark => "bg-[#141414]",
            PaneStyleVariant::Darker => "bg-[#101010]",
            PaneStyleVariant::Evil => "bg-[#0a0a0a]",
        };
    }
}

#[component]
pub fn Pane(
    class: Option<String>,
    style: Option<PaneStyleVariant>,
    border: Option<Border>,
    tabindex: Option<i32>,
    children: Element,
    role: Option<String>,
    aria_orientation: Option<String>,
    aria_labelledby: Option<String>,
    onclick: Option<EventHandler<Event<MouseData>>>
) -> Element {
    let border = border.unwrap_or_else(|| Border::none());
    let style = style.unwrap_or(PaneStyleVariant::Default);
    let class = format!(
        "{} {} {}",
        class.unwrap_or("".to_string()),
        style.classes(),
        border.classes()
    );

    return rsx! {
        div {
            class,
            role,
            aria_orientation,
            aria_labelledby,
            tabindex: tabindex.map(|v| v.to_string()),
            onclick: move |e| {
                if let Some(handler) = onclick {
                    handler.call(e);
                }
            },
            {children}
        }
    };
}
