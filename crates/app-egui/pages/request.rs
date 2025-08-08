use crate::{
    commands::Command,
    components::{self, Component},
    pages::Page,
};

pub struct RequestPage {
    panel: components::requestpanel::RequestDashboard,
}

impl RequestPage {
    pub fn new() -> Self {
        return RequestPage {
            panel: components::requestpanel::RequestDashboard::new(),
        };
    }
}

impl Page for RequestPage {
    fn show(
        &mut self,
        ctx: &egui::Context,
        state: &mut crate::state::ApplicationState,
        runtime: &mut crate::runtime::RuntimeData,
    ) -> Option<Command> {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::default()
                    .inner_margin(egui::Margin::symmetric(16, 12))
                    .fill(ctx.style().visuals.panel_fill)
            )
            .show(ctx, |ui| {
                self.panel.show(ui, state, runtime);
            });
        return None;
    }
}
