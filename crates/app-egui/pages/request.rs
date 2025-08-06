use crate::{commands::Command, pages::Page};

pub struct RequestPage;

impl Page for RequestPage {
    fn show(
        &mut self,
        ctx: &egui::Context,
        state: &mut crate::state::ApplicationState,
        runtime: &mut crate::runtime::RuntimeData,
    ) -> Option<Command> {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World!");
        });
        return None;
    }
}
