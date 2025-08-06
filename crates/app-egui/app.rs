use crate::{
    components::{Component, menubar::Menubar},
    pages::PageManager,
    runtime::RuntimeData,
    state::ApplicationState,
};

#[derive(Default)]
pub struct NativeDoctor {
    menubar: Menubar,
    state: ApplicationState,
    runtime: RuntimeData,
    pagemanager: PageManager,
}

impl NativeDoctor {
    pub fn new(_cx: &eframe::CreationContext<'_>) -> Self {
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
                self.menubar.show(ui, &mut self.state, &mut self.runtime);
            });

        self.pagemanager
            .show(ctx, &mut self.state, &mut self.runtime);
    }
}
