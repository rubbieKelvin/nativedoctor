use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind},
    style::Stylize,
    text::Span,
};

use crate::{
    commands::{ActiveInput, Command, XDirection},
    widgets::input::TextInputState,
};

mod commands;
mod render;

#[derive(Debug, Default, Clone)]
pub enum InputState {
    Editing {
        which: ActiveInput,
    },
    #[default]
    Normal,
}

#[derive(Debug, Default, Clone, strum::Display, PartialEq)]
pub enum RequestMethod {
    #[default]
    Get,
    Delete,
    Post,
    Patch,
    Put,
    Head,
    Option,
}

impl RequestMethod {
    pub fn all() -> Vec<Self> {
        return vec![
            Self::Get,
            Self::Delete,
            Self::Post,
            Self::Patch,
            Self::Put,
            Self::Head,
            Self::Option,
        ];
    }

    fn span<'a>(&self) -> Span<'a> {
        match self {
            Self::Get => self.to_string().green(),
            Self::Delete => self.to_string().red(),
            Self::Post => self.to_string().blue(),
            Self::Patch => self.to_string().magenta(),
            Self::Put => self.to_string().yellow(),
            Self::Head => self.to_string().gray(),
            Self::Option => self.to_string().gray(),
        }
    }
}

#[derive(strum::Display, Default, Clone, PartialEq, Debug)]
pub enum RequestTab {
    Params,
    Header,
    Auth,
    #[default]
    Body,
    Doc,
    Script,
}

impl RequestTab {
    pub fn all() -> Vec<Self> {
        return vec![
            Self::Params,
            Self::Header,
            Self::Auth,
            Self::Body,
            Self::Doc,
            Self::Script,
        ];
    }
}

#[derive(Debug, Default)]
pub struct SingleRequestAppState {
    pub url: TextInputState,
    pub method: RequestMethod,
    pub running: bool,
    pub input_state: InputState,
    pub request_tab: RequestTab,
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
        state: &mut SingleRequestAppState,
    ) -> Option<Command> {
        let command = match key.code {
            KeyCode::Char('q') => match state.input_state {
                InputState::Normal => Some(Command::Quit),
                InputState::Editing { .. } => None,
            },
            KeyCode::Char('u') => match state.input_state {
                InputState::Normal => Some(Command::StartEditing(ActiveInput::Url)),
                InputState::Editing { .. } => None,
            },
            KeyCode::Char('m') => match state.input_state {
                InputState::Normal => Some(Command::RotateHttpMethod),
                InputState::Editing { .. } => None,
            },
            KeyCode::Enter | KeyCode::Esc => match state.input_state {
                InputState::Editing { .. } => Some(Command::StopEditing),
                InputState::Normal => None,
            },
            KeyCode::Left => match state.input_state {
                InputState::Normal => Some(Command::RotateRequestTab(XDirection::Left)),
                InputState::Editing { .. } => None,
            },
            KeyCode::Right => match state.input_state {
                InputState::Normal => Some(Command::RotateRequestTab(XDirection::Right)),
                InputState::Editing { .. } => None,
            },
            _ => None,
        };

        // handle text input
        let input_state = state.input_state.clone();

        if let InputState::Editing { which } = input_state {
            // get the pointer to the string we'll like to manipulate
            let active_buffer = match which {
                ActiveInput::Url => &mut state.url.value,
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
