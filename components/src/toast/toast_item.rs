use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ToastItemStruct {
    pub id: String,
    pub message: String,
    pub duration: Option<u64>,
}

#[component]
fn ToastItem(item: ToastItemStruct) -> Element {
    return rsx! {
        div {
            class: "bg-red-100",
            { item.message }
        }
    };
}
