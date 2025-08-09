use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind},
};

use crate::commands::Command;

mod commands;
mod render;

#[derive(Debug, Default)]
pub struct SingleRequestAppState {
    pub running: bool,
}

pub struct SingleRequestApp;

impl SingleRequestApp {
    pub fn new() -> Self {
        return SingleRequestApp;
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        let mut state = SingleRequestAppState::default();

        state.running = true;
        while state.running {
            terminal.draw(|frame| self.draw(frame, &mut state))?;
            self.handle_events(&mut state)?;
        }

        return Ok(());
    }

    fn handle_key_event(
        &mut self,
        key: KeyEvent,
        _state: &mut SingleRequestAppState,
    ) -> Option<Command> {
        let command = match key.code {
            KeyCode::Char('q') => Some(Command::Quit),
            _ => None,
        };

        return command;
    }

    fn handle_events(&mut self, state: &mut SingleRequestAppState) -> anyhow::Result<()> {
        let command = match event::read()? {
            event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event, state)
            }
            _ => None,
        };

        if let Some(command) = command {
            return self.run_command(command, state);
        }

        return Ok(());
    }
}
