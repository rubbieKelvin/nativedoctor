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

use std::collections::HashMap;

use gpui::*;
use gpui_component::{
    button, input,
    resizable::{resizable_panel, v_resizable},
    select::{self, SelectState},
    tab::{Tab, TabBar},
    ActiveTheme, IndexPath, Sizable, StyledExt, Theme,
};
use nd_core::model::request::{RequestBody, RequestFile};

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

    pub fn load_from_request(
        &mut self,
        request: &RequestFile,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // URL
        self.url_input_state.update(cx, |state, cx| {
            state.set_value(SharedString::from(request.request.url.clone()), window, cx);
        });

        // Method — TODO: wire up SelectState index selection when API is available.
        // Currently the SelectState from gpui_component doesn't expose a public
        // `select(idx)` or `items()` API, so the method defaults to GET.

        // Body
        if let Some(body) = &request.request.body {
            let body_str = match body {
                RequestBody::Json(v) => {
                    serde_json::to_string_pretty(v).unwrap_or_default()
                }
                RequestBody::Text(s) => s.clone(),
                RequestBody::Structured(s) => s.content.to_string(),
            };
            self.body_text_state.update(cx, |state, cx| {
                state.set_value(SharedString::from(body_str), window, cx);
            });
        }

        // Headers
        self.headers_input_state.update(cx, |state, cx| {
            state.clear();
            for (key, value) in &request.request.headers {
                state.new_row(cx, window, key, value, "");
            }
        });

        // Query params
        self.param_input_state.update(cx, |state, cx| {
            state.clear();
            for (key, value) in &request.request.query {
                state.new_row(cx, window, key, value, "");
            }
        });

        // Docs / description
        self.docs_input_state.update(cx, |state, cx| {
            state.set_value(
                SharedString::from(
                    request
                        .request
                        .description
                        .clone()
                        .unwrap_or_default(),
                ),
                window,
                cx,
            );
        });

        // Settings
        self.ssl_verify = request.request.verify_tls;
        self.follow_redirects = request.request.follow_redirects;
        if let Some(timeout) = request.request.timeout_secs {
            self.timeout_state.update(cx, |state, _cx| {
                state.set_value(timeout as i32);
            });
        }

        cx.notify();
    }

    pub fn collect_request_data(&self, cx: &mut Context<Self>) -> RequestFile {
        let url = self.url_input_state.read(cx).value().to_string();
        let method = self
            .method_state
            .read(cx)
            .selected_value()
            .cloned()
            .unwrap_or_else(|| SharedString::from("GET"))
            .to_string();

        let body_text = self.body_text_state.read(cx).value();
        let body = if body_text.trim().is_empty() {
            None
        } else if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body_text) {
            Some(RequestBody::Json(json))
        } else {
            Some(RequestBody::Text(body_text.to_string()))
        };

        let headers: HashMap<String, String> = self
            .headers_input_state
            .read(cx)
            .entries(cx)
            .into_iter()
            .collect();

        let query: HashMap<String, String> = self
            .param_input_state
            .read(cx)
            .entries(cx)
            .into_iter()
            .collect();

        let description = self.docs_input_state.read(cx).value().to_string();
        let description = if description.is_empty() {
            None
        } else {
            Some(description)
        };

        let timeout = Some(self.timeout_state.read(cx).value() as u64);

        RequestFile {
            name: Some("Saved request".into()),
            request: nd_core::model::request::HttpRequestSpec {
                method,
                url,
                summary: None,
                description,
                tags: vec![],
                deprecated: false,
                query,
                headers,
                body,
                timeout_secs: timeout,
                follow_redirects: self.follow_redirects,
                verify_tls: self.ssl_verify,
            },
            version: nd_constants::DOCUMENT_DEFAULT_VERSION.into(),
            _path: None,
        }
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
