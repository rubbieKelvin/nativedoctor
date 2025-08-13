use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind},
};

use crate::app::request::{enums::{
    ActiveInput, Command, Direction, InputState, RequestTab, ResponseTab,
}, state::SingleRequestAppState};

mod commands;
mod enums;
mod render;
mod state;

#[derive(Default)]
pub struct SingleRequestApp;

impl SingleRequestApp {
    pub fn new() -> Self {
        return SingleRequestApp::default();
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
        state: &mut SingleRequestAppState,
    ) -> Option<Command> {
        let command = match key.code {
            KeyCode::Char('q') => match state.input_state {
                InputState::Normal => Some(Command::Quit),
                InputState::Editing { .. } => None,
            },
            KeyCode::Char('u') => match state.input_state {
                InputState::Normal => Some(Command::StartEditing(ActiveInput::RequestUrl)),
                InputState::Editing { .. } => None,
            },
            KeyCode::Char('t') => match state.input_state {
                InputState::Normal => Some(Command::StartEditing(ActiveInput::RequestTitle)),
                InputState::Editing { .. } => None,
            },
            KeyCode::Char('m') => match state.input_state {
                InputState::Normal => Some(Command::RotateHttpMethod(Direction::Right)),
                InputState::Editing { .. } => None,
            },
            KeyCode::Char('1') => match state.input_state {
                InputState::Normal => Some(Command::ToggleRequestOutputPane),
                InputState::Editing { .. } => None,
            },
            KeyCode::Enter | KeyCode::Esc => match state.input_state {
                InputState::Editing { .. } => Some(Command::StopEditing),
                InputState::Normal => None,
            },
            KeyCode::Left => match state.input_state {
                InputState::Normal => Some(Command::RotateRequestTab(Direction::Left)),
                InputState::Editing { .. } => None,
            },
            KeyCode::Right => match state.input_state {
                InputState::Normal => Some(Command::RotateRequestTab(Direction::Right)),
                InputState::Editing { .. } => None,
            },
            _ => None,
        };

        // handle text input
        let input_state = state.input_state.clone();

        if let InputState::Editing { which } = input_state {
            // get the pointer to the string we'll like to manipulate
            let active_buffer = match which {
                ActiveInput::RequestUrl => &mut state.url.value,
                ActiveInput::RequestTitle => &mut state.name.value,
            };

            match key.code {
                KeyCode::Char(n) => {
                    active_buffer.push(n);
                }
                KeyCode::Backspace => {
                    active_buffer.pop();
                }
                _ => {}
            };
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
