//! An editable list of name/value rows, for query parameters and headers.

use gpui_kit::component::button::*;
use gpui_kit::component::checkbox::Checkbox;
use gpui_kit::component::input::{Input, InputEvent, InputState};
use gpui_kit::component::*;
use gpui_kit::*;
use nd_core::Pairs;

pub struct PairsChanged;

struct Row {
    name: Entity<InputState>,
    value: Entity<InputState>,
    /// The name is sent as "no value", which removes a default header.
    remove: bool,
}

pub struct PairsView {
    id: &'static str,
    rows: Vec<Row>,
    /// Rows get an extra checkbox for "remove this header".
    removable: bool,
}

impl EventEmitter<PairsChanged> for PairsView {}

impl PairsView {
    pub fn new(
        id: &'static str,
        pairs: &Pairs,
        removable: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let mut view = PairsView {
            id,
            rows: Vec::new(),
            removable,
        };
        view.load(pairs, window, cx);
        return view;
    }

    /// Replaces every row, without announcing a change.
    pub fn load(&mut self, pairs: &Pairs, window: &mut Window, cx: &mut Context<Self>) {
        self.rows.clear();
        for (name, value) in &pairs.0 {
            let row = self.make_row(name, value.as_deref().unwrap_or(""), value.is_none(), window, cx);
            self.rows.push(row);
        }
        cx.notify();
    }

    fn make_row(
        &self,
        name: &str,
        value: &str,
        remove: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Row {
        let name_text = name.to_string();
        let value_text = value.to_string();
        let name = cx.new(|cx| InputState::new(window, cx).placeholder("name").default_value(name_text));
        let value = cx.new(|cx| InputState::new(window, cx).placeholder("value").default_value(value_text));
        for input in [&name, &value] {
            cx.subscribe_in(input, window, |_, _, event: &InputEvent, _, cx| {
                if matches!(event, InputEvent::Change) {
                    cx.emit(PairsChanged);
                }
            })
            .detach();
        }
        return Row { name, value, remove };
    }

    /// The rows as a model. Rows with no name are skipped.
    pub fn pairs(&self, cx: &App) -> Pairs {
        let mut pairs = Pairs::default();
        for row in &self.rows {
            let name = row.name.read(cx).value().trim().to_string();
            if name.is_empty() {
                continue;
            }
            let value = (!row.remove).then(|| row.value.read(cx).value().to_string());
            pairs.push(name, value);
        }
        return pairs;
    }

    pub fn add(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let row = self.make_row("", "", false, window, cx);
        let name = row.name.clone();
        self.rows.push(row);
        name.update(cx, |state, cx| state.focus(window, cx));
        cx.notify();
    }

    fn delete(&mut self, index: usize, cx: &mut Context<Self>) {
        if index < self.rows.len() {
            self.rows.remove(index);
            cx.emit(PairsChanged);
            cx.notify();
        }
    }

    fn set_remove(&mut self, index: usize, remove: bool, cx: &mut Context<Self>) {
        if let Some(row) = self.rows.get_mut(index) {
            row.remove = remove;
            cx.emit(PairsChanged);
            cx.notify();
        }
    }
}

impl Render for PairsView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();
        let id = self.id;
        let mut list = div().v_flex().gap_2().p_3();
        for (index, row) in self.rows.iter().enumerate() {
            let mut line = div()
                .h_flex()
                .gap_2()
                .child(div().flex_1().child(Input::new(&row.name)))
                .child(div().flex_1().child(Input::new(&row.value).disabled(row.remove)));
            if self.removable {
                line = line.child(
                    Checkbox::new((id, index))
                        .label("Remove")
                        .checked(row.remove)
                        .on_click(cx.listener(move |this, checked: &bool, _, cx| {
                            this.set_remove(index, *checked, cx)
                        })),
                );
            }
            line = line.child(
                Button::new((id, 1000 + index))
                    .ghost()
                    .compact()
                    .icon(IconName::Close)
                    .tooltip("Delete row")
                    .on_click(cx.listener(move |this, _, _, cx| this.delete(index, cx))),
            );
            list = list.child(line);
        }
        if self.rows.is_empty() {
            list = list.child(
                div()
                    .text_sm()
                    .text_color(theme.muted_foreground)
                    .child("Nothing here yet"),
            );
        }
        return list.child(
            div().child(
                Button::new((id, 9999usize))
                    .label("Add")
                    .icon(IconName::Plus)
                    .on_click(cx.listener(|this, _, window, cx| this.add(window, cx))),
            ),
        );
    }
}
