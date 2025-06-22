use dioxus::desktop::use_window;
use dioxus::prelude::*;
use dioxus_desktop::{
    tao::event::{ElementState, Event, KeyEvent, WindowEvent},
    use_global_shortcut,
};
use nativedoctor_core::{
    fs::FileObject,
    schema::roots::{ProjectRootSchema, RequestRootSchema},
};

use crate::{
    components::WmDragArea,
    states::ProjectState,
    views::project::{panel, side},
};

#[component]
pub fn ProjectView(
    schema: FileObject<ProjectRootSchema>,
    requests: Vec<FileObject<RequestRootSchema>>,
) -> Element {
    let project_state = use_context_provider(|| ProjectState::new(schema, requests));
    let mut saving = use_signal(|| false);
    let save_shortcut = if cfg!(target_os = "macos") {
        "cmd+s"
    } else {
        "ctrl+s"
    };

    _ = use_global_shortcut(save_shortcut, move || {
        if saving() {
            return;
        }

        tracing::info!("Saving");
        *saving.write() = true;
        let project_state = project_state.clone();

        spawn(async move {
            match project_state.save().await {
                Ok(_) => tracing::info!("Saved"),
                Err(e) => tracing::error!("Error saving: {}", e),
            }
            *saving.write() = false;
        });
    });

    // #[cfg(feature = "desktop")]
    // use_wry_event_handler(move |event   , _window| {
    //     if let Event::WindowEvent { event, .. } = event {
    //         if let WindowEvent::KeyboardInput { event, .. } = event {
    //             tracing::info!("Keyboard input: {:?}", event,);
    //         }
    //     }
    // });

    // #[cfg(feature = "desktop")]
    // use_wry_event_handler(move |event, _| {
    //     // Log the top-level event first to see everything
    //     tracing::info!("Raw Wry Event: {:?}", event);

    //     // Step 1: Unwrap the main Event enum
    //     if let Event::WindowEvent {
    //         event: window_event,
    //         ..
    //     } = event
    //     {
    //         // Log the WindowEvent to see what kind it is (KeyboardInput, CloseRequested, etc.)
    //         tracing::info!("Received Window Event: {:?}", window_event);

    //         // Step 2: Unwrap the WindowEvent enum for KeyboardInput
    //         if let WindowEvent::KeyboardInput {
    //             event: key_event, ..
    //         } = window_event
    //         {
    //             // Now you have the KeyEvent
    //             let KeyEvent {
    //                 state,
    //                 logical_key,
    //                 location,
    //                 physical_key,
    //                 ..
    //             } = key_event;

    //             tracing::info!(
    //                 "Keyboard Input - State: {:?}, Logical Key: {:?}, Location: {:?}, Physical Key: {:?}",
    //                 state, logical_key, location, physical_key
    //             );

    //             // Your Ctrl+S / Cmd+S logic
    //             // let is_ctrl_or_cmd = modifiers.control_key() || modifiers.super_key();
    //             // if state == ElementState::Pressed
    //             //     && is_ctrl_or_cmd
    //             //     && logical_key.to_text().map_or(false, |s| s == "s")
    //             // {
    //             //     tracing::info!("Ctrl+S or Cmd+S pressed (Wry event)! Saving...");
    //             //     // Implement your save logic here
    //             // }
    //         }
    //     }
    // });

    // load project in scope
    // {
    //     // clone upper scope
    //     let path = path.clone();
    //     let mut project = project_state.project.clone();

    //     use_effect(move || {
    //         tracing::info!("Loading project, {:?}", &path);

    //         let path = path.clone();

    //         // Asynchronously load up the project from file
    //         spawn(async move {
    //             let p = ProjectRootSchema::load(&path).await;
    //             match p {
    //                 Ok(p) => {
    //                     let mut project = project.write();
    //                     *project = Some(p);
    //                 }
    //                 // TODO: we need to let the user know an error showed up somehow.
    //                 Err(e) => tracing::error!("{e}"),
    //             };
    //         });
    //     })
    // };

    return rsx! {
        div { class: "flex flex-col h-full",
            WmDragArea { class: "h-10 flex items-center" }

            div { class: "flex-grow flex", side::SideBar {} }
        }
    };
}
