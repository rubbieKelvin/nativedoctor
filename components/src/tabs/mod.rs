use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdX};

use crate::{
    button::{Button, ButtonSizeVariant, ButtonStyleVariant},
    label::{Label, LabelSizeVariant, LabelStyleVariant},
    traits::Variant,
};

#[derive(Clone, Default, PartialEq)]
pub struct TabString(pub String);

impl Into<String> for TabString {
    fn into(self) -> String {
        return self.0;
    }
}

#[derive(Clone, PartialEq)]
pub struct TabItemData<T: PartialEq + Clone + Into<TabString>> {
    id: uuid::Uuid,
    item: T,
    closable: bool,
}

impl<T: PartialEq + Clone + Into<TabString>> TabItemData<T> {
    pub fn new(item: T) -> Self {
        return TabItemData {
            id: uuid::Uuid::new_v4(),
            item,
            closable: true,
        };
    }
}

#[allow(unused)]
#[derive(Clone, PartialEq, Default)]
pub enum TabOrientationVariant {
    #[default]
    Horizontal,
    Vertical,
}

impl Variant for TabOrientationVariant {
    fn classes(&self) -> &'static str {
        return match self {
            TabOrientationVariant::Horizontal => "flex-row",
            TabOrientationVariant::Vertical => "flex-col",
        };
    }
}

// this will be passed to the pills and the pages as contexts
#[derive(Clone, PartialEq)]
pub struct TabState<T: PartialEq + Clone + Into<TabString> + 'static> {
    tab: TabItemData<T>,
    tabs: Signal<Vec<TabItemData<T>>>,
    selected_tab: Signal<Option<uuid::Uuid>>,
}

impl<T: PartialEq + Clone + Into<TabString> + 'static> TabState<T> {
    #[allow(unused)]
    fn remove(&self){
        let tab_id = self.tab.id.clone();
        let mut tabs = self.tabs;
        tabs.with_mut(|tabs| {
            let ids = tabs.iter().map(|t| t.id).collect::<Vec<uuid::Uuid>>();
            tracing::info!("{:?} {:?}", ids, tab_id);
            tabs.retain(|t| t.id != tab_id);
        });
    }
}

#[component]
pub fn TabManager<T: PartialEq + Clone + Into<TabString> + 'static>(
    tabs: Signal<Vec<TabItemData<T>>>,
    pill: Option<Element>,
    class: Option<String>,
    orientation: Option<TabOrientationVariant>,
) -> Element {
    let selected_tab: Signal<Option<uuid::Uuid>> = use_signal(|| tabs().get(0).map(|tab| tab.id));

    let orientation = orientation.unwrap_or_default();
    let tablist_class = format!("flex {}", orientation.classes());

    return rsx! {
        div { class,

            // list
            div { class: tablist_class,
                for tab in tabs() {
                    TabListItem { tab, tabs, selected_tab }
                }
            }
        }
    };
}

#[component]
pub fn TabListItem<T: PartialEq + Clone + Into<TabString> + 'static>(
    tab: TabItemData<T>,
    tabs: Signal<Vec<TabItemData<T>>>,
    selected_tab: Signal<Option<uuid::Uuid>>,
    child: Option<Element>,
) -> Element {
    use_context_provider::<TabState<T>>(|| TabState {
        tab,
        tabs,
        selected_tab,
    });

    return match child {
        Some(child) => rsx! {
            {child}
        },
        None => rsx! {
            DefaultTabPill::<T> {}
        },
    };
}

#[component]
fn DefaultTabPill<T: PartialEq + Clone + Into<TabString> + 'static>() -> Element {
    let tabstate = use_context::<TabState<T>>();

    let name: TabString = tabstate.clone().tab.item.into();
    let name: String = name.into();
    let mut selected = tabstate.selected_tab;

    return rsx! {
        Button {
            class: "gap-1 px-2 flex items-center",
            style: if selected() == Some(tabstate.tab.id) { ButtonStyleVariant::Default } else { ButtonStyleVariant::Transparent },
            onclick: move |_| {
                selected.set(Some(tabstate.tab.id.clone()));
            },
            Label { style: LabelStyleVariant::Mild, size: LabelSizeVariant::Small, "{name}" }
            div {
                Button {
                    class: "!p-0",
                    size: ButtonSizeVariant::Small,
                    style: ButtonStyleVariant::Ghost,
                    onclick: {
                        let tabstate = tabstate.clone();
                        move |e: Event<MouseData>| {
                            e.stop_propagation();
                            tabstate.remove();
                        }
                    },
                    Icon { icon: LdX, width: 12, height: 12 }
                }
            }
        }
    };
}
