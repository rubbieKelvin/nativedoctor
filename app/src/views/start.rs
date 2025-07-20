use components_lib::border::Border;
use components_lib::button::{Button, ButtonSizeVariant, ButtonStyleVariant};
use components_lib::prelude::HorizontalSeparator;
use dioxus::prelude::*;
use dioxus_free_icons::icons::ld_icons::{
    LdCircleHelp, LdFileBox, LdFolderOpen, LdFolderPlus, LdGithub, LdMail, LdTwitter
};
use dioxus_free_icons::Icon;

use crate::components::WmDragArea;
use crate::session::Session;
use crate::PageScreen;
use components_lib::label::{Label, LabelSizeVariant, LabelStyleVariant};
use components_lib::pane::{Pane, PaneStyleVariant};

#[component]
pub fn StartScreenView() -> Element {
    let mut screen_state = use_context::<Signal<PageScreen>>();

    let create_new_project = move |_: Event<MouseData>| {
        screen_state.set(PageScreen::ProjectScreen(Session::template()));
    };

    return rsx! {
        WmDragArea { class: "h-full",
            Pane {
                class: "h-full flex items-center justify-center",
                style: PaneStyleVariant::Darker,

                div { class: "flex flex-col gap-4 min-w-[30%]",

                    div { class: "flex items-center",
                        div { class: "flex-grow",
                            Label {
                                size: LabelSizeVariant::Large,
                                style: LabelStyleVariant::Mild,
                                "Native Doctor"
                            }

                            Label {
                                size: LabelSizeVariant::Small,
                                style: LabelStyleVariant::Ghost,
                                "Let's f*#ing go! ・ Initialize project"
                            }
                        }

                        div { class: "flex gap-4 items-center",
                            Button {
                                size: ButtonSizeVariant::Icon,
                                style: ButtonStyleVariant::Ghost,
                                class: "flex gap-2 items-center",
                                Icon { icon: LdTwitter, height: 14, width: 14 }
                            }

                            Button {
                                size: ButtonSizeVariant::Icon,
                                style: ButtonStyleVariant::Ghost,
                                class: "flex gap-2 items-center",
                                Icon { icon: LdGithub, height: 14, width: 14 }
                            }

                            Button {
                                size: ButtonSizeVariant::Icon,
                                style: ButtonStyleVariant::Ghost,
                                class: "flex gap-2 items-center",
                                Icon { icon: LdMail, height: 14, width: 14 }
                            }
                        }
                    }

                    // buttons
                    Pane {
                        class: "rounded-md flex flex-col p-2 gap-2",
                        border: Border::all(),

                        Button {
                            style: ButtonStyleVariant::Ghost,
                            class: "flex gap-2 items-center",
                            onclick: create_new_project,
                            Icon { icon: LdFolderPlus, height: 14, width: 14 }
                            Label { class: "flex-grow text-start", "Create project" }
                            Label {
                                style: LabelStyleVariant::Mild,
                                size: LabelSizeVariant::Small,
                                "Ctrl+N"
                            }
                        }
                        Button {
                            style: ButtonStyleVariant::Ghost,
                            class: "flex gap-2 items-center",
                            Icon { icon: LdFolderOpen, height: 14, width: 14 }
                            Label { class: "flex-grow text-start", "Open project" }
                            Label {
                                style: LabelStyleVariant::Mild,
                                size: LabelSizeVariant::Small,
                                "Ctrl+O"
                            }
                        }
                        RecentProjects {}

                        Button {
                            style: ButtonStyleVariant::Ghost,
                            class: "flex gap-2 items-center",
                            Icon { icon: LdCircleHelp, height: 14, width: 14 }
                            Label { class: "flex-grow text-start", "Help" }
                            Label {
                                style: LabelStyleVariant::Mild,
                                size: LabelSizeVariant::Small,
                                "?"
                            }
                        }
                    }
                }
            }
        }
    };
}

#[component]
fn RecentProjects() -> Element {
    return rsx! {
        div { class: "flex gap-2 items-center",
            Label {
                size: LabelSizeVariant::Small,
                style: LabelStyleVariant::Mild,
                class: "text-nowrap",
                "Recent projects"
            }
            HorizontalSeparator {}
        }

        Button {
            style: ButtonStyleVariant::Ghost,
            class: "flex gap-2 items-center",

            Icon { icon: LdFileBox, height: 14, width: 14 }
            Label { class: "flex-grow text-start", "Untitled" }
            Label { style: LabelStyleVariant::Mild, "/Users/rubbiekelvin/Desktop/Untitled/nativedoctor.nd-project" }
        }

        HorizontalSeparator {}
    };
}