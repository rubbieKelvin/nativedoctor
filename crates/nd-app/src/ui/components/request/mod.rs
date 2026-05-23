mod request_tab_auth;
mod request_tab_body;
mod request_tab_docs;
mod request_tab_headers;
mod request_tab_params;
mod request_tab_settings;
mod response_tab_body;
mod response_tab_headers;
mod response_tab_logs;
mod response_tab_timeline;

use gpui::*;
use gpui_component::{
    button, input,
    resizable::{resizable_panel, v_resizable},
    select::{self, SelectState},
    tab::{Tab, TabBar},
    ActiveTheme, IndexPath, Sizable, StyledExt, Theme,
};

use crate::ui::components::{kvinput, number_input};

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
    active_response_tab: usize,
    body_type: usize,
    raw_sub_type: usize,
    binary_path: SharedString,
    form_data_state: Entity<kvinput::KvInputState>,
    url_encoded_state: Entity<kvinput::KvInputState>,
    ssl_verify: bool,
    follow_redirects: bool,
    http_version: usize,
    timeout_state: Entity<number_input::NumberInputState>,
    max_redirects_state: Entity<number_input::NumberInputState>,
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

        let timeout_state = cx.new(|_cx| {
            number_input::NumberInputState::new(30, 1, 300, Some(SharedString::new("s")))
        });
        let max_redirects_state =
            cx.new(|_cx| number_input::NumberInputState::new(10, 0, 50, None));

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
            active_response_tab: 0,
            body_type: 0,
            raw_sub_type: 0,
            binary_path: SharedString::default(),
            form_data_state,
            url_encoded_state,
            ssl_verify: true,
            follow_redirects: true,
            http_version: 0,
            timeout_state,
            max_redirects_state,
        };
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
            .child(button::Button::new("send-request-btn").label("Send"));
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

    fn render_tab_bar(&mut self, _theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let active = self.active_tab;

        return TabBar::new("request-tabs")
            .underline()
            .selected_index(active)
            .on_click(cx.listener(move |this, &index, _window, _cx| {
                this.active_tab = index;
            }))
            .child(Tab::new().label("Docs").small())
            .child(Tab::new().label("Params").small())
            .child(Tab::new().label("Auth").small())
            .child(Tab::new().label("Headers").small())
            .child(Tab::new().label("Body").small())
            .child(Tab::new().label("Settings").small())
            .px_2();
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
                &self.timeout_state,
                &self.max_redirects_state,
                theme,
                cx,
            )
            .into_any_element(),
            _ => div().into_any_element(),
        };
    }

    fn render_response_panel(&mut self, theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        return div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .child(
                div()
                    .px_2()
                    .flex()
                    .border_b(px(1.))
                    .border_color(theme.border)
                    .items_center()
                    .justify_between()
                    .child(self.response_tab_bar(theme, cx))
                    .child(self.response_status_bar(theme)),
            )
            .child(response_tab_content(self.active_response_tab, theme));
    }

    fn response_tab_bar(&mut self, _theme: &Theme, cx: &mut Context<Self>) -> impl IntoElement {
        let active = self.active_response_tab;

        return TabBar::new("response-tabs")
            .selected_index(active)
            .underline()
            .on_click(cx.listener(move |this, &index, _window, _cx| {
                this.active_response_tab = index;
            }))
            .child(Tab::new().label("Body").small())
            .child(Tab::new().label("Headers").small())
            .child(Tab::new().label("Logs").small());
    }

    fn response_status_bar(&self, theme: &Theme) -> impl IntoElement {
        return div()
            .flex()
            .flex_row()
            .items_center()
            .gap_3()
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
}

fn response_tab_content(active_tab: usize, theme: &Theme) -> AnyElement {
    match active_tab {
        0 => response_tab_body::render(theme).into_any_element(),
        1 => response_tab_headers::render(theme).into_any_element(),
        2 => response_tab_logs::render(theme).into_any_element(),
        3 => response_tab_timeline::render(theme).into_any_element(),
        _ => div().into_any_element(),
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
                    .child(
                        resizable_panel()
                            .child(self.render_request_panel(&theme, cx))
                            .bg(theme.background),
                    )
                    .child(
                        resizable_panel()
                            .child(self.render_response_panel(&theme, cx))
                            .bg(theme.background),
                    ),
            );
    }
}
