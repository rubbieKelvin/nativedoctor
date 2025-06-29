use dioxus::prelude::*;

#[component]
pub fn Dialog(
    show: Signal<bool>,
    children: Element,
    title: Option<String>,
    close_on_click_outside: Option<bool>,
) -> Element {
    // If the 'show' signal is false, don't render anything.
    if !show() {
        return rsx! {};
    }

    let title = title.unwrap_or("Dialog".to_string());
    let close_on_click_outside = close_on_click_outside.unwrap_or(true);

    rsx! {
        // Backdrop
        div {
            class: "fixed inset-0 bg-black/40 flex items-center justify-center z-50 p-4",
            onclick: move |_| {
                if close_on_click_outside {
                show.set(false)
                }
            },

            // Dialog
            div {
                class: "w-max mx-auto relative",
                onclick: move |event| event.stop_propagation(),
                role: "dialog",
                aria_modal: "true",
                aria_labelledby: "dialog-title",

                span {
                    id: "dialog-title",
                    class: "text-2xl font-semibold text-gray-900 hidden",
                    "{title}"
                }


                {children}

            }
        }
    }
}
