use dioxus::prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TabKey {
    pub key: String,
    pub title: String,
}

#[component]
pub fn Tab(tabs: Vec<TabKey>) -> Element {
    return rsx! {};
}
