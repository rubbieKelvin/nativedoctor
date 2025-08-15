use models::direction::Direction;
use strum::IntoEnumIterator;

use crate::app::request::{
    SingleRequestApp, SingleRequestAppState,
    enums::{ActiveInput, Command, InputState, RequestMethod, RequestTab},
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
            Command::StartEditing(which) => self.command_start_editing(state, which),
            Command::StopEditing => self.command_stop_editing(state),
            Command::RotateHttpMethod(dir) => self.command_rotate_method(state, dir),
            Command::RotateRequestTab(dir) => self.command_rotate_request_tab(state, dir),
            Command::SendRequest => self.command_send_request(state),
            Command::ToggleRequestOutputPane => self.command_toggle_req_output(state),
        }

        return Ok(());
    }

    fn command_toggle_req_output(&mut self, state: &mut SingleRequestAppState) {
        state.output_pane_visible = !state.output_pane_visible;
    }

    fn command_quit(&mut self, state: &mut SingleRequestAppState) {
        state.running = false;
    }

    fn command_send_request(&mut self, _state: &mut SingleRequestAppState) {
        todo!()
    }

    fn command_start_editing(&mut self, _state: &mut SingleRequestAppState, which: ActiveInput) {
        self.input_state = InputState::Editing { which };
    }

    fn command_stop_editing(&mut self, state: &mut SingleRequestAppState) {
        // When we hit enter, we want to get the value from the input state and put it in our model
        if let InputState::Editing { which } = &self.input_state {
            match which {
                ActiveInput::RequestUrl => {
                    state.model_state.url = self.url_input_state.value.clone();
                }
                ActiveInput::RequestTitle => {
                    state.model_state.name = self.title_input_state.value.clone();
                }
            }
        }

        // then set the input state back to normal
        self.input_state = InputState::Normal;
    }

    fn command_rotate_method(&mut self, state: &mut SingleRequestAppState, dir: Direction) {
        dir.apply_usize(&mut state.method_index, RequestMethod::iter().count());
    }

    fn command_rotate_request_tab(&mut self, state: &mut SingleRequestAppState, dir: Direction) {
        dir.apply_usize(&mut state.request_tab_index, RequestTab::iter().count());
    }
}
