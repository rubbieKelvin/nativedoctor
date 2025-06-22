use dioxus::prelude::*;

use crate::states::ProjectState;

#[component]
pub fn RequestPanel() -> Element {
    let mut project_state = ProjectState::inject();
    let project = project_state.project.clone();

    let mut name = use_signal(|| String::new());
    let request_memo = {
        let project_state = project_state.clone();
        use_memo(move || project_state.get_selected_request())
    };

    use_effect(move || {
        match request_memo() {
            Some(request) => {
                let n = request.get_name().unwrap_or_else(|| String::new());
                name.with_mut(|name_mut| {
                    *name_mut = n;
                });
            },
            None => {}
        };
    });


    let request_name = {
        let request = request_memo();
        use_memo(move || match &request {
            Some(r) => r.get_name().unwrap_or_else(|| String::new()),
            None => String::new(),
        })
    };

    use_effect(move || {
        let name = name();
        match request_memo() {
            Some(mut request_copy) => {
                request_copy.set_name(&name, &project.unwrap().path);
                project_state.update_request(request_copy);
            }
            None => {}
        };
    });

    return match request_memo() {
        Some(r) => rsx! {
            div { class: "flex-grow",
                "{r.id}"
                br {}
                span { "Name" }
                input {
                    value: "{name}",
                    placeholder: "Request name",
                    oninput: move |e| {
                        let mut name = name.write();
                        *name = e.value().to_ascii_lowercase();
                    }
                    // {
                    //     let mut project_state = project_state.clone();

                    //     move |e: Event<FormData>| {
                    //         let value = e.value();
                    //         let value = value.to_lowercase();
                    //         let mut request = request_memo().unwrap();
                    //         request.set_name(&value, &project().unwrap().path);
                    //         project_state.update_request(request);
                    //     }
                    // },
                }
            }
        },
        None => rsx! {
            div { "Hello, click a request or create" }
        },
    };
}
