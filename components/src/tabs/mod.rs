use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdX};
use uuid::Uuid;

use crate::{
    button::{Button, ButtonSizeVariant, ButtonStyleVariant},
    label::{Label, LabelSizeVariant, LabelStyleVariant},
    traits::Variant,
};

#[derive(Clone, Default, PartialEq)]
pub struct TabString(pub String);

impl From<String> for TabString {
    fn from(s: String) -> Self {
        TabString(s)
    }
}

impl Into<String> for TabString {
    fn into(self) -> String {
        self.0
    }
}

#[derive(Clone, PartialEq)]
pub struct TabItemData<T: PartialEq + Clone + Into<TabString>> {
    pub id: Uuid,
    pub item: T,
    pub closable: bool,
}

impl<T: PartialEq + Clone + Into<TabString>> TabItemData<T> {
    pub fn new(item: T) -> Self {
        TabItemData {
            id: Uuid::new_v4(),
            item,
            closable: true,
        }
    }
}

#[allow(unused)]
#[derive(Clone, PartialEq, Default, strum::EnumIter)]
pub enum TabOrientationVariant {
    #[default]
    Horizontal,
    Vertical,
}

impl Variant for TabOrientationVariant {
    fn classes(&self) -> &'static str {
        match self {
            TabOrientationVariant::Horizontal => "flex-row",
            TabOrientationVariant::Vertical => "flex-col",
        }
    }
}

/// Context passed to each tab pill and its children.
#[derive(Clone, PartialEq)]
pub struct TabState<T: PartialEq + Clone + Into<TabString> + 'static> {
    pub tab: TabItemData<T>,
    pub tabs: Signal<Vec<TabItemData<T>>>,
    pub selected_tab: Signal<Option<Uuid>>,
}

impl<T: PartialEq + Clone + Into<TabString> + 'static> TabState<T> {
    /// Removes the current tab from the list and handles re-selection.
    fn remove(&self) {
        let tab_id_to_remove = self.tab.id;
        let mut tabs = self.tabs;
        let mut selected_tab = self.selected_tab;

        let currently_selected_id = selected_tab.cloned();

        tabs.with_mut(|tabs_vec| {
            let Some(index_to_remove) = tabs_vec.iter().position(|t| t.id == tab_id_to_remove)
            else {
                tracing::warn!("Tried to remove a tab that does not exist.");
                return;
            };

            // Remove the tab.
            tabs_vec.remove(index_to_remove);
            tracing::info!(
                "Removed tab at index {}. New count: {}",
                index_to_remove,
                tabs_vec.len()
            );

            // If the removed tab was the selected one, we need to select a new one.
            if currently_selected_id == Some(tab_id_to_remove) {
                if tabs_vec.is_empty() {
                    // No tabs left, so clear selection.
                    selected_tab.set(None);
                } else {
                    // Select the item at the same index, or the new last item
                    // if the original last item was the one removed.
                    let new_index = index_to_remove.min(tabs_vec.len() - 1);
                    let new_selected_id = tabs_vec.get(new_index).map(|t| t.id);
                    selected_tab.set(new_selected_id);
                }
            }
        });
    }
}

