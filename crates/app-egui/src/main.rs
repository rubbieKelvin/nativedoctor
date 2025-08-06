use eframe::egui;
use egui::ViewportBuilder;
use nd_core::constants::{APPLICATION_ID, APPLICATION_NAME};

mod shortcuts;

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions {
        viewport: ViewportBuilder::default()
            .with_title(APPLICATION_NAME.to_string())
            .with_app_id(APPLICATION_ID.to_string())
            .with_min_inner_size((800.0, 600.0))
            .with_fullsize_content_view(true)
            .with_titlebar_shown(false)
            .with_title_shown(false)
            .with_active(true),
        persist_window: true,
        ..Default::default()
    };

    return eframe::run_native(
        APPLICATION_NAME,
        native_options,
        Box::new(|cc| Ok(Box::new(NativeDoctor::new(cc)))),
    );
}

#[derive(Default)]
struct NativeDoctor {}

impl NativeDoctor {
    fn new(_cx: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for NativeDoctor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Menu bar
        egui::TopBottomPanel::top("menu_bar")
            .frame(
                egui::Frame::default()
                    .inner_margin(egui::Margin {
                        left: 80,
                        top: 8,
                        bottom: 8,
                        ..Default::default()
                    })
                    .fill(ctx.style().visuals.panel_fill),
            )
            .show(ctx, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    // handle shortcuts
                    if ui.input_mut(|i| i.consume_shortcut(&shortcuts::FILE_NEW)) {
                        println!("Clicked button from shorcut");
                    }

                    ui.menu_button("File", |ui| {
                        if ui.button("New").clicked() {
                            println!("Clicked button");
                        }
                    });
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
    }
}
