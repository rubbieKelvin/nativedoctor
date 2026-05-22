use gpui::*;
use gpui_component::{
    checkbox::Checkbox,
    input::{Input, InputEvent, InputState},
    ActiveTheme, Disableable,
};

#[derive(Debug)]
pub struct KvRowState {
    _is_dummy_row: bool,
    pub enabled: Entity<bool>,
    pub key: Entity<InputState>,
    pub value: Entity<InputState>,
    #[allow(unused)]
    is_secret: bool,
    pub description: Entity<InputState>,
    /// subscriptions kept alive for the lifetime of this row.
    #[allow(unused)]
    subscriptions: Vec<Subscription>,
}

#[derive(Debug, Clone)]
pub enum KvRowEvent {
    KeyChanged(SharedString),
    Blur,
}

impl EventEmitter<KvRowEvent> for KvRowState {}

impl KvRowState {
    pub fn new(cx: &mut Context<Self>, window: &mut Window) -> Self {
        let key = cx.new(|cx| InputState::new(window, cx).placeholder("Key"));
        let value = cx.new(|cx| InputState::new(window, cx).placeholder("Value"));
        let description = cx.new(|cx| InputState::new(window, cx).placeholder("Description"));

        let subscriptions = vec![
            Self::subscribe_to_key_input(&key, cx, window),
            Self::subscribe_to_blur(&value, cx, window),
            Self::subscribe_to_blur(&description, cx, window),
        ];

        return Self {
            _is_dummy_row: false,
            enabled: cx.new(|_cx| false),
            key,
            value,
            description,
            is_secret: false,
            subscriptions,
        };
    }

    fn subscribe_to_key_input(
        key: &Entity<InputState>,
        cx: &mut Context<Self>,
        window: &mut Window,
    ) -> Subscription {
        return cx.subscribe_in(
            key,
            window,
            |_this, state, event: &InputEvent, _window, cx| match event {
                InputEvent::Change => {
                    let content = state.read(cx).value();
                    cx.emit(KvRowEvent::KeyChanged(content));
                }
                InputEvent::Blur => {
                    cx.emit(KvRowEvent::Blur);
                }
                _ => {}
            },
        );
    }

    fn subscribe_to_blur(
        input: &Entity<InputState>,
        cx: &mut Context<Self>,
        window: &mut Window,
    ) -> Subscription {
        return cx.subscribe_in(
            input,
            window,
            |_this, _state, event: &InputEvent, _window, cx| match event {
                InputEvent::Blur => {
                    cx.emit(KvRowEvent::Blur);
                }
                _ => {}
            },
        );
    }

    pub fn as_dummy(mut self) -> Self {
        self._is_dummy_row = true;
        self
    }

    pub fn with_defaults(
        self,
        cx: &mut Context<Self>,
        window: &mut Window,
        key: SharedString,
        value: SharedString,
        description: SharedString,
    ) -> Self {
        self.key
            .update(cx, |this, cx| this.set_value(key, window, cx));
        self.value
            .update(cx, |this, cx| this.set_value(value, window, cx));
        self.description
            .update(cx, |this, cx| this.set_value(description, window, cx));
        return self;
    }

    // Put focus on the key input
    pub fn focus(&mut self, cx: &mut Context<Self>, window: &mut Window) {
        self.key.update(cx, |this, cx| this.focus(window, cx));
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

    fn render_checkbox(inner_state: &KvRowState, cx: &App) -> impl IntoElement {
        return Checkbox::new(ElementId::Name("kv-row-enabled".into()))
            .checked(*inner_state.enabled.read(cx))
            .disabled(inner_state._is_dummy_row)
            .pr_2()
            .on_click({
                let enabled = inner_state.enabled.clone();
                move |checked, _, cx| {
                    enabled.write(cx, *checked);
                }
            });
    }

    fn render_input(input: &Entity<InputState>, border_color: Hsla) -> Div {
        return div().size_full().child(
            Input::new(input)
                .appearance(false)
                .border_color(border_color)
                .border_l(px(1.)),
        );
    }
}

impl RenderOnce for KvRow {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let inner_state = self.state.read(cx);
        let border_color = cx.theme().border;

        return div()
            .flex()
            .flex_row()
            .px_2()
            .border_b(px(1.))
            .border_color(border_color)
            .items_center()
            .child(Self::render_checkbox(&inner_state, cx))
            .child(Self::render_input(&inner_state.key, border_color))
            .child(Self::render_input(&inner_state.value, border_color))
            .child(Self::render_input(&inner_state.description, border_color));
    }
}
