use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdX};
use uuid::Uuid;

use crate::{
    button::{Button, ButtonSizeVariant, ButtonStyleVariant},
    label::{Label, LabelStyleVariant},
    traits::Variant,
};

use crate::tabs::utils::TabGenerics;
pub use crate::tabs::utils::{TabItemData, TabState, TabPayload, TabSet};

mod utils;

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

// main shi
#[component]
pub fn TabsManager<T: TabGenerics + 'static>(
    tabs: Signal<TabSet<T>>,
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
    let tablist_class = format!(
        "flex {} {}",
        orientation.classes(),
        list_class.unwrap_or_default()
    );
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
fn TabListItem<T: TabGenerics + 'static>(
    tab: TabItemData<T>,
    tabs: Signal<TabSet<T>>,
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
fn DefaultTabPill<T: TabGenerics + 'static>() -> Element {
    let tabstate = use_context::<TabState<T>>();

    let name = tabstate.tab.payload.get_title();
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
fn TabContent<T: TabGenerics + 'static>(
    tab: TabItemData<T>,
    tabs: Signal<TabSet<T>>,
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
