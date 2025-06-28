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
            ButtonSizeVariant::Icon => "w-6 h-6 rounded",
        };
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct ButtonProps {
    pub children: Element,
    pub class: Option<&'static str>,
    pub style: Option<ButtonStyleVariant>,
    pub size: Option<ButtonSizeVariant>,
    pub onclick: Option<EventHandler<Event<MouseData>>>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let style = props.style.unwrap_or_default();
    let size = props.size.unwrap_or_default();
    let class = format!(
        "{} {} {}",
        props.class.unwrap_or(""),
        style.classes(),
        size.classes()
    );

    return rsx! {
        button {
            class,
            onclick: move |e| {
                if props.onclick.is_none(){
                    return;
                }

                props.onclick.unwrap().call(e)
            },
            {props.children}
        }
    };
}
