use crate::{
    app::request::{InputState, SingleRequestApp, SingleRequestAppState},
    commands::{ActiveInput, Command},
};

impl SingleRequestApp {
    pub fn run_command(
        &mut self,
        command: Command,
        state: &mut SingleRequestAppState,
    ) -> anyhow::Result<()> {
        match command {
            Command::Quit => self.command_quit(state),
            Command::StartEditing(which) => self.command_start_editing(state, which),
            Command::StopEditing => self.command_stop_editing(state),
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
}
