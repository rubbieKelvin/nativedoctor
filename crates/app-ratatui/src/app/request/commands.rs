use std::thread::spawn;

use nd_core::{direction::Direction, iterself::CircularIterSelf};

use crate::app::request::{
    SingleRequestApp, SingleRequestAppState,
    enums::{ActiveInput, ApplicationEvent, Command, InputState},
};

impl SingleRequestApp {
    /// Handles all commands gotten from the user/system
    pub fn run_command(
        &mut self,
        command: Command,
        state: &mut SingleRequestAppState,
    ) -> anyhow::Result<()> {
        match command {
            Command::Quit => self.command_quit(state),
            Command::StartTextEditing(which) => self.command_start_editing(state, which),
            Command::FinishTextEditing => self.command_stop_editing(state),
            Command::AbortTextEditing => self.command_abort_editing(state),
            Command::RotateHttpMethod => self.command_rotate_method(state),
            Command::RotateRequestTab(dir) => self.command_rotate_request_tab(state, dir),
            Command::SendRequest => self.command_send_request(state),
            Command::ToggleRequestOutputPane => self.command_toggle_req_output(state),
            Command::RotateResponseTab(dir) => self.command_rotate_response_tab(state, dir),
        }

        return Ok(());
    }

    fn command_toggle_req_output(&mut self, state: &mut SingleRequestAppState) {
        state.output_pane_visible = !state.output_pane_visible;
    }

    fn command_quit(&mut self, state: &mut SingleRequestAppState) {
        state.running = false;
    }

    fn command_send_request(&mut self, state: &mut SingleRequestAppState) {
        if state.is_making_request {
            return;
        }

        let model = state.requestmodel.clone();
        let tx = self.event_transmitter.clone().unwrap();
        let request = self
            .executor
            .build_request(model)
            // We could manage this error better
            .unwrap();

        // send the request in a new thread
        state.is_making_request = true;
        spawn(move || -> Result<(), anyhow::Error> {
            let response = request.send();
            tx.send(ApplicationEvent::HttpRequestCallCompleted(response))?;
            return Ok(());
        });
    }

    fn command_start_editing(&mut self, _state: &mut SingleRequestAppState, which: ActiveInput) {
        self.input_state = InputState::Editing { which };
    }

    fn command_abort_editing(&mut self, state: &mut SingleRequestAppState) {
        // revert input to previous state
        if let InputState::Editing { which } = &self.input_state {
            match which {
                ActiveInput::RequestUrl => {
                    self.url_input_state.value = state.requestmodel.url.clone();
                }
                ActiveInput::RequestTitle => {
                    self.title_input_state.value = state.requestmodel.name.clone();
                }
            }
        }

        // then set the input state back to normal
        self.input_state = InputState::Normal;
    }

    fn command_stop_editing(&mut self, state: &mut SingleRequestAppState) {
        // When we hit enter, we want to get the value from the input state and put it in our model
        if let InputState::Editing { which } = &self.input_state {
            match which {
                ActiveInput::RequestUrl => {
                    state.requestmodel.url = self.url_input_state.value.clone();
                }
                ActiveInput::RequestTitle => {
                    state.requestmodel.name = self.title_input_state.value.clone();
                }
            }
        }

        // then set the input state back to normal
        self.input_state = InputState::Normal;
    }

    fn command_rotate_method(&mut self, state: &mut SingleRequestAppState) {
        let meth = &mut state.requestmodel.method;
        meth.movecursor(Direction::Right);
    }

    fn command_rotate_request_tab(&mut self, state: &mut SingleRequestAppState, dir: Direction) {
        state.request_tab.movecursor(dir);
    }

    fn command_rotate_response_tab(&mut self, state: &mut SingleRequestAppState, dir: Direction) {
        state.response_tab.movecursor(dir);
    }
}
