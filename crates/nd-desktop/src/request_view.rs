//! One request tab: the form, its run, and the response panel.

use std::path::{Path, PathBuf};

use gpui_kit::component::button::*;
use gpui_kit::component::dialog::DialogButtonProps;
use gpui_kit::component::scroll::ScrollableElement as _;
use gpui_kit::component::input::{EditorState, Editor, Input, InputEvent, InputState};
use gpui_kit::component::select::{Select, SelectEvent, SelectState};
use gpui_kit::component::searchable_list::SearchableVec;
use gpui_kit::component::switch::Switch;
use gpui_kit::component::tab::{Tab, TabBar};
use gpui_kit::component::*;
use gpui_kit::*;
use nd_core::RequestFile;
use tokio::runtime::Handle;
use tokio::sync::mpsc;

use crate::model::document::Document;
use crate::model::exchange::{Exchange, Language};
use crate::model::run::{saved_scope, Outcome, RunInput, Source, Target};
use crate::pairs_view::{PairsChanged, PairsView};
use crate::runtime;

actions!(request, [Send, Save]);

pub const KEY_CONTEXT: &str = "RequestView";
const METHODS: [&str; 7] = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];

#[derive(Clone, Copy, PartialEq)]
enum RequestTab {
    Params,
    Headers,
    Yaml,
}

#[derive(Clone, Copy, PartialEq)]
enum ResponseTab {
    Body,
    Headers,
    Sent,
}

/// What a request needs to know about where it runs.
#[derive(Clone, Default)]
pub struct Place {
    pub root: Option<PathBuf>,
    pub env: Option<String>,
}

pub struct RequestView {
    document: Document,
    draft: RequestFile,
    place: Place,
    url: Entity<InputState>,
    method: Entity<SelectState<SearchableVec<&'static str>>>,
    body: Entity<EditorState>,
    headers: Entity<EditorState>,
    sent: Entity<EditorState>,
    params: Entity<PairsView>,
    request_headers: Entity<PairsView>,
    yaml: Entity<EditorState>,
    /// The YAML tab holds text the form doesn't know about yet.
    yaml_dirty: bool,
    request_tab: RequestTab,
    exchange: Exchange,
    tab: ResponseTab,
    pretty: bool,
    load_error: Option<String>,
    notice: Option<String>,
    running: Option<tokio::task::AbortHandle>,
    _task: Option<Task<()>>,
}

impl RequestView {
    pub fn open(path: &Path, place: Place, window: &mut Window, cx: &mut Context<Self>) -> Self {
        let (document, draft, load_error) = match Document::open(path) {
            Ok((document, Ok(draft))) => (document, draft, None),
            Ok((document, Err(error))) => (document, RequestFile::default(), Some(error.to_string())),
            Err(error) => (Document::scratch(), RequestFile::default(), Some(error.to_string())),
        };
        return RequestView::build(document, draft, place, load_error, window, cx);
    }

