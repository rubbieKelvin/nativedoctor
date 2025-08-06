use crate::{runtime::RuntimeData, state::ApplicationState};

pub mod menubar;
pub mod requestpanel;

pub trait Component {
    type Response;

    fn show(
        &mut self,
        ui: &mut egui::Ui,
        state: &mut ApplicationState,
        runtime: &mut RuntimeData,
    ) -> Self::Response;
}
