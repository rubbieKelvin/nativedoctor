use crate::traits::Variant;

#[derive(PartialEq, Clone)]
#[allow(unused)]
pub enum BorderStyleVariant {
    Default,
    Mild,
}

impl Variant for BorderStyleVariant {
    fn classes(&self) -> &'static str {
        match self {
            BorderStyleVariant::Default => "border-[#3e3e3e]",
            BorderStyleVariant::Mild => "border-[#212121]",
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Border {
    pub left: Option<BorderSide>,
    pub top: Option<BorderSide>,
    pub right: Option<BorderSide>,
    pub bottom: Option<BorderSide>,
    pub style: BorderStyleVariant,
}

impl Border {
    pub fn all() -> Border {
        return Border {
            left: Some(BorderSide),
            right: Some(BorderSide),
            top: Some(BorderSide),
            bottom: Some(BorderSide),
            style: BorderStyleVariant::Default,
        };
    }

    pub fn none() -> Border {
        return Border {
            left: None,
            top: None,
            right: None,
            bottom: None,
            style: BorderStyleVariant::Default,
        };
    }

    #[allow(unused)]
    pub fn left() -> Border {
        return Border {
            left: Some(BorderSide),
            top: None,
            right: None,
            bottom: None,
            style: BorderStyleVariant::Default,
        };
    }

    #[allow(unused)]
    pub fn right() -> Border {
        return Border {
            right: Some(BorderSide),
            top: None,
            left: None,
            bottom: None,
            style: BorderStyleVariant::Default,
        };
    }

    #[allow(unused)]
    pub fn bottom() -> Border {
        return Border {
            bottom: Some(BorderSide),
            top: None,
            left: None,
            right: None,
            style: BorderStyleVariant::Default,
        };
    }

    #[allow(unused)]
    pub fn with_style(mut self, style: BorderStyleVariant) -> Self {
        self.style = style;
        return self;
    }
}

#[derive(PartialEq, Clone)]
pub struct BorderSide;

impl Border {
    pub fn classes(&self) -> String {
        let mut c: Vec<&'static str> = vec![];

        if let Some(_) = &self.top {
            c.push("border-t");
        }

        if let Some(_) = &self.bottom {
            c.push("border-b");
        }

        if let Some(_) = &self.left {
            c.push("border-l");
        }

        if let Some(_) = &self.right {
            c.push("border-r");
        }

        if c.len() > 0 {
            c.push(self.style.classes());
        }

        let classes = c.join(" ");
        return classes;
    }
}