    fn build(
        document: Document,
        draft: RequestFile,
        place: Place,
        load_error: Option<String>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let url_text = draft.http.url.clone();
        let url = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("https://api.example.com/users  or  POST :3000/users name=John")
                .default_value(url_text)
        });
        let current = draft.http.method.clone().unwrap_or_else(|| "GET".to_string());
        let index = METHODS.iter().position(|m| *m == current).unwrap_or(0);
        let method = cx.new(|cx| {
            SelectState::new(
                SearchableVec::new(METHODS.to_vec()),
                Some(IndexPath::new(index)),
                window,
                cx,
            )
        });
        let viewer = |window: &mut Window, cx: &mut Context<Self>, language: &'static str| {
            cx.new(|cx| EditorState::new(window, cx).language(language).line_number(false))
        };
        let body = viewer(window, cx, "plaintext");
        let headers = viewer(window, cx, "plaintext");
        let sent = viewer(window, cx, "plaintext");
        let params = {
            let pairs = draft.http.query.clone();
            cx.new(|cx| PairsView::new("params", &pairs, false, window, cx))
        };
        let request_headers = {
            let pairs = draft.http.headers.clone();
            cx.new(|cx| PairsView::new("headers", &pairs, true, window, cx))
        };
        let yaml = cx.new(|cx| EditorState::new(window, cx).language("yaml").line_number(true));
        let request_tab = if load_error.is_some() {
            RequestTab::Yaml
        } else {
            RequestTab::Params
        };

        cx.subscribe_in(&params, window, |this, view, _: &PairsChanged, _, cx| {
            this.draft.http.query = view.read(cx).pairs(cx);
            this.yaml_dirty = false;
            cx.notify();
        })
        .detach();
        cx.subscribe_in(&request_headers, window, |this, view, _: &PairsChanged, _, cx| {
            this.draft.http.headers = view.read(cx).pairs(cx);
            this.yaml_dirty = false;
            cx.notify();
        })
        .detach();
        cx.subscribe_in(&yaml, window, |this, _, event: &InputEvent, _, cx| {
            if matches!(event, InputEvent::Change) {
                this.yaml_dirty = true;
                cx.notify();
            }
        })
        .detach();

        cx.subscribe_in(&url, window, |this, state, event: &InputEvent, _, cx| {
            if matches!(event, InputEvent::Change) {
                this.draft.http.url = state.read(cx).value().to_string();
                this.yaml_dirty = false;
                cx.notify();
            }
        })
        .detach();
        cx.subscribe_in(
            &method,
            window,
            |this, _, event: &SelectEvent<SearchableVec<&'static str>>, _, cx| {
                let SelectEvent::Confirm(Some(method)) = event else {
                    return;
                };
                this.draft.http.method = Some((*method).to_string());
                this.yaml_dirty = false;
                cx.notify();
            },
        )
        .detach();

        let mut view = RequestView {
            document,
            draft,
            place,
            url,
            method,
            body,
            headers,
            sent,
            params,
            request_headers,
            yaml,
            yaml_dirty: false,
            request_tab,
            exchange: Exchange::default(),
            tab: ResponseTab::Body,
            pretty: true,
            load_error,
            notice: None,
            running: None,
            _task: None,
        };
        if view.request_tab == RequestTab::Yaml {
            view.show_yaml(window, cx);
        }
        return view;
    }

    pub fn title(&self) -> String {
        return match &self.document.path {
            Some(path) => self.draft.display_name(path),
            None => self.draft.name.clone().unwrap_or_else(|| "Untitled".to_string()),
        };
    }

    pub fn is_dirty(&self) -> bool {
        return self.yaml_dirty || self.document.is_dirty(&self.draft);
    }

    pub fn is_running(&self) -> bool {
        return self.running.is_some();
    }

    fn send(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.is_running() {
            return;
        }
        let base = self
            .document
            .path
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_default().join("request"));
        let name = self.title();
        let input = RunInput {
            root: self.place.root.clone(),
            env: self.place.env.clone(),
            use_cookies: true,
            targets: vec![Target {
                name,
                source: Source::Draft(Box::new(self.draft.clone())),
                items: Vec::new(),
                files: saved_scope(&base, self.place.root.as_deref()),
            }],
            ..RunInput::default()
        };

        let (sender, mut updates) = mpsc::unbounded_channel();
        let handle: Handle = runtime::handle(cx);
        let job = handle.spawn(crate::model::run::run(input, sender));
        self.running = Some(job.abort_handle());
        self.exchange = Exchange::start();
        self.notice = None;
        self.refresh_viewers(window, cx);

        self._task = Some(cx.spawn_in(window, async move |this, cx| {
            while let Some(update) = updates.recv().await {
                let step = this.update_in(cx, |this, window, cx| {
                    let finished = this.exchange.apply(update);
                    if finished {
                        this.running = None;
                    }
                    this.refresh_viewers(window, cx);
                    cx.notify();
                    finished
                });
                if step.unwrap_or(true) {
                    break;
                }
            }
        }));
        cx.notify();
    }

    fn cancel(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(job) = self.running.take() {
            job.abort();
        }
        self._task = None;
        self.exchange.cancelled();
        self.notice = Some("Cancelled".to_string());
        self.refresh_viewers(window, cx);
        cx.notify();
    }

    /// Fills the YAML tab from the form, unless it holds edits of its own.
    fn show_yaml(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.yaml_dirty {
            return;
        }
        let text = match (&self.load_error, self.document.disk_text()) {
            (Some(_), Some(disk)) => disk.to_string(),
            _ => match self.document.yaml_for(&self.draft) {
                Ok(text) => text,
                // The form can't be applied without losing comments, so show the file as it is.
                Err(_) => self.document.disk_text().unwrap_or_default().to_string(),
            },
        };
        self.yaml.update(cx, |state, cx| state.set_value(text, window, cx));
    }

    /// Replaces the form with a parsed draft, as after saving from the YAML tab.
    fn load_draft(&mut self, draft: RequestFile, window: &mut Window, cx: &mut Context<Self>) {
        let method = draft.http.method.clone().unwrap_or_else(|| "GET".to_string());
        let url = draft.http.url.clone();
        self.url.update(cx, |state, cx| state.set_value(url, window, cx));
        if let Some(method) = METHODS.iter().find(|m| **m == method) {
            self.method
                .update(cx, |state, cx| state.set_selected_value(method, window, cx));
        }
        let (query, headers) = (draft.http.query.clone(), draft.http.headers.clone());
        self.params.update(cx, |view, cx| view.load(&query, window, cx));
        self.request_headers.update(cx, |view, cx| view.load(&headers, window, cx));
        self.draft = draft;
        self.load_error = None;
    }

    fn save(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.yaml_dirty {
            let text = self.yaml.read(cx).value().to_string();
            match self.document.save_yaml(&text) {
                Ok(draft) => {
                    self.load_draft(draft, window, cx);
                    self.yaml_dirty = false;
                    self.notice = Some("Saved".to_string());
                }
                Err(error) => self.notice = Some(error.to_string()),
            }
            cx.notify();
            return;
        }
        match self.document.save(&self.draft) {
            Ok(()) => self.notice = Some("Saved".to_string()),
            Err(nd_core::Error::Unpreserved(reason)) => self.ask_how_to_save(reason, window, cx),
            Err(error) => self.notice = Some(error.to_string()),
        }
        cx.notify();
    }

    /// The form can't be saved without losing comments. Nothing has been written.
    fn ask_how_to_save(&mut self, reason: String, window: &mut Window, cx: &mut Context<Self>) {
        let this = cx.entity().downgrade();
        let rewrite = this.clone();
        window.open_dialog(cx, move |dialog, _, _| {
            let this = this.clone();
            let rewrite = rewrite.clone();
            dialog
                .title("Can't keep this file's comments")
                .child(format!(
                    "{reason}. You can edit the YAML directly, or rewrite the file, which drops its comments."
                ))
                .button_props(
                    DialogButtonProps::default()
                        .ok_text("Rewrite file (loses comments)")
                        .cancel_text("Open YAML tab"),
                )
                .on_ok(move |_, window, cx| {
                    rewrite
                        .update(cx, |view, cx| {
                            view.notice = Some(match view.document.save_rewriting(&view.draft) {
                                Ok(()) => "Saved (comments dropped)".to_string(),
                                Err(error) => error.to_string(),
                            });
                            let _ = window;
                            cx.notify();
                        })
                        .ok();
                    true
                })
                .on_cancel(move |_, window, cx| {
                    this.update(cx, |view, cx| {
                        view.request_tab = RequestTab::Yaml;
                        view.show_yaml(window, cx);
                        cx.notify();
                    })
                    .ok();
                    true
                })
        });
    }

    /// Pushes the exchange's text into the three read-only viewers.
    fn refresh_viewers(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let language = match self.exchange.language() {
            Language::Json => "json",
            Language::Html => "html",
            Language::Plain => "plaintext",
        };
        let body = self.exchange.body_display(self.pretty);
        let headers = self.exchange.headers_display();
        let sent = self.exchange.sent_display();
        self.body.update(cx, |state, cx| {
            if state.language_name().as_ref() != language {
                state.set_highlighter(language, cx);
            }
            state.set_value(body, window, cx);
        });
        self.headers.update(cx, |state, cx| state.set_value(headers, window, cx));
        self.sent.update(cx, |state, cx| state.set_value(sent, window, cx));
    }

    fn render_status(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();
        let color = match (self.exchange.outcome(), self.exchange.head.as_ref()) {
            (Some(Outcome::Error { .. }), _) => theme.danger,
            (Some(Outcome::Assertions(_)), _) | (Some(Outcome::BadStatus(_)), _) => theme.danger,
            (_, Some(head)) if head.status >= 400 => theme.danger,
            (_, Some(head)) if head.status >= 300 => theme.warning,
            (_, Some(_)) => theme.success,
            _ => theme.muted_foreground,
        };
        let mut row = div()
            .h_flex()
            .gap_3()
            .px_3()
            .py_2()
            .child(div().text_color(color).child(self.exchange.status_text()));
        if let Some(notice) = &self.notice {
            row = row.child(div().text_xs().text_color(theme.muted_foreground).child(notice.clone()));
        }
        for failure in self.exchange.failures() {
            row = row.child(div().text_xs().text_color(theme.danger).child(failure.clone()));
        }
        return row;
    }

    fn render_request_side(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = match self.request_tab {
            RequestTab::Params => 0,
            RequestTab::Headers => 1,
            RequestTab::Yaml => 2,
        };
        let count = |n: usize, label: &str| {
            if n == 0 { label.to_string() } else { format!("{label} ({n})") }
        };
        let content = match self.request_tab {
            RequestTab::Params => div().size_full().child(self.params.clone()).into_any_element(),
            RequestTab::Headers => div()
                .size_full()
                .child(self.request_headers.clone())
                .into_any_element(),
            RequestTab::Yaml => Editor::new(&self.yaml).size_full().into_any_element(),
        };
        return div()
            .v_flex()
            .size_full()
            .child(
                TabBar::new("request-side-tabs")
                    .underline()
                    .selected_index(selected)
                    .children([
                        Tab::new().label(count(self.draft.http.query.0.len(), "Params")),
                        Tab::new().label(count(self.draft.http.headers.0.len(), "Headers")),
                        Tab::new().label("YAML"),
                    ])
                    .on_click(cx.listener(|this, index: &usize, window, cx| {
                        this.request_tab = match index {
                            1 => RequestTab::Headers,
                            2 => RequestTab::Yaml,
                            _ => RequestTab::Params,
                        };
                        if this.request_tab == RequestTab::Yaml {
                            this.show_yaml(window, cx);
                        }
                        cx.notify();
                    })),
            )
            .child(div().flex_1().min_h_0().overflow_y_scrollbar().child(content));
    }

    fn render_response(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = match self.tab {
            ResponseTab::Body => 0,
            ResponseTab::Headers => 1,
            ResponseTab::Sent => 2,
        };
        let viewer = match self.tab {
            ResponseTab::Body => &self.body,
            ResponseTab::Headers => &self.headers,
            ResponseTab::Sent => &self.sent,
        };
        return div()
            .v_flex()
            .flex_1()
            .min_h_0()
            .child(
                div()
                    .h_flex()
                    .justify_between()
                    .items_center()
                    .child(
                        TabBar::new("response-tabs")
                            .underline()
                            .selected_index(selected)
                            .children([
                                Tab::new().label("Body"),
                                Tab::new().label("Headers"),
                                Tab::new().label("Sent"),
                            ])
                            .on_click(cx.listener(|this, index: &usize, _, cx| {
                                this.tab = match index {
                                    1 => ResponseTab::Headers,
                                    2 => ResponseTab::Sent,
                                    _ => ResponseTab::Body,
                                };
                                cx.notify();
                            })),
                    )
                    .child(
                        div().h_flex().gap_2().px_3().text_xs().child("Pretty").child(
                            Switch::new("pretty")
                                .checked(self.pretty)
                                .on_click(cx.listener(|this, checked: &bool, window, cx| {
                                    this.pretty = *checked;
                                    this.refresh_viewers(window, cx);
                                    cx.notify();
                                })),
                        ),
                    ),
            )
            .child(div().flex_1().min_h_0().child(Editor::new(viewer).readonly(true).size_full()));
    }
}

