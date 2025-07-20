use dioxus::prelude::*;

use crate::{border::Border, pane::{Pane, PaneStyleVariant}};

#[component]
pub fn HorizontalSeparator(class: Option<String>) -> Element {
    let class = class.unwrap_or("w-full".to_string());

    return rsx!{
        Pane { border: Border::bottom(), style: PaneStyleVariant::Transparent, class }
    }
}

#[component]
pub fn VerticalSeparator(class: Option<String>) -> Element {
    let class = class.unwrap_or("h-full".to_string());

    return rsx!{
        Pane { border: Border::left(), style: PaneStyleVariant::Transparent, class }
    };
}