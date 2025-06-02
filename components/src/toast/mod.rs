use crate::toast::toast_item::ToastItemStruct;
use dioxus::prelude::*;

pub mod toast_item;

#[derive(Clone, PartialEq)]
pub struct ToastManager {
    pub items: Vec<ToastItemStruct>,
}

impl ToastManager {
    pub fn provide() {
        use_context_provider::<Signal<ToastManager>>(|| {
            Signal::new(ToastManager { items: vec![] })
        }); 
    }
}

#[component]
pub fn Toast() -> Element {
    use_context_provider::<Signal<Vec<ToastItemStruct>>>(move || Signal::new(vec![]));

    return rsx! {
        div {
            class: "bg-red-100",
            "Hello, world!"
        }
    };
}
