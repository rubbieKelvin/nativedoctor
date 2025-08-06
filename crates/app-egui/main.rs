#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;
use egui::ViewportBuilder;
use nd_core::constants::{APPLICATION_ID, APPLICATION_NAME};

mod app;
mod commands;
mod components;
mod pages;
mod runtime;
mod shortcuts;
mod state;

fn main() -> Result<(), eframe::Error> {
    // Enable logger
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title(APPLICATION_NAME.to_string())
            .with_app_id(APPLICATION_ID.to_string())
            .with_min_inner_size((800.0, 600.0))
            .with_fullsize_content_view(true)
            .with_titlebar_shown(false)
            .with_title_shown(false)
            .with_icon(load_icon())
            .with_active(true),
        persist_window: true,
        ..Default::default()
    };

    return eframe::run_native(
        APPLICATION_NAME,
        native_options,
        Box::new(|cc| Ok(Box::new(app::NativeDoctor::new(cc)))),
    );
}

fn load_icon() -> egui::IconData {
    return eframe::icon_data::from_png_bytes(include_bytes!("resources/app-icon.png"))
        .unwrap_or_default();
}
