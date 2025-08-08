use gpui::{px, rgb, AbsoluteLength, Rgba};

pub struct Theme {
    pub text: Rgba,
    pub stroke: Rgba,
    pub background: Rgba,
    pub font_size_normal: AbsoluteLength
}

impl Default for Theme {
    fn default() -> Self {
        return Theme {
            text: rgb(0x737373),
            background: rgb(0x0e0e0e),
            stroke: rgb(0x48484a),
            font_size_normal: AbsoluteLength::Pixels(px(14.0))
        };
    }
}
