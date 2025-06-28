use std::collections::HashSet;

use crate::button;
use dioxus::prelude::*;

#[derive(PartialEq, Clone)]
pub struct ButtonGroupInjection {
    pub multiselect: bool,
    pub selected_many: HashSet<uuid::Uuid>, // all selected button id
    pub selected_single: Option<uuid::Uuid>, // single selected button id
    pub active_style: button::ButtonStyleVariant,
    pub inactive_style: button::ButtonStyleVariant,
}

#[component]
pub fn ButtonGroup(
    multiselect: Option<bool>,
    class: Option<&'static str>,
    children: Element,
    active_style: Option<button::ButtonStyleVariant>,
    inactive_style: Option<button::ButtonStyleVariant>,
) -> Element {
    let multiselect = multiselect.unwrap_or(false);
    let active_style = active_style.unwrap_or_default();
    let inactive_style = inactive_style.unwrap_or_else(|| button::ButtonStyleVariant::Ghost);

    // set the group
    let button_group_injection = use_context_provider::<Signal<ButtonGroupInjection>>(|| {
        Signal::new(ButtonGroupInjection {
            multiselect,
            selected_many: HashSet::new(),
            selected_single: None,
            active_style,
            inactive_style,
        })
    });

    // set multiselect when it changes
    use_effect(move || {
        let mut signal = button_group_injection.clone();
        let mut data = signal.write();
        data.multiselect = multiselect.clone();
    });

    return rsx! {
        div { class, {children} }
    };
}

#[component]
pub fn GroupButton(
    children: Element,
    class: Option<&'static str>,
    size: Option<button::ButtonSizeVariant>,
    onclick: Option<EventHandler<Event<MouseData>>>,
) -> Element {
    let id = use_signal(|| uuid::Uuid::new_v4());
    let mut button_group_injection_signal = use_context::<Signal<ButtonGroupInjection>>();

    let is_selected = use_memo(move || {
        let id = id();
        let data = button_group_injection_signal();

        if data.multiselect {
            return data.selected_many.contains(&id);
        } else {
            return data.selected_single == Some(id);
        }
    });

    // on mounted add id to parent's data
    // TODO: remove on unmount
    use_hook(move || {
        let id = id();
        let mut group_injection = button_group_injection_signal.clone();
        let mut data = group_injection.write();

        if !data.multiselect {
            if data.selected_single.is_none() {
                data.selected_single = Some(id);
            }
        }
    });

    return rsx! {
        button::Button {
            class,
            style: if is_selected() { button_group_injection_signal().active_style } else { button_group_injection_signal().inactive_style },
            size,
            onclick: move |e| {
                let id = id();
                let mut data = button_group_injection_signal.write();
                if data.multiselect {
                    if data.selected_many.contains(&id) {
                        data.selected_many.remove(&id);
                    } else {
                        data.selected_many.insert(id.clone());
                    }
                } else {
                    data.selected_single = Some(id.clone());
                }
                if onclick.is_some() {
                    onclick.unwrap().call(e);
                }
            },

            {children}
        }
    };
}
