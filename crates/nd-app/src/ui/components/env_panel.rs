use gpui::*;
use gpui_component::{input, ActiveTheme, StyledExt, Theme};

use crate::ui::components::kvinput::{self, KvInputState};

pub struct EnvPanel {
    env_name_state: Entity<input::InputState>,
    variables_state: Entity<KvInputState>,
}

impl EnvPanel {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let env_name_state =
            cx.new(|cx| input::InputState::new(window, cx).placeholder("Environment name..."));

        let variables_state = cx.new(|cx| {
            let mut state = KvInputState::new(cx, window);
            state.new_row(cx, window, "API_URL", "https://api.example.com", "");
            state.new_row(cx, window, "DEBUG", "true", "");
            state.new_row(cx, window, "LOG_LEVEL", "info", "");
            return state;
        });

        return Self {
            env_name_state,
            variables_state,
        };
    }

    fn render_header(&mut self, theme: &Theme) -> impl IntoElement {
        return div()
            .px_2()
            .py_2()
            .border_b(px(1.))
            .border_color(theme.border)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .text_xs()
                            .font_semibold()
                            .text_color(theme.muted_foreground)
                            .child("Name"),
                    )
                    .child(
                        div().flex_1().child(input::Input::new(&self.env_name_state)),
                    ),
            )
            .child(
                div()
                    .pt_2()
                    .text_xs()
                    .font_semibold()
                    .text_color(theme.muted_foreground)
                    .child("Variables"),
            );
    }
}

impl Render for EnvPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();

        return div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .child(self.render_header(&theme))
            .child(div().flex_1().child(kvinput::KvInput::new(&self.variables_state)));
    }
}
