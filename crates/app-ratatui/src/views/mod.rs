use std::{collections::HashMap, io};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, KeyEvent, KeyEventKind},
    layout::Rect,
};

use crate::{
    commands::{self, Command},
    state::{self, ApplicationState, ViewType},
};

pub mod request;

pub trait View {
    fn render(&self, area: Rect, buf: &mut Buffer, state: &ApplicationState);

    fn handle_key_event(
        &mut self,
        _key: KeyEvent,
        _state: &mut ApplicationState,
    ) -> io::Result<Option<Command>> {
        return Ok(None);
    }

    fn handle_event(
        &mut self,
        state: &mut state::ApplicationState,
    ) -> io::Result<Option<commands::Command>> {
        match event::read()? {
            event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event, state)
            }
            _ => Ok(None),
        }
    }

    #[allow(unused)]
    fn on_activate(&self, _state: &ApplicationState) {}
    #[allow(unused)]
    fn on_deactivate(&self, _state: &ApplicationState) {}
}

pub struct ViewManager {
    views: HashMap<ViewType, Box<dyn View>>,
    current: ViewType,
}

impl ViewManager {
    pub fn new() -> Self {
        let mut views: HashMap<ViewType, Box<dyn View>> = HashMap::new();
        views.insert(ViewType::RequestPage, Box::new(request::RequestPage::new()));

        return ViewManager {
            current: ViewType::default(),
            views,
        };
    }

    pub fn render(&self, area: Rect, buf: &mut Buffer, state: &ApplicationState) {
        // Show current view
        if let Some(view) = self.views.get(&self.current) {
            return view.render(area, buf, state);
        }
    }

    #[allow(unused)]
    pub fn change(&mut self, new: ViewType, state: &ApplicationState) {
        if let Some(view) = self.views.get(&self.current) {
            view.on_deactivate(state);
        }

        if let Some(view) = self.views.get(&new) {
            view.on_activate(state);
        }

        self.current = new;
    }

    pub fn handle_event(&mut self, state: &mut ApplicationState) -> io::Result<Option<Command>> {
        if let Some(view) = self.views.get_mut(&self.current) {
            return view.handle_event(state);
        }
        return Ok(None);
    }
}

impl Default for ViewManager {
    fn default() -> Self {
        return ViewManager::new();
    }
}
