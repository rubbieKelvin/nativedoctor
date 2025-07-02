use components_lib::{
    border::{Border, BorderStyleVariant},
    button::{Button, ButtonSizeVariant, ButtonStyleVariant},
    label::{Label, LabelSizeVariant},
    pane::{Pane, PaneStyleVariant},
    tabs::{TabItemData, TabSet},
};
use dioxus::prelude::*;
use dioxus_free_icons::{icons::ld_icons::LdEllipsisVertical, Icon};

use crate::{
    session::{RequestDefination, Session},
    views::project::utils::{get_label_style_for_method, WorkspaceTab},
};

#[component]
pub fn RequestList(class: Option<String>, tabs: Signal<TabSet<WorkspaceTab>>) -> Element {
    let session = use_context::<Signal<Session>>();

    return rsx! {
        div { class,
            for request in session().requests {
                RequestListItem { tabs, request }
            }
        }
    };
}

#[component]
fn RequestListItem(request: RequestDefination, tabs: Signal<TabSet<WorkspaceTab>>) -> Element {
    let method_style = get_label_style_for_method(&request.method);

    return rsx! {
        Pane {
            class: "flex gap-2 px-2 items-center group/requestitem hover:bg-[#202020] py-1",
            style: PaneStyleVariant::Transparent,
            border: Border::bottom().with_style(BorderStyleVariant::Mild),
            onclick: {
                let request = request.clone();
                move |_| {
                    let mut tabs = tabs.write();
                    let tabdata = TabItemData::new(WorkspaceTab::Request(request.clone()));
                    let similar = tabs.get_similar(&tabdata).cloned();
                    if let Some(tabdata) = similar {
                        tabs.select(Some(tabdata.id));
                        return;
                    }
                    let id = tabdata.id.clone();
                    tabs.add_tab(tabdata);
                    tabs.select(Some(id));
                }
            },
            Label {
                class: "uppercase w-10",
                size: LabelSizeVariant::Small,
                style: method_style,
                "{request.method}"
            }
            Label { class: "flex-grow", "{request.name}" }
            Button {
                class: "opacity-0 group-hover/requestitem:opacity-100",
                style: ButtonStyleVariant::Transparent,
                size: ButtonSizeVariant::Icon,
                Icon {
                    width: 16,
                    height: 16,
                    class: "text-white",
                    icon: LdEllipsisVertical,
                }
            }
        }
    };
}
