use dioxus::prelude::*;

#[component]
pub fn KeypressListener() -> Element {
    use_effect(move || {
        let mut eval = document::eval(
            "document.addEventListener(
                'keydown',
                (event) => {
                    const keys = [];

                    if (event.ctrlKey) keys.push('Ctrl');
                    if (event.altkey) keys.push('Alt');
                    if (event.shiftKey) keys.push('Shift');
                    if (event.metaKey) keys.push('Meta');

                    const key = event.key;

                    if (!keys.includes(key)) {
                        keys.push(key);
                    }

                    const combo = keys.join('+');
                    dioxus.send(combo);
                }
            );",
        );

        spawn(async move {
            while let Ok(data) = eval.recv::<String>().await {
                tracing::info!("Recived keypress from javascript: {}", data);
            }
        });
    });

    // return empty fragment
    return rsx!{};
}