impl Render for RequestView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.theme().clone();
        let running = self.is_running();
        let send = if running {
            Button::new("cancel")
                .label("Cancel")
                .on_click(cx.listener(|this, _, window, cx| this.cancel(window, cx)))
        } else {
            Button::new("send")
                .primary()
                .label("Send")
                .on_click(cx.listener(|this, _, window, cx| this.send(window, cx)))
        };
        let mut column = div()
            .key_context(KEY_CONTEXT)
            .on_action(cx.listener(|this, _: &Send, window, cx| this.send(window, cx)))
            .on_action(cx.listener(|this, _: &Save, window, cx| this.save(window, cx)))
            .v_flex()
            .size_full();
        if let Some(error) = &self.load_error {
            column = column.child(
                div()
                    .px_3()
                    .py_2()
                    .text_color(theme.danger)
                    .child(format!("This file can't be edited as a form: {error}")),
            );
        }
        return column
            .child(
                div()
                    .h_flex()
                    .gap_2()
                    .p_3()
                    .child(Select::new(&self.method).w(px(110.)))
                    .child(div().flex_1().child(Input::new(&self.url)))
                    .child(send)
                    .child(
                        Button::new("save")
                            .label("Save")
                            .disabled(self.document.path.is_none() || !self.is_dirty())
                            .on_click(cx.listener(|this, _, window, cx| this.save(window, cx))),
                    ),
            )
            .child(
                div().flex_1().min_h_0().child(
                    v_resizable("request-response")
                        .child(
                            resizable_panel()
                                .size(px(240.))
                                .size_range(px(120.)..px(600.))
                                .child(self.render_request_side(cx)),
                        )
                        .child(
                            resizable_panel().child(
                                div()
                                    .v_flex()
                                    .size_full()
                                    .child(self.render_status(cx))
                                    .child(self.render_response(cx)),
                            ),
                        ),
                ),
            );
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use std::path::PathBuf;

    use gpui_kit::{
        px, size, AnyWindowHandle, App, AppContext as _, Bounds, Entity, Focusable as _, Point,
        TestAppContext, Window, WindowBounds, WindowOptions,
    };
    use gpui_kit::test::TestWindowExt as _;
    use nd_core::Collection;
    use wiremock::matchers::path;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    use super::{Outcome, Place, RequestView};
    use crate::runtime;

    struct Fixture {
        _dir: tempfile::TempDir,
        _net: tokio::runtime::Runtime,
        view: Entity<RequestView>,
        window: AnyWindowHandle,
        file: PathBuf,
    }

    fn fixture(cx: &mut TestAppContext, status: u16, body: &str, delay: Duration) -> Fixture {
        let net = tokio::runtime::Runtime::new().unwrap();
        let server = net.block_on(MockServer::start());
        net.block_on(
            Mock::given(path("/hello"))
                .respond_with(ResponseTemplate::new(status).set_body_string(body).set_delay(delay))
                .mount(&server),
        );
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().canonicalize().unwrap();
        Collection::init(&root, None).unwrap();
        std::fs::write(
            root.join("environments/dev.yaml"),
            format!("vars: {{ BASE_URL: '{}' }}\n", server.uri()),
        )
        .unwrap();
        let file = root.join("hello.request.yaml");
        std::fs::write(
            &file,
            "# keep me\nname: Hello\nhttp:\n  url: ${BASE_URL}/hello # note\n",
        )
        .unwrap();

        cx.update(|cx| {
            gpui_kit::init(cx);
            runtime::init(cx);
        });
        cx.executor().allow_parking();
        let place = Place {
            root: Some(root),
            env: Some("dev".to_string()),
        };
        let path_for_view = file.clone();
        let (window, view) = open_test_window(cx, move |window, cx| {
            cx.new(|cx| RequestView::open(&path_for_view, place, window, cx))
        });
        return Fixture {
            _dir: dir,
            _net: net,
            view,
            window,
            file,
        };
    }

    fn open_test_window(
        cx: &mut TestAppContext,
        build: impl FnOnce(&mut Window, &mut App) -> Entity<RequestView>,
    ) -> (AnyWindowHandle, Entity<RequestView>) {
        return cx.update(|cx| {
            let options = WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(Bounds {
                    origin: Point::default(),
                    size: size(px(900.), px(600.)),
                })),
                ..Default::default()
            };
            gpui_kit::open_window(options, cx, build).expect("test window")
        });
    }

    fn send(cx: &mut TestAppContext, fixture: &Fixture) {
        cx.update_window(fixture.window, |_, window, cx| {
            fixture.view.update(cx, |view, cx| view.send(window, cx));
        })
        .unwrap();
    }

    /// Lets the executor and the network threads run until `done` holds.
    fn settle(cx: &mut TestAppContext, fixture: &Fixture, done: impl Fn(&RequestView) -> bool) {
        let started = Instant::now();
        loop {
            cx.run_until_parked();
            if cx.read_entity(&fixture.view, |view, _| done(view)) {
                return;
            }
            assert!(started.elapsed() < Duration::from_secs(10), "timed out");
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    #[gpui_kit::test]
    fn send_shows_the_response_everywhere(cx: &mut TestAppContext) {
        let fixture = fixture(cx, 200, "hi there", Duration::ZERO);
        send(cx, &fixture);
        settle(cx, &fixture, |view| view.exchange.result.is_some() && !view.is_running());

        cx.read_entity(&fixture.view, |view, cx| {
            assert_eq!(view.exchange.head.as_ref().unwrap().status, 200);
            assert!(view.exchange.status_text().starts_with("200"), "{}", view.exchange.status_text());
            assert_eq!(view.body.read(cx).value().as_ref(), "hi there");
            assert!(view.headers.read(cx).value().contains("content-length"));
            let sent = view.sent.read(cx).value();
            assert!(sent.starts_with("GET /hello"), "{sent}");
        });
    }

    #[gpui_kit::test]
    fn a_failed_request_reports_an_error_not_a_hang(cx: &mut TestAppContext) {
        let fixture = fixture(cx, 200, "", Duration::ZERO);
        cx.update_window(fixture.window, |_, window, cx| {
            fixture.view.update(cx, |view, cx| {
                view.draft.http.url = "http://127.0.0.1:1/".to_string();
                view.send(window, cx);
            });
        })
        .unwrap();
        settle(cx, &fixture, |view| !view.is_running() && view.exchange.result.is_some());
        cx.read_entity(&fixture.view, |view, _| {
            assert!(matches!(view.exchange.outcome(), Some(Outcome::Error { .. })));
        });
    }

    #[gpui_kit::test]
    fn cancel_stops_a_slow_request(cx: &mut TestAppContext) {
        let fixture = fixture(cx, 200, "late", Duration::from_secs(30));
        send(cx, &fixture);
        settle(cx, &fixture, |view| view.is_running());
        cx.update_window(fixture.window, |_, window, cx| {
            fixture.view.update(cx, |view, cx| view.cancel(window, cx));
        })
        .unwrap();
        cx.run_until_parked();
        cx.read_entity(&fixture.view, |view, _| {
            assert!(!view.is_running());
            assert!(!view.exchange.running);
            assert_eq!(view.notice.as_deref(), Some("Cancelled"));
        });
    }

    #[gpui_kit::test]
    fn editing_the_url_marks_dirty_and_save_keeps_comments(cx: &mut TestAppContext) {
        let fixture = fixture(cx, 200, "", Duration::ZERO);
        cx.read_entity(&fixture.view, |view, _| assert!(!view.is_dirty()));

        cx.update_window(fixture.window, |_, window, cx| {
            let url = fixture.view.read(cx).url.clone();
            url.read(cx).focus_handle(cx).focus(window, cx);
            window.render_frame(cx);
            window.press("secondary-a", cx);
            window.input("http://example.com/changed", cx);
        })
        .unwrap();
        cx.run_until_parked();
        cx.read_entity(&fixture.view, |view, _| {
            assert_eq!(view.draft.http.url, "http://example.com/changed");
            assert!(view.is_dirty());
        });

        cx.update_window(fixture.window, |_, window, cx| {
            fixture.view.update(cx, |view, cx| view.save(window, cx));
        })
        .unwrap();
        let saved = std::fs::read_to_string(&fixture.file).unwrap();
        assert!(saved.contains("# keep me") && saved.contains("# note"), "{saved}");
        assert!(saved.contains("http://example.com/changed"), "{saved}");
        cx.read_entity(&fixture.view, |view, _| assert!(!view.is_dirty()));
    }

    #[gpui_kit::test]
    fn a_typed_header_reaches_the_draft_and_the_file(cx: &mut TestAppContext) {
        let fixture = fixture(cx, 200, "", Duration::ZERO);
        cx.update_window(fixture.window, |_, window, cx| {
            fixture
                .view
                .update(cx, |view, cx| {
                    view.request_tab = super::RequestTab::Headers;
                    cx.notify();
                });
            window.render_frame(cx);
            let headers = fixture.view.read(cx).request_headers.clone();
            headers.update(cx, |view, cx| view.add(window, cx));
            window.render_frame(cx);
            window.input("x-new", cx);
            window.press("tab", cx);
            window.input("42", cx);
        })
        .unwrap();
        cx.run_until_parked();
        cx.read_entity(&fixture.view, |view, _| {
            assert_eq!(
                view.draft.http.headers.0,
                vec![("x-new".to_string(), Some("42".to_string()))]
            );
            assert!(view.is_dirty());
        });

        cx.update_window(fixture.window, |_, window, cx| {
            fixture.view.update(cx, |view, cx| view.save(window, cx));
        })
        .unwrap();
        let saved = std::fs::read_to_string(&fixture.file).unwrap();
        assert!(saved.contains("x-new: '42'") || saved.contains("x-new: \"42\"") || saved.contains("x-new: 42"), "{saved}");
        assert!(saved.contains("# keep me") && saved.contains("# note"), "{saved}");
    }

    #[gpui_kit::test]
    fn the_yaml_tab_saves_exactly_what_was_typed(cx: &mut TestAppContext) {
        let fixture = fixture(cx, 200, "", Duration::ZERO);
        let text = "http: {url: 'http://y.test/z'}   # my  spacing";
        cx.update_window(fixture.window, |_, window, cx| {
            fixture.view.update(cx, |view, cx| {
                view.request_tab = super::RequestTab::Yaml;
                view.show_yaml(window, cx);
            });
            let yaml = fixture.view.read(cx).yaml.clone();
            yaml.read(cx).focus_handle(cx).focus(window, cx);
            window.render_frame(cx);
            window.press("secondary-a", cx);
            window.input(text, cx);
        })
        .unwrap();
        cx.run_until_parked();
        cx.read_entity(&fixture.view, |view, _| assert!(view.yaml_dirty && view.is_dirty()));

        cx.update_window(fixture.window, |_, window, cx| {
            fixture.view.update(cx, |view, cx| view.save(window, cx));
        })
        .unwrap();
        cx.run_until_parked();
        assert_eq!(std::fs::read_to_string(&fixture.file).unwrap(), text);
        cx.read_entity(&fixture.view, |view, cx| {
            assert!(!view.yaml_dirty && !view.is_dirty());
            assert_eq!(view.draft.http.url, "http://y.test/z");
            assert_eq!(view.url.read(cx).value().as_ref(), "http://y.test/z");
        });
    }

    #[gpui_kit::test]
    fn broken_yaml_is_refused_and_leaves_the_file_alone(cx: &mut TestAppContext) {
        let fixture = fixture(cx, 200, "", Duration::ZERO);
        let before = std::fs::read_to_string(&fixture.file).unwrap();
        cx.update_window(fixture.window, |_, window, cx| {
            fixture.view.update(cx, |view, cx| {
                view.request_tab = super::RequestTab::Yaml;
                view.show_yaml(window, cx);
            });
            let yaml = fixture.view.read(cx).yaml.clone();
            yaml.read(cx).focus_handle(cx).focus(window, cx);
            window.render_frame(cx);
            window.press("secondary-a", cx);
            window.input("http: [oops", cx);
            fixture.view.update(cx, |view, cx| view.save(window, cx));
        })
        .unwrap();
        assert_eq!(std::fs::read_to_string(&fixture.file).unwrap(), before);
        cx.read_entity(&fixture.view, |view, _| {
            assert!(view.yaml_dirty, "the edit must survive a refused save");
            assert!(view.notice.is_some());
        });
    }
}
