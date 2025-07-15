use dioxus::prelude::*;

#[component]
pub fn Popup(
    class: Option<String>,
    item: Element,
    content: Element,
    is_open: Signal<bool>,
) -> Element {
    // let mut position = use_signal(|| (0, 0));
    let class = class.unwrap_or_default();

    rsx! {
        div {
            class: format!("relative {}", class),
            oncontextmenu: move |e| {
                e.prevent_default();

                if is_open() {
                    is_open.set(false);
                    return;
                }

                // let rect = e.client_coordinates();
                // position.set((rect.x as i32, rect.y as i32));
                is_open.set(true);
            },

            // Render children
            {item}

            // Context menu
            if is_open() {
                // Backdrop to close menu when clicking outside
                div {
                    class: "fixed inset-0 z-10",
                    onclick: move |_| is_open.set(false),
                }

                // Menu container
                div {
                    class: "absolute z-20 bg-bg-secondary border rounded-md shadow-lg ring-1 ring-black ring-opacity-5 min-w-[160px]",
                    // style: format!("left: {}px; top: {}px;", position().0, position().1),
                    onclick: move |e| e.stop_propagation(),

                    {content}
                }
            }
        }
    }
}
