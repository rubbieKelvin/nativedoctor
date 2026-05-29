use std::path::PathBuf;

use gpui::*;
use gpui_component::{
    button::{self, ButtonVariants},
    dialog::DialogButtonProps,
    input, ActiveTheme, IconName, Sizable, WindowExt,
};
use nd_core::model::project::ProjectFile;

/// State backing the "Create Project" dialog.
///
/// Holds the project name input and the base-path (folder) selection so the
/// dialog can validate both before delegating to
/// [`ProjectFile::create_in_path`].
pub struct CreateProjectState {
    pub project_name: Entity<input::InputState>,
    pub base_path: Entity<SharedString>,
}

pub enum CreateProjectEvent {
    ProjectFileCreated(PathBuf),
}

impl EventEmitter<CreateProjectEvent> for CreateProjectState {}

impl CreateProjectState {
    fn on_created(&self, path: PathBuf, cx: &mut Context<Self>) {
        cx.emit(CreateProjectEvent::ProjectFileCreated(path));
    }
}

impl CreateProjectState {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let project_name =
            cx.new(|cx| input::InputState::new(window, cx).placeholder("my-project"));
        let base_path = cx.new(|_cx| SharedString::from(""));

        return Self {
            project_name,
            base_path,
        };
    }
}

fn path_input(cx: &mut App, state: Entity<CreateProjectState>) -> impl IntoElement {
    let theme = cx.theme();
    let _state = state.read(cx);

    return div()
        .flex()
        .flex_row()
        .gap_2()
        .items_center()
        .border(px(1.))
        .border_color(theme.border)
        .rounded_md()
        .px_1()
        .child(
            div()
                .flex_1()
                .p_1()
                .text_sm()
                .text_color(if _state.base_path.read(cx).is_empty() {
                    theme.muted_foreground
                } else {
                    theme.foreground
                })
                .child(if _state.base_path.read(cx).is_empty() {
                    SharedString::from("Select a folder...")
                } else {
                    _state.base_path.read(cx).clone()
                }),
        )
        .child({
            let state = state.clone();
            button::Button::new(SharedString::from("browse-folder"))
                .small()
                .with_variant(button::ButtonVariant::Secondary)
                .icon(IconName::FolderOpen)
                .on_click(move |_event, _window, cx| {
                    let state = state.clone();
                    cx.spawn(|cx: &mut AsyncApp| {
                        let cx = (*cx).clone();
                        async move {
                            let result: Option<std::path::PathBuf> = cx.background_executor().spawn(async move {
                                rfd::FileDialog::new().pick_folder()
                            }).await;

                            if let Some(path) = result {
                                cx.update(|cx| {
                                    let path_str: SharedString = path.to_string_lossy().to_string().into();
                                    state.update(cx, |s, cx| {
                                        s.base_path.update(cx, |v, cx| {
                                            *v = path_str;
                                            cx.notify();
                                        });
                                    });
                                });
                            }
                        }
                    }).detach();
                })
        });
}

fn render_content(cx: &mut App, state: Entity<CreateProjectState>) -> impl IntoElement {
    let theme = cx.theme();
    let _state = state.read(cx);

    return div()
        .flex()
        .flex_col()
        .gap_4()
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.foreground)
                        .child("Project Name"),
                )
                .child(input::Input::new(&_state.project_name)),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(theme.foreground)
                        .child("Base Path"),
                )
                .child(path_input(cx, state)),
        );
}

/// Open the "Create Project" dialog as a modal over the given window.
///
/// On successful creation, the dialog emits
/// [`CreateProjectEvent::ProjectFileCreated`] with the path to the
/// newly written `nativedoctor.yaml`.
pub fn open_create_project(state: Entity<CreateProjectState>, window: &mut Window, cx: &mut App) {
    window.open_dialog(cx, move |dialog, _window, _cx| {
        dialog
            .title("Create Project")
            .w(px(420.))
            .content({
                let state = state.clone();
                move |content, _window, cx| content.child(render_content(cx, state.clone()))
            })
            .button_props(
                DialogButtonProps::default()
                    .ok_text("Create")
                    .cancel_text("Cancel")
                    .show_cancel(true)
                    .on_ok({
                        let state_mut = state.clone();
                        let state = state.clone();

                        move |_event, _window, cx| {
                            let state = state.read(cx);
                            let name = state.project_name.read(cx).value();
                            let base = state.base_path.read(cx).clone();

                            let name = name.trim();
                            let base = base.trim();

                            if name.is_empty() || base.is_empty() {
                                return false;
                            }

                            let project_dir = std::path::Path::new(base).join(name);
                            let project_file = project_dir.join("nativedoctor.yaml");

                            // Delegate directory creation, sample files, README, and
                            // YAML serialisation to the shared create_project helper.
                            if ProjectFile::create_in_path(PathBuf::from(base), name.to_string())
                                .is_err()
                            {
                                return false;
                            }

                            state_mut.update(cx, |this, cx| {
                                this.on_created(project_file, cx);
                            });

                            return true;
                        }
                    }),
            )
    });
}
