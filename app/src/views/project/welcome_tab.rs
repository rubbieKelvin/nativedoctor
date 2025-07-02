use components_lib::{
    border::Border,
    button::{Button, ButtonSizeVariant, ButtonStyleVariant},
    label::{Label, LabelSizeVariant, LabelStyleVariant},
    pane::Pane,
    tabs::{TabItemData, TabState},
};
use dioxus::prelude::*;
use dioxus_free_icons::{
    icons::ld_icons::{LdCircleHelp, LdGithub, LdMail, LdPencil, LdPlus, LdTwitter},
    Icon,
};

use crate::{session::Session, views::project::utils::WorkspaceTab};

#[component]
pub fn WelcomePage() -> Element {
    let state = use_context::<TabState<WorkspaceTab>>();
    let mut tabset = state.tabs;
    let mut session = use_context::<Signal<Session>>();

    return rsx! {
        div { class: "h-full flex items-center justify-center",
            div { class: "flex flex-col gap-4 min-w-[40%]",

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
                            "Let's f*#ing go!"
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
                        onclick: move |_| {
                            let mut session = session.write();
                            let created_defination = session.new_empty_request();
                            let mut tabset = tabset.write();
                            let tabitem = TabItemData::new(WorkspaceTab::Request(created_defination));
                            tabset.add_tab(tabitem.clone());
                            tabset.select(Some(tabitem.id));
                        },
                        Icon { icon: LdPlus, height: 14, width: 14 }
                        Label { class: "flex-grow text-start", "Add a request" }
                        Label { style: LabelStyleVariant::Mild, size: LabelSizeVariant::Small, "Ctrl+O" }
                    }
                    Button {
                        style: ButtonStyleVariant::Ghost,
                        class: "flex gap-2 items-center",
                        Icon { icon: LdPencil, height: 14, width: 14 }
                        Label { "Edit environment" }
                    }
                    Button {
                        style: ButtonStyleVariant::Ghost,
                        class: "flex gap-2 items-center",
                        Icon { icon: LdCircleHelp, height: 14, width: 14 }
                        Label { class: "flex-grow text-start", "Help" }
                        Label { style: LabelStyleVariant::Mild, size: LabelSizeVariant::Small, "?" }
                    }
                }
            }
        }
    };
}
