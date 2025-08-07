use gpui::{App, Menu, MenuItem, actions};
use nd_core::constants;

// Define all actions
actions!(
    my_app,
    [
        Quit,
        About,
        NewDocument,
        OpenDocument,
        SaveDocument,
        IncrementCounter,
        DecrementCounter,
        ResetCounter
    ]
);

pub fn init(cx: &mut App) {
    cx.set_menus(vec![
        // App menu
        Menu {
            name: constants::APPLICATION_NAME.into(),
            items: vec![
                MenuItem::action("About My App", About),
                MenuItem::separator(),
                MenuItem::submenu(Menu {
                    name: "Preferences".into(),
                    items: vec![
                        MenuItem::action("Settings...", About), // Placeholder
                        MenuItem::action("Keyboard Shortcuts...", About), // Placeholder
                    ],
                }),
                MenuItem::separator(),
                MenuItem::action("Quit My App", Quit),
            ],
        },
        // File menu
        Menu {
            name: "File".into(),
            items: vec![
                MenuItem::action("New", NewDocument),
                MenuItem::action("Open...", OpenDocument),
                MenuItem::separator(),
                MenuItem::action("Save", SaveDocument),
                MenuItem::action("Save As...", SaveDocument), // Placeholder
            ],
        },
        // Edit menu
        Menu {
            name: "Edit".into(),
            items: vec![
                MenuItem::action("Undo", About), // Placeholder
                MenuItem::action("Redo", About), // Placeholder
                MenuItem::separator(),
                MenuItem::action("Cut", About),   // Placeholder
                MenuItem::action("Copy", About),  // Placeholder
                MenuItem::action("Paste", About), // Placeholder
            ],
        },
        // View menu
        Menu {
            name: "View".into(),
            items: vec![MenuItem::submenu(Menu {
                name: "Counter".into(),
                items: vec![
                    MenuItem::action("Increment", IncrementCounter),
                    MenuItem::action("Decrement", DecrementCounter),
                    MenuItem::separator(),
                    MenuItem::action("Reset", ResetCounter),
                ],
            })],
        },
        // Help menu
        Menu {
            name: "Help".into(),
            items: vec![
                MenuItem::action("Documentation", About), // Placeholder
                MenuItem::separator(),
                MenuItem::action("About", About),
            ],
        },
    ]);
}

// Action handlers
fn quit(_: &Quit, cx: &mut App) {
    cx.quit();
}

fn about(_: &About, cx: &mut App) {
    println!("About My App - A GPUI application with menus!");
}

fn new_document(_: &NewDocument, cx: &mut App) {
    println!("Creating new document...");
}

fn open_document(_: &OpenDocument, cx: &mut App) {
    println!("Opening document...");
}

fn save_document(_: &SaveDocument, cx: &mut App) {
    println!("Saving document...");
}

fn increment_counter(_: &IncrementCounter, cx: &mut App) {
    println!("Stuff")
}

fn decrement_counter(_: &DecrementCounter, cx: &mut App) {
    println!("Decrementing counter...");
}

fn reset_counter(_: &ResetCounter, cx: &mut App) {
    println!("Resetting counter...");
}
