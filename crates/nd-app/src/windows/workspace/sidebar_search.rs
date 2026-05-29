use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariant, ButtonVariants},
    input::{Input, InputState},
    ActiveTheme, Icon, IconName,
};

pub fn render(cx: &mut App, input: &Entity<InputState>) -> impl IntoElement {
    let theme = cx.theme();

    return div()
        .min_h(px(40.))
        .max_h(px(40.))
        .gap_2()
        .flex()
        .flex_row()
        .items_center()
        .justify_center()
        .border_b(px(1.))
        .border_color(theme.border)
        .child(
            Input::new(input)
                .prefix(Icon::new(IconName::Search))
                .appearance(false),
        )
        .child(
            Button::new("tests")
                .icon(Icon::new(IconName::Plus))
                .with_variant(ButtonVariant::Ghost)
                .h_full()
                .w_10()
                .border_l(px(1.))
                .border_color(theme.border)
                .rounded_none(),
        );
}
