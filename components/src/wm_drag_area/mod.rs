// ui/draggable_header.rs
use dioxus::{html::input_data::MouseButton, prelude::*};
use dioxus_desktop::use_window;

#[component]
pub fn WmDragArea(class: Option<&'static str>, children: Element) -> Element {
    let window = use_window();

    rsx! {
        div {
            class,
            onmousedown: move |event| {
                if event.held_buttons() == MouseButton::Primary {
                    window.drag_window()?;
                }
                Ok(())
            },
            {children}
        }
    }
}
