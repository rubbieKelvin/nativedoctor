use gpui::*;
pub mod row;

pub struct KvInputState {
    empty_row: Entity<row::KvRowState>,
    items: Vec<Entity<row::KvRowState>>,
}

impl KvInputState {
    pub fn new(cx: &mut Context<Self>, window: &mut Window) -> Self {
        return KvInputState {
            empty_row: cx.new(|cx| row::KvRowState::new(cx, window)),
            items: vec![],
        };
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
                        return row::KvRow::new(row_state);
                    })
                    .collect::<Vec<row::KvRow>>(),
            )
            .child(row::KvRow::new(&state.empty_row));
    }
}
