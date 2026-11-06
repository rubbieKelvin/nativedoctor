//! One request tab: the form, its run, and the response panel.

use std::path::{Path, PathBuf};

use gpui_kit::component::button::*;
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
use crate::runtime;

actions!(request, [Send, Save]);

pub const KEY_CONTEXT: &str = "RequestView";
const METHODS: [&str; 7] = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];

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

        cx.subscribe_in(&url, window, |this, state, event: &InputEvent, _, cx| {
            if matches!(event, InputEvent::Change) {
                this.draft.http.url = state.read(cx).value().to_string();
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
                cx.notify();
            },
        )
        .detach();

        return RequestView {
            document,
            draft,
            place,
            url,
            method,
            body,
            headers,
            sent,
            exchange: Exchange::default(),
            tab: ResponseTab::Body,
            pretty: true,
            load_error,
            notice: None,
            running: None,
            _task: None,
        };
    }

    pub fn title(&self) -> String {
        return match &self.document.path {
            Some(path) => self.draft.display_name(path),
            None => self.draft.name.clone().unwrap_or_else(|| "Untitled".to_string()),
        };
    }

    pub fn is_dirty(&self) -> bool {
        return self.document.is_dirty(&self.draft);
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

    fn save(&mut self, cx: &mut Context<Self>) {
        self.notice = Some(match self.document.save(&self.draft) {
            Ok(()) => "Saved".to_string(),
            Err(error) => error.to_string(),
        });
        cx.notify();
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
            .on_action(cx.listener(|this, _: &Save, _, cx| this.save(cx)))
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
                            .on_click(cx.listener(|this, _, _, cx| this.save(cx))),
                    ),
            )
            .child(self.render_status(cx))
            .child(self.render_response(cx));
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

        cx.update_window(fixture.window, |_, _, cx| {
            fixture.view.update(cx, |view, cx| view.save(cx));
        })
        .unwrap();
        let saved = std::fs::read_to_string(&fixture.file).unwrap();
        assert!(saved.contains("# keep me") && saved.contains("# note"), "{saved}");
        assert!(saved.contains("http://example.com/changed"), "{saved}");
        cx.read_entity(&fixture.view, |view, _| assert!(!view.is_dirty()));
    }
}
