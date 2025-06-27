use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub enum PaneStyleVariant {
    Lighter,
    Default,
    Dark,
    Evil,
}

impl PaneStyleVariant {
    fn classes(&self) -> &'static str {
        return match self {
            PaneStyleVariant::Lighter => "bg-[#202020]",
            PaneStyleVariant::Default => "bg-[#1b1b1b]",
            PaneStyleVariant::Dark => "bg-[#141414]",
            PaneStyleVariant::Evil => "bg-[#0a0a0a]",
        };
    }

    pub fn all() -> &'static [PaneStyleVariant] {
        return &[
            PaneStyleVariant::Lighter,
            PaneStyleVariant::Default,
            PaneStyleVariant::Dark,
            PaneStyleVariant::Evil
        ];
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