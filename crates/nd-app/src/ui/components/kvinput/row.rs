use gpui::*;
use gpui_component::{
    checkbox::Checkbox,
    input::{Input, InputEvent, InputState},
    ActiveTheme, Disableable,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct KvRowState {
    pub id: Uuid,
    _disabled: bool,
    enabled: bool,
    pub key: Entity<InputState>,
    pub value: Entity<InputState>,
    #[allow(unused)]
    is_secret: bool,
    pub description: Entity<InputState>,
    // sub
    subscriptions: Vec<Subscription>,
}

#[derive(Debug, Clone)]
pub enum KvColumn {
    Key,
    Value,
    Description,
}

#[derive(Debug, Clone)]
pub enum KvRowEvent {
    KeyChanged(SharedString),
    Blur(KvColumn),
}

impl EventEmitter<KvRowEvent> for KvRowState {}

impl KvRowState {
    pub fn new(cx: &mut Context<Self>, window: &mut Window) -> Self {
        let key = cx.new(|cx| InputState::new(window, cx).placeholder("Key"));
        let value = cx.new(|cx| InputState::new(window, cx).placeholder("Value"));
        let description = cx.new(|cx| InputState::new(window, cx).placeholder("Description"));

        let subscriptions = vec![
            // key input subscriptions
            cx.subscribe_in(
                &key,
                window,
                |_this, state, event: &InputEvent, _window, cx| match event {
                    InputEvent::Change => {
                        let content = state.read(cx).value();
                        cx.emit(KvRowEvent::KeyChanged(content));
                    }
                    InputEvent::Blur => {
                        cx.emit(KvRowEvent::Blur(KvColumn::Key));
                    }
                    _ => {}
                },
            ),
            // value input subscriptin
            cx.subscribe_in(
                &value,
                window,
                |_this, _state, event: &InputEvent, _window, cx| match event {
                    InputEvent::Blur => {
                        cx.emit(KvRowEvent::Blur(KvColumn::Value));
                    }
                    _ => {}
                },
            ),
            // description sub
            cx.subscribe_in(
                &value,
                window,
                |_this, _state, event: &InputEvent, _window, cx| match event {
                    InputEvent::Blur => {
                        cx.emit(KvRowEvent::Blur(KvColumn::Description));
                    }
                    _ => {}
                },
            ),
        ];

        return Self {
            id: Uuid::new_v4(),
            _disabled: false,
            enabled: false,
            key,
            value,
            description,
            is_secret: false,
            subscriptions,
        };
    }

    pub fn disabled(mut self) -> Self {
        self._disabled = true;
        return self;
    }

    pub fn with_key(
        self,
        cx: &mut Context<Self>,
        window: &mut Window,
        value: SharedString,
    ) -> Self {
        self.key.update(cx, |this, cx| {
            this.set_value(value, window, cx);
        });
        return self;
    }

    pub fn with_value(
        self,
        cx: &mut Context<Self>,
        window: &mut Window,
        value: SharedString,
    ) -> Self {
        self.value.update(cx, |this, cx| {
            this.set_value(value, window, cx);
        });
        return self;
    }

    pub fn with_description(
        self,
        cx: &mut Context<Self>,
        window: &mut Window,
        value: SharedString,
    ) -> Self {
        self.description.update(cx, |this, cx| {
            this.set_value(value, window, cx);
        });
        return self;
    }

    pub fn focus(&mut self, cx: &mut Context<Self>, window: &mut Window) {
        self.key.update(cx, |this, cx| {
            this.focus(window, cx);
        });
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
                    .disabled(inner_state._disabled)
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
