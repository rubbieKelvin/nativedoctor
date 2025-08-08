use gpui::Context;

use crate::{states::theme::Theme, views::ViewManager};

pub mod theme;
pub struct ApplicationState {
    pub theme: Theme,
    pub view: ViewManager,
}

impl ApplicationState {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        return Self {
            theme: Theme::default(),
            view: ViewManager::new(),
        };
    }
}
