use gpui::{prelude::FluentBuilder, *};
use gpui_component::{button, Sizable, Theme};

pub struct NumberInputState {
    value: i32,
    min: i32,
    max: i32,
    step: i32,
    unit: Option<SharedString>,
}

impl NumberInputState {
    pub fn new(value: i32, min: i32, max: i32, unit: Option<SharedString>) -> Self {
        return Self {
            value: value.clamp(min, max),
            min,
            max,
            step: 1,
            unit,
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
    dec_id: String,
    inc_id: String,
    theme: Theme,
}

impl NumberInput {
    pub fn new(id: impl Into<ElementId>, state: &Entity<NumberInputState>, theme: &Theme) -> Self {
        let element_id: ElementId = id.into();
        let key = match &element_id {
            ElementId::Name(s) => s.to_string(),
            ElementId::Integer(n) => n.to_string(),
            _ => format!("{:?}", element_id),
        };
        Self {
            dec_id: format!("{}-dec", key),
            inc_id: format!("{}-inc", key),
            id: element_id,
            state: state.clone(),
            theme: theme.clone(),
        }
    }
}

impl RenderOnce for NumberInput {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let value = self.state.read(cx).value;
        let unit = &self.state.read(cx).unit;
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
                button::Button::new(self.dec_id)
                    .label("-")
                    .xsmall()
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
                            .text_color(theme.foreground)
                            .child(value.to_string())
                            .when(unit.is_some(), {
                                let unit = unit.clone();
                                |el| {
                                    if let Some(unit) = unit {
                                        return el.flex().flex_row().gap(px(0.5)).child(unit);
                                    }
                                    return el;
                                }
                            }),
                    ),
            )
            .child(
                button::Button::new(self.inc_id)
                    .label("+")
                    .xsmall()
                    .on_click(move |_event, _window, cx| {
                        state_inc.update(cx, |s, _cx| s.increment());
                    }),
            );
    }
}
