use gpui::*;
use gpui_component::Theme;

pub struct NumberInputState {
    value: i32,
    min: i32,
    max: i32,
    step: i32,
}

impl NumberInputState {
    pub fn new(value: i32, min: i32, max: i32) -> Self {
        return Self {
            value: value.clamp(min, max),
            min,
            max,
            step: 1,
        };
    }

    #[allow(unused)]
    pub fn value(&self) -> i32 {
        return self.value;
    }

    #[allow(unused)]
    pub fn set_value(&mut self, value: i32) {
        self.value = value.clamp(self.min, self.max);
    }

    pub fn increment(&mut self) {
        self.value = (self.value + self.step).min(self.max);
    }

    pub fn decrement(&mut self) {
        self.value = (self.value - self.step).max(self.min);
    }
}

#[derive(IntoElement)]
pub struct NumberInput {
    state: Entity<NumberInputState>,
    id: ElementId,
    theme: Theme,
}

impl NumberInput {
    pub fn new(id: impl Into<ElementId>, state: &Entity<NumberInputState>, theme: &Theme) -> Self {
        Self {
            id: id.into(),
            state: state.clone(),
            theme: theme.clone(),
        }
    }
}

impl RenderOnce for NumberInput {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let value = self.state.read(cx).value;
        let state = self.state.clone();
        let theme = self.theme.clone();
        let state_dec = state.clone();
        let state_inc = state.clone();

        return div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap_0p5()
            .child(
                div()
                    .id(ElementId::Name("number-decrement".into()))
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(24.))
                    .h(px(24.))
                    .rounded(px(4.))
                    .bg(theme.muted.opacity(0.08))
                    .cursor_pointer()
                    .text_color(theme.foreground)
                    .text_sm()
                    .child("-")
                    .on_click(move |_event, _window, cx| {
                        state_dec.update(cx, |s, _cx| s.decrement());
                    }),
            )
            .child(
                div()
                    .w(px(48.))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.foreground.clone())
                            .child(value.to_string()),
                    ),
            )
            .child(
                div()
                    .id(ElementId::Name("number-increment".into()))
                    .flex()
                    .items_center()
                    .justify_center()
                    .w(px(24.))
                    .h(px(24.))
                    .rounded(px(4.))
                    .bg(theme.muted.opacity(0.08))
                    .cursor_pointer()
                    .text_color(theme.foreground)
                    .text_sm()
                    .child("+")
                    .on_click(move |_event, _window, cx| {
                        state_inc.update(cx, |s, _cx| s.increment());
                    }),
            );
    }
}
