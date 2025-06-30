use dioxus::prelude::*;
use dioxus_free_icons::{Icon, icons::ld_icons::LdX};

use crate::{
    button::{Button, ButtonSizeVariant, ButtonStyleVariant},
    label::{Label, LabelStyleVariant},
    traits::Variant,
};

use crate::tabs::utils::TabGenerics;
pub use crate::tabs::utils::{TabItemData, TabPayload, TabSet, TabState};

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
    // when the tabs list changes, ensure a tab is selected if possible.
    use_effect(move || {
        let mut tabset = tabs.write();
        if tabset.is_empty() {
            let first_id = tabset.first().map(|t| t.id);
            tabset.select(first_id);
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
    let selected_tab_data = use_memo(move || tabs().get_selected().cloned());

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
    child: Option<Element>,
) -> Element {
    use_context_provider(|| TabState { tab, tabs });

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
        .tabs
        .with(|tabset| tabset.get_selected().map(|t| t.id) == Some(tabstate.tab.id));

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
                    let mut tabs = tabstate.tabs;
                    move |_| {
                        tabs.with_mut(|tabset| {
                            tabset.select(Some(tabstate.tab.id.clone()));
                        })
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
    children: Element,
) -> Element {
    use_context_provider(|| TabState {
        tab,
        tabs,
    });

    return rsx! {{children}};
}
