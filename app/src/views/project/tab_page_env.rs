use components_lib::prelude::{TextField, TextFieldSizeVariant, TextFieldStyleVariant};
use dioxus::prelude::*;

#[component]
pub fn EnvPage() -> Element {
    let mut env_name = use_signal(|| String::new());

    return rsx!{
        div {
            class: "h-full flex pt-2 flex-col",
            TextField {
                    placeholder: "Name",
                    value: "{env_name}",
                    style: TextFieldStyleVariant::Ghost,
                    size: TextFieldSizeVariant::Large,
                    oninput: move |e: Event<FormData>| {
                        let value = e.value();
                        env_name.set(value);
                    }
                }
        }
    };
}