/// The main Tabs component.
///
/// It manages the state for a list of tabs and renders the tab pills
/// and the content for the currently active tab.
#[component]
pub fn TabsManager<T: PartialEq + Clone + Into<TabString> + 'static>(
    tabs: Signal<Vec<TabItemData<T>>>,
    class: Option<String>,
    list_class: Option<String>,
    content_class: Option<String>,
    pill: Option<Element>,
    orientation: Option<TabOrientationVariant>,
    emptystate: Option<Element>,
    children: Element,
) -> Element {
    let mut selected_tab: Signal<Option<Uuid>> = use_signal(|| tabs().first().map(|tab| tab.id));

    // when the tabs list changes, ensure a tab is selected if possible.
    use_effect(move || {
        if selected_tab.with(|id| id.is_none()) && !tabs.with(|t| t.is_empty()) {
            selected_tab.set(tabs.with(|t| t.first().map(|tab| tab.id)));
        }
    });

    let orientation = orientation.unwrap_or_default();
    let content_class = content_class.unwrap_or_default();
    let tablist_class = format!("flex {} {}", orientation.classes(), list_class.unwrap_or_default());
    let main_class = class.unwrap_or_default();
    let main_class = match orientation {
        TabOrientationVariant::Horizontal => format!("{main_class} flex flex-col"),
        TabOrientationVariant::Vertical => format!("{main_class} flex"),
    };

    // full data for the selected tab.
    let selected_tab_data = use_memo(move || {
        if let Some(selected_id) = selected_tab() {
            return tabs().iter().find(|t| t.id == selected_id).cloned();
        }
        None
    });

    rsx! {
        div { class: main_class,
            // list
            div {
                class: tablist_class,
                role: "tablist",
                for tab in tabs() {
                    TabListItem {
                        key: "{tab.id}",
                        tab,
                        tabs,
                        selected_tab,
                        child: pill.clone()
                    }
                }
            }

            // content
            div { class: "{content_class} flex-grow", role: "tabpanel",
                if let Some(tab) = &selected_tab_data() {
                    TabContent {
                        key: "{tab.id}",
                        tab: tab.clone(),
                        tabs,
                        selected_tab,
                        {children}
                    }
                } else if let Some(es) = emptystate {
                    {es}
                }
            }
        }
    }
}

/// individual item in the tab list to provides the `TabState` context for children pills.
#[component]
fn TabListItem<T: PartialEq + Clone + Into<TabString> + 'static>(
    tab: TabItemData<T>,
    tabs: Signal<Vec<TabItemData<T>>>,
    selected_tab: Signal<Option<Uuid>>,
    child: Option<Element>,
) -> Element {
    use_context_provider(|| TabState {
        tab,
        tabs,
        selected_tab,
    });

    return match child {
        Some(child) => rsx! {{child}},
        None => rsx! {DefaultTabPill::<T> {}},
    };
}

/// default visual representation of a single tab pill.
#[component]
fn DefaultTabPill<T: PartialEq + Clone + Into<TabString> + 'static>() -> Element {
    let tabstate = use_context::<TabState<T>>();

    let name: TabString = tabstate.tab.item.clone().into();
    let name: String = name.into();
    let is_selected = tabstate
        .selected_tab
        .with(|id| *id == Some(tabstate.tab.id));

    let button_style = if is_selected {
        ButtonStyleVariant::Default
    } else {
        ButtonStyleVariant::Transparent
    };

    rsx! {
        div { role: "tab", "aria-selected": "{is_selected}",
            Button {
                class: "gap-1 px-2 flex items-center w-full",
                style: button_style,
                onclick: {
                    let mut selected_tab = tabstate.selected_tab;
                    move |_| {
                        selected_tab.set(Some(tabstate.tab.id.clone()));
                    }
                },

                Label {
                    class: "flex-grow text-start",
                    style: LabelStyleVariant::Mild,
                    size: LabelSizeVariant::Small,
                    "{name}"
                }

                if tabstate.tab.closable {
                    Button {
                        class: "!p-0 flex items-center justify-center",
                        size: ButtonSizeVariant::Small,
                        style: ButtonStyleVariant::Ghost,
                        onclick: move |e: MouseEvent| {
                            e.stop_propagation();
                            tabstate.remove();
                        },
                        Icon { icon: LdX, width: 12, height: 12 }
                    }
                }
            }
        }
    }
}

#[component]
fn TabContent<T: PartialEq + Clone + Into<TabString> + 'static>(
    tab: TabItemData<T>,
    tabs: Signal<Vec<TabItemData<T>>>,
    selected_tab: Signal<Option<Uuid>>,
    children: Element,
) -> Element {
    use_context_provider(|| TabState {
        tab,
        tabs,
        selected_tab,
    });

    return rsx! {{children}};
}
