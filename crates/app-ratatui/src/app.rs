use std::io;

use ratatui::{DefaultTerminal, Frame, widgets::Widget};

use crate::{runtime::RuntimeData, state::ApplicationState, views::ViewManager};

pub struct App {
    viewmanager: ViewManager,
    state: ApplicationState,
    #[allow(unused)]
    runtime: RuntimeData,
}

impl App {
    pub fn new() -> Self {
        return Self {
            viewmanager: ViewManager::default(),
            state: ApplicationState::default(),
            runtime: RuntimeData::default(),
        };
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> io::Result<()> {
        self.state.running = true;
        while self.state.running {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        return Ok(());
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        let viewmanager = &mut self.viewmanager;
        let state = &mut self.state;
        let _command = viewmanager.handle_event(state)?;

        // handle command here
        return Ok(());
    }

    pub fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let state = &self.state;
        let viewmanager = &self.viewmanager;
        viewmanager.render(area, buf, state);
    }
}
