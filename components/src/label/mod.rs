use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum LabelSizeVariant {
    Large,
    Default,
    Small,
    Tiny,
}

impl LabelSizeVariant {
    fn classes(&self) -> &'static str {
        return match self {
            LabelSizeVariant::Large => "text-xl",
            LabelSizeVariant::Default => "text-base",
            LabelSizeVariant::Small => "text-sm",
            LabelSizeVariant::Tiny => "text-xs",
        };
    }

    #[allow(unused)]
    pub fn to_string(&self) -> &'static str {
        return match self {
            LabelSizeVariant::Large => "Large",
            LabelSizeVariant::Default => "Default",
            LabelSizeVariant::Small => "Small",
            LabelSizeVariant::Tiny => "Tiny",
        };
    }

    #[allow(unused)]
    pub fn all() -> &'static [LabelSizeVariant] {
        return &[
            LabelSizeVariant::Large,
            LabelSizeVariant::Default,
            LabelSizeVariant::Small,
            LabelSizeVariant::Tiny,
        ];
    }
}


#[derive(Clone, PartialEq)]
pub enum LabelStyleVariant {
    Default,
    Ghost,
}

impl LabelStyleVariant {
    fn classes(&self) -> &'static str {
        return match self {
            LabelStyleVariant::Default => "text-[#b4b4b4]",
            LabelStyleVariant::Ghost => "text-[#898989]",
        };
    }

    #[allow(unused)]
    pub fn all() -> &'static [LabelStyleVariant] {
        return &[LabelStyleVariant::Default, LabelStyleVariant::Ghost];
    }
}

#[component]
pub fn Label(
    class: Option<&'static str>,
    style: Option<LabelStyleVariant>,
    size: Option<LabelSizeVariant>,
    children: Element,
) -> Element {
    let size = size.unwrap_or(LabelSizeVariant::Default);
    let style = style.unwrap_or(LabelStyleVariant::Default);
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
