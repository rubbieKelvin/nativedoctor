use dioxus::prelude::*;
use crate::appdata::requests::HttpMethod;

#[component]
pub fn HttpMethodBadge(method: HttpMethod) -> Element {
    let color = match method {
        HttpMethod::GET => "text-blue-400",
        HttpMethod::POST => "text-green-400",
        HttpMethod::PUT => "text-yellow-400",
        HttpMethod::DELETE => "text-red-400",
        HttpMethod::PATCH => "text-purple-400",
        HttpMethod::OPTIONS => "text-gray-400",
        HttpMethod::HEAD => "text-gray-400",
        HttpMethod::CONNECT => "text-gray-400",
        HttpMethod::TRACE => "text-gray-400",
    };

    return rsx! {
        div {
            class: "rounded-sm p-0.5 text-xs max-w-[36px] w-[36px] min-w-[36px] text-start {color}",
            "{method.to_string()}"
        }
    };
}