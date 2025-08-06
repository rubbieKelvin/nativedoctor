use std::collections::HashMap;

use crate::{
    commands::Command,
    runtime::RuntimeData,
    state::{ApplicationState, PageType},
};

pub mod request;

pub trait Page {
    fn show(
        &mut self,
        ctx: &egui::Context,
        state: &mut ApplicationState,
        runtime: &mut RuntimeData,
    ) -> Option<Command>;

    fn on_activate(&mut self, _state: &mut ApplicationState) {}
    fn on_deactivate(&mut self, _state: &mut ApplicationState) {}
}

pub struct PageManager {
    pages: HashMap<PageType, Box<dyn Page>>,
    current: PageType,
}

impl PageManager {
    pub fn new() -> Self {
        let mut pages: HashMap<PageType, Box<dyn Page>> = HashMap::new();
        pages.insert(PageType::RequestPage, Box::new(request::RequestPage {}));

        return PageManager {
            current: PageType::default(),
            pages,
        };
    }

    pub fn show(
        &mut self,
        ctx: &egui::Context,
        state: &mut ApplicationState,
        runtime: &mut RuntimeData,
    ) -> Option<Command> {
        // check for view changes in the app state
        if state.page != self.current {
            self.change(state.page, state);
        }

        // Show current view
        if let Some(view) = self.pages.get_mut(&self.current) {
            return view.show(ctx, state, runtime);
        }

        return None;
    }

    pub fn change(&mut self, new: PageType, state: &mut ApplicationState) {
        if let Some(view) = self.pages.get_mut(&self.current) {
            view.on_deactivate(state);
        }

        if let Some(view) = self.pages.get_mut(&new) {
            view.on_activate(state);
        }

        self.current = new;
    }
}

impl Default for PageManager {
    fn default() -> Self {
        return PageManager::new();
    }
}
