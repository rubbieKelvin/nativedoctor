use gpui::*;
pub mod row;

pub struct KvInputState {
    empty_row: Entity<row::KvRowState>,
    // items with they subscripion that needs to stay alive with them
    items: Vec<(Entity<row::KvRowState>, Subscription)>,
    // sub
    #[allow(unused)]
    subscriptions: Vec<Subscription>,
}

impl KvInputState {
    pub fn new(cx: &mut Context<Self>, window: &mut Window) -> Self {
        let empty_row = cx.new(|cx| row::KvRowState::new(cx, window).disabled());

        let _empty_row_sub = cx.subscribe_in(
            &empty_row,
            window,
            |this, state, event: &row::KvRowEvent, window, cx| match event {
                row::KvRowEvent::KeyChanged(content) => {
                    let key = content.trim();

                    if key.len() == 0 {
                        return;
                    }

                    let value = state.read(cx).value.read(cx).value();
                    let description = state.read(cx).description.read(cx).value();

                    // add a new item in the row with that content
                    this.new_row(cx, window, key, &value, &description);

                    // then clear the key of the current content
                    state.update(cx, |this, cx| {
                        this.key.update(cx, |this, cx| {
                            this.set_value(SharedString::new(""), window, cx);
                        });

                        this.value.update(cx, |this, cx| {
                            this.set_value(SharedString::new(""), window, cx);
                        });

                        this.description.update(cx, |this, cx| {
                            this.set_value(SharedString::new(""), window, cx);
                        });
                    })
                }
                _ => {}
            },
        );

        return KvInputState {
            empty_row,
            items: vec![],
            subscriptions: vec![_empty_row_sub],
        };
    }

    pub fn new_row(
        &mut self,
        cx: &mut Context<Self>,
        window: &mut Window,
        key: &str,
        value: &str,
        description: &str,
    ) {
        let row_state = cx.new(|cx| {
            let mut new_row = row::KvRowState::new(cx, window)
                .with_key(cx, window, SharedString::new(key))
                .with_value(cx, window, SharedString::new(value))
                .with_description(cx, window, SharedString::new(description));

            new_row.enabled.write(cx, true);
            new_row.focus(cx, window);
            return new_row;
        });

        let subscription = cx.subscribe_in(
            &row_state,
            window,
            |this, state, event, _window, cx| match event {
                row::KvRowEvent::Blur => {
                    let key = state.read(cx).key.read(cx).value();
                    let value = state.read(cx).value.read(cx).value();
                    let description = state.read(cx).description.read(cx).value();

                    if key.trim().len() == 0
                        && description.trim().len() == 0
                        && value.trim().len() == 0
                    {
                        this.items.retain(|(entity, _)| entity != state);
                    }
                }
                _ => {}
            },
        );

        self.items.push((row_state, subscription));
    }
}

#[derive(IntoElement)]
pub struct KvInput {
    state: Entity<KvInputState>,
}

impl KvInput {
    pub fn new(state: &Entity<KvInputState>) -> Self {
        return Self {
            state: state.clone(),
        };
    }
}

impl RenderOnce for KvInput {
    fn render(self, _window: &mut gpui::Window, cx: &mut App) -> impl gpui::prelude::IntoElement {
        let state = self.state.read(cx);

        return div()
            .children(
                state
                    .items
                    .iter()
                    .map(|row_state| {
                        return row::KvRow::new(&row_state.0);
                    })
                    .collect::<Vec<row::KvRow>>(),
            )
            .child(row::KvRow::new(&state.empty_row));
    }
}
