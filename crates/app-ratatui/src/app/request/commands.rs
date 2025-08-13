use strum::IntoEnumIterator;

use crate::app::request::{
    SingleRequestApp, SingleRequestAppState,
    enums::{ActiveInput, Command, Direction, InputState, RequestMethod, RequestTab},
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
            Command::ToggleRequestOutputPane => {
                state.output_pane_visible = !state.output_pane_visible;
            }
        }

        return Ok(());
    }

    fn command_quit(&mut self, state: &mut SingleRequestAppState) {
        state.running = false;
    }

    fn command_start_editing(&mut self, state: &mut SingleRequestAppState, which: ActiveInput) {
        state.input_state = InputState::Editing { which };
    }

    fn command_stop_editing(&mut self, state: &mut SingleRequestAppState) {
        state.input_state = InputState::Normal;
    }

    fn command_rotate_method(&mut self, state: &mut SingleRequestAppState, dir: Direction) {
        dir.apply_usize(&mut state.method_index, RequestMethod::iter().count());
    }

    fn command_rotate_request_tab(&mut self, state: &mut SingleRequestAppState, dir: Direction) {
        dir.apply_usize(&mut state.request_tab_index, RequestTab::iter().count());
    }
}
