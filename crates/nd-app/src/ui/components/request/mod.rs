mod request_tab_auth;
mod request_tab_body;
mod request_tab_docs;
mod request_tab_headers;
mod request_tab_params;
mod request_tab_settings;

use gpui::*;
use gpui_component::{
    input,
    resizable::{resizable_panel, v_resizable},
    select::{self, SelectState},
    ActiveTheme, IndexPath, StyledExt, Theme,
};

use crate::ui::components::kvinput;

pub struct RequestPanel {
    url_input_state: Entity<input::InputState>,
    body_text_state: Entity<input::InputState>,
    docs_input_state: Entity<input::InputState>,
    method_state: Entity<select::SelectState<Vec<SharedString>>>,
    param_input_state: Entity<kvinput::KvInputState>,
    headers_input_state: Entity<kvinput::KvInputState>,
    auth_type: usize,
    bearer_token_state: Entity<input::InputState>,
    basic_auth_username_state: Entity<input::InputState>,
    basic_auth_password_state: Entity<input::InputState>,
    api_key_name_state: Entity<input::InputState>,
    api_key_value_state: Entity<input::InputState>,
    active_tab: usize,
    body_type: usize,
    raw_sub_type: usize,
    binary_path: SharedString,
    form_data_state: Entity<kvinput::KvInputState>,
    url_encoded_state: Entity<kvinput::KvInputState>,
    ssl_verify: bool,
    follow_redirects: bool,
    http_version: usize,
}

impl RequestPanel {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let url_input_state =
            cx.new(|cx| input::InputState::new(window, cx).placeholder("Enter request URL..."));
        let body_text_state = cx.new(|cx| {
            input::InputState::new(window, cx)
                .placeholder("{\n  \"key\": \"value\"\n}")
                .code_editor("json")
                .multi_line(true)
                .soft_wrap(true)
                .line_number(false)
        });

        let docs_input_state = cx.new(|cx| {
            input::InputState::new(window, cx)
                .placeholder("Documentation for this request")
                .multi_line(true)
                .soft_wrap(true)
                .code_editor("markdown")
                .line_number(false)
        });
        let param_input_state = cx.new(|cx| kvinput::KvInputState::new(cx, window));
        let headers_input_state = cx.new(|cx| kvinput::KvInputState::new(cx, window));
        let form_data_state = cx.new(|cx| kvinput::KvInputState::new(cx, window));
        let url_encoded_state = cx.new(|cx| kvinput::KvInputState::new(cx, window));

        let method_state = cx.new(|cx| {
            SelectState::new(
                vec![
                    SharedString::new("GET"),
                    SharedString::new("POST"),
                    SharedString::new("PUT"),
                    SharedString::new("PATCH"),
                    SharedString::new("DELETE"),
                    SharedString::new("HEAD"),
                    SharedString::new("OPTIONS"),
                ],
                Some(IndexPath::default()),
                window,
                cx,
            )
        });

        let bearer_token_state =
            cx.new(|cx| input::InputState::new(window, cx).placeholder("Enter bearer token..."));
        let basic_auth_username_state =
            cx.new(|cx| input::InputState::new(window, cx).placeholder("Username"));
        let basic_auth_password_state =
            cx.new(|cx| input::InputState::new(window, cx).placeholder("Password"));
        let api_key_name_state =
            cx.new(|cx| input::InputState::new(window, cx).placeholder("Key name"));
        let api_key_value_state =
            cx.new(|cx| input::InputState::new(window, cx).placeholder("Key value"));

