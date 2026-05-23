use gpui::{prelude::FluentBuilder, *};
use gpui_component::{
    button::{self, ButtonVariants},
    input,
    popover::Popover,
    scroll::ScrollableElement,
    ActiveTheme, Icon, IconName, Sizable,
};
use std::rc::Rc;

pub struct EnvPopupState {
    pub environments: Vec<SharedString>,
    pub active_idx: usize,
    search_state: Entity<input::InputState>,
    is_open: bool,
    on_select: Option<Rc<dyn Fn(usize)>>,
}

impl EnvPopupState {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_state =
            cx.new(|cx| input::InputState::new(window, cx).placeholder("Search environments..."));

        return Self {
            environments: vec!["Development".into(), "Staging".into(), "Production".into()],
            active_idx: 0,
            search_state,
            is_open: false,
            on_select: None,
        };
    }

    #[allow(unused)]
    pub fn on_select(mut self, callback: impl Fn(usize) + 'static) -> Self {
        self.on_select = Some(Rc::new(callback));
        return self;
    }

    fn select_item(&mut self, idx: usize, _cx: &mut Context<Self>) {
        self.active_idx = idx;
        self.is_open = false;
        if let Some(cb) = &self.on_select {
            cb(idx);
        }
    }

    fn filtered(&self, cx: &App) -> Vec<(usize, SharedString)> {
        let query = self.search_state.read(cx).value();
        if query.is_empty() {
            return self
                .environments
                .iter()
                .enumerate()
                .map(|(i, n)| (i, n.clone()))
                .collect();
        }

        let q = query.to_lowercase();
        return self
            .environments
            .iter()
            .enumerate()
            .filter(|(_, name)| name.to_lowercase().contains(&q))
            .map(|(i, n)| (i, n.clone()))
            .collect();
    }
}

#[derive(IntoElement)]
pub struct EnvPopup {
    state: Entity<EnvPopupState>,
}

impl EnvPopup {
    pub fn new(state: Entity<EnvPopupState>) -> Self {
        return Self { state };
    }
}

impl RenderOnce for EnvPopup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self.state.clone();
        let active_name = {
            let s = state.read(cx);
            s.environments
                .get(s.active_idx)
                .cloned()
                .unwrap_or_default()
        };

        return Popover::new("env-popup")
            .anchor(Anchor::TopLeft)
            .open(state.read(cx).is_open)
            .p_0()
            .on_open_change({
                let state = state.clone();
                move |is_open, _window, cx| {
                    state.update(cx, |this, cx| {
                        this.is_open = *is_open;
                        cx.notify();
                    });
                }
            })
            .trigger(
                button::Button::new(SharedString::from("env-button"))
                    .label(active_name)
                    .small()
                    .with_variant(button::ButtonVariant::Text)
                    .icon(IconName::ChevronsUpDown),
            )
            .content({
                let state = state.clone();
                move |_, _window, cx| {
                    let theme = cx.theme();
                    let active = state.read(cx).active_idx;
                    let filtered = state.read(cx).filtered(cx);

                    let popover_entity = cx.entity();

                    return div()
                        .w_64()
                        .gap_2()
                        .child(
                            div()
                                .flex()
                                .flex_row()
                                .items_center()
                                .gap_2()
                                .pr_1()
                                .child(
                                    input::Input::new(&state.read(cx).search_state)
                                        .appearance(false),
                                )
                                .child(
                                    button::Button::new(SharedString::from("add-env"))
                                        .icon(Icon::new(IconName::Plus))
                                        .small(),
                                )
                                .border_b_1()
                                .border_color(theme.border),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .max_h(px(240.))
                                .overflow_y_scrollbar()
                                .children(filtered.iter().map(|(idx, name)| {
                                    let is_active = *idx == active;
                                    let name = name.clone();
                                    let my_idx = *idx;
                                    let state = state.clone();
                                    let popover_entity = popover_entity.clone();

                                    div()
                                        .px_2()
                                        .py_1()
                                        .when(is_active, |this: Div| {
                                            this.bg(theme.primary.opacity(0.1))
                                        })
                                        .hover(|style| style.bg(theme.muted))
                                        .cursor_pointer()
                                        .child(name)
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            move |_event, window, cx| {
                                                state.update(cx, |this, cx| {
                                                    this.select_item(my_idx, cx);
                                                });
                                                popover_entity.update(cx, |s, cx| {
                                                    s.dismiss(window, cx);
                                                });
                                            },
                                        )
                                })),
                        );
                }
            });
    }
}
