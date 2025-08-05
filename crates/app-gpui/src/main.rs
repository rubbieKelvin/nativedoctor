// // Disable command line from opening on release mode
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use gpui::{
//     App, Application, Bounds, Context, TitlebarOptions, Window, WindowBounds, WindowOptions, div,
//     prelude::*, px, size,
// };

// mod constants;

// struct HelloWorld;

// impl Render for HelloWorld {
//     fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
//         return div();
//     }
// }

// fn main() {
//     let app = Application::new();

//     app.run(|cx: &mut App| {
//         // Close the application once all windows are closed
//         cx.on_window_closed(|cx| {
//             if cx.windows().is_empty() {
//                 cx.quit();
//             }
//         })
//         .detach();

//         // Open window
//         cx.open_window(
//             WindowOptions {
//                 app_id: Some(constants::APP_ID.to_string()),
//                 titlebar: Some(TitlebarOptions {
//                     title: Some(constants::APP_NAME.into()),
//                     appears_transparent: true,
//                     traffic_light_position: Default::default(),
//                 }),
//                 window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
//                     None,
//                     size(px(500.), px(500.0)),
//                     cx,
//                 ))),
//                 ..Default::default()
//             },
//             |_, cx| cx.new(|_| HelloWorld {}),
//         )
//         .unwrap();
//     });
// }

use gpui::{
    App, Application, Context, Menu, MenuItem, Window, WindowOptions, actions, div, prelude::*, rgb,
};

use crate::states::{AppState, GlobalAppState};

mod constants;
mod menu;
mod states;

struct NativeDoctor;

impl Render for NativeDoctor {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .bg(rgb(0x1e1e1e))
            .size_full()
            .justify_center()
            .items_center()
            .text_color(rgb(0xffffff))
            .child(div().text_2xl().child("My GPUI App"))
            .child(
                div()
                    .mt_4()
                    .text_lg()
                    .child(format!("Counter: {}", self.counter)),
            )
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        cx.activate(true);

        // set global app state
        // cx.set_global(GlobalAppState(cx.new(|_| AppState::default())));
        AppState::init(cx);

        // Register all action handlers
        // cx.on_action(quit);
        // cx.on_action(about);
        // cx.on_action(new_document);
        // cx.on_action(open_document);
        // cx.on_action(save_document);
        // cx.on_action(increment_counter);
        // cx.on_action(decrement_counter);
        // cx.on_action(reset_counter);

        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| NativeDoctor {})
        })
        .unwrap();
    });
}
