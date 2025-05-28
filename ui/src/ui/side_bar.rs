use crate::ui::enviroment_selector;
use dioxus::prelude::*;

#[component]
pub fn SideBar() -> Element {
    rsx! {
        div {
            class: "h-full flex flex-col border-r-gray-200 border-r w-[26%]",

            // Environment selector
            div {
                class: "p-2 border-b border-gray-200",
                enviroment_selector::EnviromentSelector{}
            }

            // Request list section
            div {
                class: "flex-1 overflow-y-auto p-4",
                h3 {
                    class: "text-sm font-semibold mb-2",
                    "Requests"
                }
                div {
                    class: "space-y-2",
                    // Example request items
                    div {
                        class: "p-2 hover:bg-gray-100 rounded cursor-pointer",
                        "GET /api/users"
                    }
                    div {
                        class: "p-2 hover:bg-gray-100 rounded cursor-pointer",
                        "POST /api/auth"
                    }
                    div {
                        class: "p-2 hover:bg-gray-100 rounded cursor-pointer",
                        "PUT /api/settings"
                    }
                }
            }

            // Call sequence section
            div {
                class: "p-4 border-t border-gray-200",
                h3 {
                    class: "text-sm font-semibold mb-2",
                    "Call Sequence"
                }
                div {
                    class: "space-y-2",
                    // Example sequence items
                    div {
                        class: "p-2 bg-gray-50 rounded",
                        "1. Auth Request"
                    }
                    div {
                        class: "p-2 bg-gray-50 rounded",
                        "2. User Data"
                    }
                    div {
                        class: "p-2 bg-gray-50 rounded",
                        "3. Settings Update"
                    }
                }
            }
        }
    }
}
