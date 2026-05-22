use gpui::*;
use gpui_component::{
    checkbox::Checkbox,
    input::{Input, InputState},
    ActiveTheme,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct KvRowState {
    enabled: bool,
    key: Entity<InputState>,
    value: Entity<InputState>,
    #[allow(unused)]
    is_secret: bool,
    description: Entity<InputState>,
}

impl KvRowState {
    pub fn new(cx: &mut App, window: &mut Window) -> Self {
        return Self {
            enabled: false,
            key: cx.new(|cx| InputState::new(window, cx).placeholder("Key")),
            value: cx.new(|cx| InputState::new(window, cx).placeholder("Value")),
            is_secret: false,
            description: cx.new(|cx| InputState::new(window, cx).placeholder("Description")),
        };
    }
}

#[derive(IntoElement)]
pub struct KvRow {
    state: Entity<KvRowState>,
}

impl KvRow {
    pub fn new(state: &Entity<KvRowState>) -> Self {
        return Self {
            state: state.clone(),
        };
    }
}

impl RenderOnce for KvRow {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let inner_state = self.state.read(cx);
        let theme = cx.theme();

        return div()
            .flex()
            .flex_row()
            .px_2()
            .border_b(px(1.))
            .border_color(theme.border)
            .items_center()
            .child(
                Checkbox::new(ElementId::Uuid(Uuid::new_v4()))
                    .checked(inner_state.enabled)
                    .pr_2(),
            )
            .child(
                Input::new(&inner_state.key)
                    .appearance(false)
                    .border_color(theme.border)
                    .border_l(px(1.)),
            )
            .child(
                Input::new(&inner_state.value)
                    .appearance(false)
                    .border_color(theme.border)
                    .border_l(px(1.)),
            )
            .child(
                Input::new(&inner_state.description)
                    .appearance(false)
                    .border_color(theme.border)
                    .border_l(px(1.)),
            );
    }
}
