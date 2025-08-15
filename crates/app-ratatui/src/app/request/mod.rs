use std::{
    sync::mpsc::{self, Sender},
    thread::spawn,
};

use models::direction::Direction;
use nd_core::executor::Executor;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, KeyCode, KeyEvent, KeyEventKind},
};

use crate::{
    app::request::{
        enums::{ActiveInput, ApplicationEvent, Command, InputState, RequestTab, ResponseTab},
        state::SingleRequestAppState,
    },
    widgets::input::TextInputState,
};

mod commands;
mod enums;
mod render;
mod state;

#[derive(Default)]
pub struct SingleRequestApp {
    pub input_state: InputState,
    pub url_input_state: TextInputState,
    pub title_input_state: TextInputState,
    pub executor: Executor,
    pub event_transmitter: Option<Sender<ApplicationEvent>>,
}

impl SingleRequestApp {
    pub fn new() -> Self {
        return SingleRequestApp::default();
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        let mut state = SingleRequestAppState::default();
        state.running = true;

        let (tx, rx) = mpsc::channel::<ApplicationEvent>();
        self.event_transmitter = Some(tx.clone());

        // spawn draw and input event ls
        // here we're basically listening for crossterm events
        spawn(move || -> anyhow::Result<()> {
            loop {
                let ev = event::read()?;
                tx.send(ApplicationEvent::Input(ev))?;
            }
        });

        // block on the multiple procuder single consumer channel
        while state.running {
            terminal.draw(|frame| self.draw(frame, &mut state))?;
            let app_event = rx.recv()?;
            self.handle_events(app_event, &mut state)?;
        }

        return Ok(());
    }

    fn handle_key_event(
        &mut self,
        key: KeyEvent,
        _state: &mut SingleRequestAppState,
    ) -> Option<Command> {
        let command = match key.code {
            KeyCode::Char('q') => match self.input_state {
                InputState::Normal => Some(Command::Quit),
                InputState::Editing { .. } => None,
            },
            KeyCode::Char('u') => match self.input_state {
                InputState::Normal => Some(Command::StartTextEditing(ActiveInput::RequestUrl)),
                InputState::Editing { .. } => None,
            },
            KeyCode::Char('t') => match self.input_state {
                InputState::Normal => Some(Command::StartTextEditing(ActiveInput::RequestTitle)),
                InputState::Editing { .. } => None,
            },
            KeyCode::Char('m') => match self.input_state {
                InputState::Normal => Some(Command::RotateHttpMethod),
                InputState::Editing { .. } => None,
            },
            KeyCode::Char('1') => match self.input_state {
                InputState::Normal => Some(Command::ToggleRequestOutputPane),
                InputState::Editing { .. } => None,
            },
            KeyCode::Enter => match self.input_state {
                InputState::Editing { .. } => Some(Command::FinishTextEditing),
                InputState::Normal => Some(Command::SendRequest),
            },
            KeyCode::Esc => match self.input_state {
                InputState::Editing { .. } => Some(Command::AbortTextEditing),
                InputState::Normal => None,
            },
            KeyCode::Left => match self.input_state {
                InputState::Normal => Some(Command::RotateRequestTab(Direction::Left)),
                InputState::Editing { .. } => None,
            },
            KeyCode::Right => match self.input_state {
                InputState::Normal => Some(Command::RotateRequestTab(Direction::Right)),
                InputState::Editing { .. } => None,
            },
            _ => None,
        };

        // handle text input
        let input_state = self.input_state.clone();

        if let InputState::Editing { which } = input_state {
            // get the pointer to the string we'll like to manipulate
            let active_buffer = match which {
                ActiveInput::RequestUrl => &mut self.url_input_state.value,
                ActiveInput::RequestTitle => &mut self.title_input_state.value,
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

    fn handle_events(
        &mut self,
        event: ApplicationEvent,
        state: &mut SingleRequestAppState,
    ) -> anyhow::Result<()> {
        let command = match event {
            // input events
            ApplicationEvent::Input(input_event) => match input_event {
                event::Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event, state)
                }
                _ => None,
            },
            // _ => None,
        };

        if let Some(command) = command {
            return self.run_command(command, state);
        }

        return Ok(());
    }
}
