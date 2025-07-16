use dioxus::prelude::*;
use strum;

use crate::traits::Variant;

#[derive(PartialEq, Clone, strum::EnumIter, strum::Display, Default)]
pub enum ButtonStyleVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Transparent,
    Link,
}

impl Variant for ButtonStyleVariant {
    fn classes(&self) -> &'static str {
        return match self {
            ButtonStyleVariant::Default => "bg-[#3d3d3d] hover:bg-[#464646] active:bg-[#5a5a5a]",
            ButtonStyleVariant::Secondary => "bg-[#245c80] hover:bg-[#26658d] active:bg-[#2d75a3]",
            ButtonStyleVariant::Destructive => {
                "bg-[#c50004] hover:bg-[#b8000b] active:bg-[#ca0004]"
            }
            ButtonStyleVariant::Outline => {
                "border border-[#82857e] hover:border-[#ffffff] active:bg-[#36373a]"
            }
            ButtonStyleVariant::Transparent => "bg-transparent",
            ButtonStyleVariant::Ghost => "hover:bg-[#353535] active:bg-[#4b4b4b]",
            ButtonStyleVariant::Link => "text-[#5c95ff] hover:underline",
        };
    }
}

#[derive(PartialEq, Clone, strum::Display, strum::EnumIter, Default)]
pub enum ButtonSizeVariant {
    Large,
    #[default]
    Default,
    Small,
    Tiny,
    Icon,
}

impl Variant for ButtonSizeVariant {
    fn classes(&self) -> &'static str {
        return match self {
            ButtonSizeVariant::Large => "rounded-md px-4 py-1",
            ButtonSizeVariant::Default => "px-2 py-0.5 rounded",
            ButtonSizeVariant::Small => "rounded px-1",
            ButtonSizeVariant::Tiny => "rounded-sm px-0.5",
            ButtonSizeVariant::Icon => "rounded p-1 flex items-center justify-center",
        };
    }
}

#[component]
pub fn Button(
    children: Element,
    title: Option<String>,
    class: Option<String>,
    style: Option<ButtonStyleVariant>,
    size: Option<ButtonSizeVariant>,
    onclick: Option<EventHandler<Event<MouseData>>>,
) -> Element {
    let style = style.unwrap_or_default();
    let size = size.unwrap_or_default();
    let class = format!(
        "{} {} {}",
        class.unwrap_or_default(),
        style.classes(),
        size.classes()
    );

    return rsx! {
        button {
            class,
            title,
            onclick: move |e| {
                if onclick.is_none(){
                    return;
                }

                onclick.unwrap().call(e)
            },
            {children}
        }
    };
}
