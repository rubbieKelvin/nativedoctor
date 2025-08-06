use crate::{
    commands::Command,
    components::{self, Component},
    pages::Page,
};

pub struct RequestPage {
    panel: components::requestpanel::RequestPanel,
}

impl RequestPage {
    pub fn new() -> Self {
        return RequestPage {
            panel: components::requestpanel::RequestPanel::new(),
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
        egui::CentralPanel::default().show(ctx, |ui| {
            self.panel.show(ui, state, runtime);
        });
        return None;
    }
}
