use crate::{runtime::RuntimeData, state::ApplicationState};

pub mod menubar;

pub trait Component {
    type Response;

    fn show(
        &mut self,
        ctx: &egui::Context,
        state: &mut ApplicationState,
        runtime: &mut RuntimeData,
    ) -> Self::Response;
}