        return Self {
            url_input_state,
            body_text_state,
            docs_input_state,
            param_input_state,
            headers_input_state,
            method_state,
            auth_type: 0,
            bearer_token_state,
            basic_auth_username_state,
            basic_auth_password_state,
            api_key_name_state,
            api_key_value_state,
            active_tab: 0,
            body_type: 0,
            raw_sub_type: 0,
            binary_path: SharedString::default(),
            form_data_state,
            url_encoded_state,
            ssl_verify: true,
            follow_redirects: true,
            http_version: 0,
        };
    }

    fn render_tab_button(
        label: &str,
        index: usize,
        active_tab: usize,
        theme: &Theme,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_active = index == active_tab;

        let fg = if is_active {
            theme.foreground
        } else {
            theme.muted_foreground
        };

        let border_color = if is_active {
            theme.blue
        } else {
            hsla(0., 0., 0., 0.)
        };

        let label = label.to_string();
        let id = format!("tab-{}", label.to_lowercase());

        return div()
            .id(ElementId::Name(id.into()))
            .p_2()
            .text_sm()
            .text_color(fg)
            .cursor_pointer()
            .border_b_2()
            .border_color(border_color)
            .on_click(cx.listener(move |this, _event, _window, _cx| {
                this.active_tab = index;
            }))
            .child(label);
    }

    fn render_top_bar(&mut self, theme: &Theme, _cx: &mut Context<Self>) -> impl IntoElement {
        return div()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .justify_center()
            .px_2()
            .min_h(px(50.))
            .max_h(px(50.))
            .border_b(px(1.))
            .border_color(theme.border)
            .child(
                div()
                    .flex()
                    .flex_row()
                    .gap_0p5()
                    .child(select::Select::new(&self.method_state)),
            )
            .child(
                div()
                    .flex_1()
                    .child(input::Input::new(&self.url_input_state)),
            )
            .child(self.render_send_button(theme));
    }

    fn render_send_button(&self, theme: &Theme) -> impl IntoElement {
        return div()
            .id("send-button")
            .px_4()
            .py_1p5()
            .bg(theme.blue)
            .rounded(px(4.))
            .text_sm()
            .cursor_pointer()
            .child("Send");
    }

    fn render_request_panel(&mut self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        return div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .child(self.render_tab_bar(theme, cx))
            .child(self.render_tab_content(theme, cx));
    }

    fn render_tab_bar(&mut self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let active = self.active_tab;

        return div()
            .flex()
            .flex_row()
            .border_b(px(1.))
            .border_color(theme.border)
            .px_2()
            .bg(theme.muted.opacity(0.04))
            .child(Self::render_tab_button("Docs", 0, active, theme, cx))
            .child(Self::render_tab_button("Params", 1, active, theme, cx))
            .child(Self::render_tab_button("Auth", 2, active, theme, cx))
            .child(Self::render_tab_button("Headers", 3, active, theme, cx))
            .child(Self::render_tab_button("Body", 4, active, theme, cx))
            .child(Self::render_tab_button("Settings", 5, active, theme, cx));
    }

    fn render_tab_content(&mut self, theme: &Theme, cx: &mut Context<Self>) -> AnyElement {
        return match self.active_tab {
            0 => request_tab_docs::render(theme, &self.docs_input_state).into_any_element(),
            1 => request_tab_params::render(theme, &self.param_input_state).into_any_element(),
            2 => request_tab_auth::render(
                self.auth_type,
                &self.bearer_token_state,
                &self.basic_auth_username_state,
                &self.basic_auth_password_state,
                &self.api_key_name_state,
                &self.api_key_value_state,
                theme,
                cx,
            )
            .into_any_element(),
            3 => request_tab_headers::render(theme, &self.headers_input_state).into_any_element(),
            4 => request_tab_body::render(
                self.body_type,
                &self.body_text_state,
                &self.form_data_state,
                &self.url_encoded_state,
                self.raw_sub_type,
                &self.binary_path,
                theme,
                cx,
            )
            .into_any_element(),
            5 => request_tab_settings::render(
                self.ssl_verify,
                self.follow_redirects,
                self.http_version,
                theme,
                cx,
            )
            .into_any_element(),
            _ => div().into_any_element(),
        };
    }

    fn render_response_panel(&self, theme: &Theme) -> impl IntoElement {
        return div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .child(self.response_status_bar(theme))
            .child(self.response_headers_section(theme))
            .child(self.response_body_section(theme));
    }

    fn response_status_bar(&self, theme: &Theme) -> impl IntoElement {
        return div()
            .flex()
            .flex_row()
            .items_center()
            .gap_3()
            .px_4()
            .py_2()
            .border_b(px(1.))
            .border_color(theme.border)
            .child(
                div()
                    .px_2()
                    .py_0p5()
                    .rounded(px(4.))
                    .bg(theme.green.opacity(0.15))
                    .text_color(theme.green)
                    .text_xs()
                    .font_semibold()
                    .child("200 OK"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.muted_foreground)
                    .child("125ms"),
            )
            .child(
                div()
                    .text_xs()
                    .text_color(theme.muted_foreground)
                    .child("342 B"),
            );
    }

    fn response_headers_section(&self, theme: &Theme) -> impl IntoElement {
        return div()
            .flex()
            .flex_col()
            .border_b(px(1.))
            .border_color(theme.border)
            .child(
                div()
                    .px_4()
                    .py_2()
                    .text_xs()
                    .font_semibold()
                    .text_color(theme.muted_foreground)
                    .child("Headers"),
            )
            .child(
                div()
                    .px_4()
                    .py_2()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(response_header_row(
                        "content-type",
                        "application/json",
                        theme,
                    ))
                    .child(response_header_row("server", "nginx/1.21", theme))
                    .child(response_header_row("cache-control", "no-cache", theme)),
            );
    }

    fn response_body_section(&self, theme: &Theme) -> impl IntoElement {
        return div()
            .flex()
            .flex_col()
            .flex_1()
            .min_h_0()
            .child(
                div()
                    .px_4()
                    .py_2()
                    .text_xs()
                    .font_semibold()
                    .text_color(theme.muted_foreground)
                    .child("Body"),
            )
            .child(
                div()
                    .flex_1()
                    .min_h_0()
                    .px_4()
                    .py_2()
                    .text_sm()
                    .text_color(theme.muted_foreground)
                    .child("{\n  \"id\": 1,\n  \"name\": \"John Doe\",\n  \"email\": \"john@example.com\"\n}"),
            );
    }
}

impl Render for RequestPanel {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();

        return div()
            .flex()
            .flex_col()
            .size_full()
            .bg(theme.background)
            .text_color(theme.foreground)
            .child(self.render_top_bar(&theme, cx))
            .child(
                v_resizable("request-response")
                    .child(resizable_panel().child(self.render_request_panel(&theme, cx)))
                    .child(resizable_panel().child(self.render_response_panel(&theme))),
            );
    }
}

fn response_header_row(key: &str, value: &str, theme: &Theme) -> impl IntoElement {
    return div()
        .flex()
        .flex_row()
        .gap_2()
        .text_xs()
        .child(
            div()
                .text_color(theme.foreground)
                .font_semibold()
                .child(SharedString::from(key)),
        )
        .child(
            div()
                .text_color(theme.muted_foreground)
                .child(SharedString::from(value)),
        );
}
