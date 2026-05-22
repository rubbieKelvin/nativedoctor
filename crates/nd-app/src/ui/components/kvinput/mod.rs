use gpui::*;
pub mod row;

pub struct KvInputState {
    /// always-present empty row used to create new entries.
    empty_row: Entity<row::KvRowState>,
    /// we need to hold the item along side it's subscription, so the subsction only lives as link at it's state is in the vector
    items: Vec<(Entity<row::KvRowState>, Subscription)>,
    /// subscriptions that must live as long as this state.
    #[allow(unused)]
    subscriptions: Vec<Subscription>,
}

impl KvInputState {
    pub fn new(cx: &mut Context<Self>, window: &mut Window) -> Self {
        let empty_row = cx.new(|cx| row::KvRowState::new(cx, window).as_dummy());
        let empty_row_sub = Self::create_empty_row_subscription(&empty_row, cx, window);

        return KvInputState {
            empty_row,
            items: vec![],
            subscriptions: vec![empty_row_sub],
        };
    }

    fn create_empty_row_subscription(
        empty_row: &Entity<row::KvRowState>,
        cx: &mut Context<Self>,
        window: &mut Window,
    ) -> Subscription {
        return cx.subscribe_in(
            empty_row,
            window,
            |this, state, event: &row::KvRowEvent, window, cx| match event {
                row::KvRowEvent::KeyChanged(content) => {
                    // when we get key input in empty row, we need to create a new row with the key and clear the empty row's inputs
                    let key = content.trim();

                    if key.is_empty() {
                        return;
                    }

                    let value = state.read(cx).value.read(cx).value();
                    let description = state.read(cx).description.read(cx).value();

                    this.new_row(cx, window, key, &value, &description);
                    this.clear_empty_row_inputs(state, window, cx);
                }
                _ => {}
            },
        );
    }

    fn clear_empty_row_inputs(
        &self,
        state: &Entity<row::KvRowState>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        state.update(cx, |this, cx| {
            this.key.update(cx, |input, cx| {
                input.set_value(SharedString::new(""), window, cx)
            });
            this.value.update(cx, |input, cx| {
                input.set_value(SharedString::new(""), window, cx)
            });
            this.description.update(cx, |input, cx| {
                input.set_value(SharedString::new(""), window, cx)
            });
        });
    }

    pub fn new_row(
        &mut self,
        cx: &mut Context<Self>,
        window: &mut Window,
        key: &str,
        value: &str,
        description: &str,
    ) {
        let row_state = Self::create_row_entity(cx, window, key, value, description);
        let subscription = Self::create_regular_row_subscription(&row_state, cx, window);
        self.items.push((row_state, subscription));
    }

    fn create_row_entity(
        cx: &mut Context<Self>,
        window: &mut Window,
        key: &str,
        value: &str,
        description: &str,
    ) -> Entity<row::KvRowState> {
        return cx.new(|cx| {
            let mut new_row = row::KvRowState::new(cx, window).with_defaults(
                cx,
                window,
                SharedString::new(key),
                SharedString::new(value),
                SharedString::new(description),
            );

            new_row.enabled.write(cx, true);
            new_row.focus(cx, window);

            return new_row;
        });
    }

    fn create_regular_row_subscription(
        row_state: &Entity<row::KvRowState>,
        cx: &mut Context<Self>,
        window: &mut Window,
    ) -> Subscription {
        return cx.subscribe_in(
            row_state,
            window,
            |this, state, event, _window, cx| match event {
                row::KvRowEvent::Blur => {
                    if Self::is_row_empty(state, cx) {
                        this.items.retain(|(entity, _)| entity != state);
                    }
                }
                _ => {}
            },
        );
    }

    fn is_row_empty(state: &Entity<row::KvRowState>, cx: &mut App) -> bool {
        let inner = state.read(cx);
        let key = inner.key.read(cx).value();
        let value = inner.value.read(cx).value();
        let description = inner.description.read(cx).value();

        return key.trim().is_empty() && value.trim().is_empty() && description.trim().is_empty();
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
                    .map(|(entity, _)| row::KvRow::new(entity))
                    .collect::<Vec<row::KvRow>>(),
            )
            //  render the empty row last so the user can add new entries.
            .child(row::KvRow::new(&state.empty_row));
    }
}
