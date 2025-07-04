use crate::button::{self, Button};
use dioxus::prelude::*;

pub trait ButtonGroupInner {
    fn render(&self) -> Element;
}

impl ButtonGroupInner for String {
    fn render(&self) -> Element {
        return rsx! {
            span {
                "{self}"
            }
        };
    }
}

#[component]
pub fn ButtonGroup<T: Clone + PartialEq + ButtonGroupInner + 'static>(
    value: T,
    buttons: Vec<T>,
    class: Option<String>,
    child_class: Option<String>,
    size: Option<button::ButtonSizeVariant>,
    active_style: Option<button::ButtonStyleVariant>,
    inactive_style: Option<button::ButtonStyleVariant>,
    onselect: Option<EventHandler<T>>,
) -> Element {
    let size = size.unwrap_or_default();
    let active_style = active_style.unwrap_or_default();
    let inactive_style = inactive_style.unwrap_or_else(|| button::ButtonStyleVariant::Ghost);
    let child_class = child_class.unwrap_or_default();
    
    return rsx! {
        div { class,
            for button in buttons {
                Button {
                    size: size.clone(),
                    class: child_class.clone(),
                    style: if value == button.clone() { active_style.clone() } else { inactive_style.clone() },
                    onclick: {
                        let button = button.clone();
                        move |_| {
                            if let Some(onselect) = onselect {
                                onselect.call(button.clone());
                            }
                        }
                    },
                    {button.render()}
                }
            }
        }
    };
}
