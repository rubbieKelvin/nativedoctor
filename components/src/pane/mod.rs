use super::traits::Variant;
use crate::border::Border;
use dioxus::prelude::*;

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
pub fn Pane(
    class: Option<String>,
    style: Option<PaneStyleVariant>,
    border: Option<Border>,
    children: Element,
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
            {children}
        }
    };
}
