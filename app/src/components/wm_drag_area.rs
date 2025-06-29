// ui/draggable_header.rs
use dioxus::{html::input_data::MouseButton, prelude::*};

#[cfg(feature = "desktop")]
use dioxus_desktop::use_window;

#[component]
pub fn WmDragArea(class: Option<&'static str>, children: Element) -> Element {
    #[cfg(feature = "desktop")]
    {
        let window = use_window();
        let mdwindow_handle = window.clone();
        let dbc_window_handle = window.clone();

        return rsx! {
            div {
                class,
                onmousedown: move |event| {
                    if event.held_buttons() == MouseButton::Primary {
                        mdwindow_handle.drag_window()?;
                    }
                    Ok(())
                },
                ondoubleclick: move |event| {
                    if event.trigger_button() == Some(MouseButton::Primary) {
                        let maximized = dbc_window_handle.is_maximized();
                        dbc_window_handle.set_maximized(!maximized);
                    }
                    Ok(())
                },
                {children}
            }
        };
    }

    #[cfg(not(feature = "desktop"))]
    return rsx! {
        div { class, {children}}
    };
}
