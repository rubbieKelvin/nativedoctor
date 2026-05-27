use std::path::PathBuf;

use gpui::{prelude::FluentBuilder, *};
use gpui_component::{
    button::{self, ButtonVariants},
    input,
    popover::{Popover, PopoverState},
    scroll::ScrollableElement,
    ActiveTheme, Icon, IconName, Sizable,
};

pub struct ProjectPopupState {
    pub recent_projects: Vec<RecentProject>,
    pub active_project_name: SharedString,
    search_state: Entity<input::InputState>,
    is_open: bool,
}

#[derive(Debug, Clone)]
pub enum ProjectPopupEvents {
    OpenProject(PathBuf),
}

impl EventEmitter<ProjectPopupEvents> for ProjectPopupState {}

#[derive(Clone)]
pub struct RecentProject {
    pub name: SharedString,
    pub path: SharedString,
}

impl ProjectPopupState {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let search_state =
            cx.new(|cx| input::InputState::new(window, cx).placeholder("Search projects..."));

        return Self {
            recent_projects: Vec::new(),
            active_project_name: "No project".into(),
            search_state,
            is_open: false,
        };
    }

    #[allow(dead_code)]
    pub fn set_recent_projects(&mut self, projects: Vec<RecentProject>) {
        self.recent_projects = projects;
    }

    #[allow(dead_code)]
    pub fn set_active_project(&mut self, name: impl Into<SharedString>) {
        self.active_project_name = name.into();
    }

    // // Here path is the path to the project file.
    // fn open_project(&mut self, _path: PathBuf, creating: bool) {
    //     // emit event here
    //     self.is_open = false;
    // }
    //

    fn select_project(&mut self, cx: &mut Context<Self>, path: PathBuf) {
        cx.emit(ProjectPopupEvents::OpenProject(path));
        self.is_open = false;
    }

    fn open_project(&mut self, cx: &mut Context<Self>) {
        // open a folder and ask the user to select a project file
        // then emit the project file
        let project_file = PathBuf::new();
        cx.emit(ProjectPopupEvents::OpenProject(project_file));
        self.is_open = false;
    }

    fn create_project(&mut self, cx: &mut Context<Self>) {
        // open a dialog that ask the user to input a path and a name for the project
        let project_file = PathBuf::new();
        cx.emit(ProjectPopupEvents::OpenProject(project_file));
        self.is_open = false;
    }

    fn filtered(&self, cx: &App) -> Vec<(usize, RecentProject)> {
        let query = self.search_state.read(cx).value();
        if query.is_empty() {
            return self
                .recent_projects
                .iter()
                .enumerate()
                .map(|(i, p)| (i, p.clone()))
                .collect();
        }

        let q = query.to_lowercase();
        self.recent_projects
            .iter()
            .enumerate()
            .filter(|(_, p)| p.name.to_lowercase().contains(&q))
            .map(|(i, p)| (i, p.clone()))
            .collect()
    }
}

#[derive(IntoElement)]
pub struct ProjectPopup {
    state: Entity<ProjectPopupState>,
}

impl ProjectPopup {
    pub fn new(state: Entity<ProjectPopupState>) -> Self {
        Self { state }
    }

    fn render_search_bar(&self, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let search_state = self.state.read(cx).search_state.clone();

        return div()
            .child(input::Input::new(&search_state).appearance(false))
            .border_b_1()
            .border_color(theme.border);
    }

    fn render_recent_projects(
        &self,
        cx: &mut App,
        popover_entity: Entity<PopoverState>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let state = self.state.clone();
        let filtered = state.read(cx).filtered(cx).clone();

        return div()
            .flex()
            .flex_col()
            .max_h(px(240.))
            .overflow_y_scrollbar()
            .when(filtered.is_empty(), |this| {
                this.child(
                    div()
                        .p_4()
                        .text_xs()
                        .text_color(theme.muted_foreground)
                        .child("No recent projects"),
                )
            })
            .children(filtered.iter().map(|(_, project)| {
                let name = project.name.clone();
                let path = project.path.clone();
                let state = state.clone();
                let popover_entity = popover_entity.clone();

                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .px_3()
                    .py_2()
                    .hover(|style| style.bg(theme.muted))
                    .cursor_pointer()
                    .child(Icon::new(IconName::Folder).small())
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .child(
                                div()
                                    .text_sm()
                                    .text_color(theme.foreground)
                                    .child(name.clone()),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme.muted_foreground)
                                    .child(path.clone()),
                            ),
                    )
                    .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                        state.update(cx, |this, cx| {
                            this.select_project(cx, PathBuf::from(path.as_str()));
                        });

                        popover_entity.update(cx, |s, cx| {
                            s.dismiss(window, cx);
                        });
                    })
            }));
    }

    fn render_footer(
        &self,
        cx: &mut App,
        popover_entity: Entity<PopoverState>,
    ) -> impl IntoElement {
        let theme = cx.theme();
        let state = self.state.clone();

        return div()
            .flex()
            .flex_row()
            .gap_2()
            .p_2()
            .border_t_1()
            .border_color(theme.border)
            .child(
                button::Button::new(SharedString::from("create-project"))
                    .label("Create Project")
                    .small()
                    .with_variant(button::ButtonVariant::Ghost)
                    .icon(IconName::Plus)
                    .on_click({
                        let state = state.clone();
                        let popover_entity = popover_entity.clone();

                        move |_event, window, cx| {
                            popover_entity.update(cx, |s, cx| {
                                s.dismiss(window, cx);
                            });

                            state.update(cx, |this, cx| {
                                // TODO: empty path buf for now, improve later
                                this.create_project(cx);
                            });
                        }
                    }),
            )
            .child(
                button::Button::new(SharedString::from("open-project"))
                    .label("Open Project")
                    .small()
                    .with_variant(button::ButtonVariant::Ghost)
                    .icon(IconName::FolderOpen)
                    .on_click({
                        let state = state.clone();
                        let popover_entity = popover_entity.clone();

                        move |_event, window, cx| {
                            popover_entity.update(cx, |s, cx| {
                                s.dismiss(window, cx);
                            });

                            state.update(cx, |this, cx| {
                                this.open_project(cx);
                            });
                        }
                    }),
            );
    }

    fn render_popup_content(
        &self,
        cx: &mut App,
        popover_state: Entity<PopoverState>,
    ) -> impl IntoElement {
        return div()
            .w_72()
            .flex()
            .flex_col()
            .child(self.render_search_bar(cx))
            .child(self.render_recent_projects(cx, popover_state.clone()))
            .child(self.render_footer(cx, popover_state.clone()));
    }
}

impl RenderOnce for ProjectPopup {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = self.state.clone();
        let active_name = state.read(cx).active_project_name.clone();

        Popover::new("project-popup")
            .anchor(Anchor::TopLeft)
            .open(state.read(cx).is_open)
            .p_0()
            .on_open_change({
                let state = state.clone();
                move |is_open, _window, cx| {
                    state.update(cx, |this, cx| {
                        this.is_open = *is_open;
                        cx.notify();
                    });
                }
            })
            .trigger(
                button::Button::new(SharedString::from("project-button"))
                    .label(active_name)
                    .small()
                    .with_variant(button::ButtonVariant::Text)
                    .icon(IconName::Folder),
            )
            .content(move |_, _window, cx| {
                let popover_entity = cx.entity();

                return self.render_popup_content(cx, popover_entity);
            })
    }
}
