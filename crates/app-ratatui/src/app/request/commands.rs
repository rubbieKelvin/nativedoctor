use crate::{
    app::request::{
        InputState, RequestMethod, RequestTab, SingleRequestApp, SingleRequestAppState,
    },
    commands::{ActiveInput, Command, XDirection},
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
            Command::RotateHttpMethod => self.command_rotate_method(state),
            Command::RotateRequestTab(dir) => self.command_rotate_request_tab(state, dir),
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

    fn command_rotate_method(&mut self, state: &mut SingleRequestAppState) {
        let methods = RequestMethod::all();
        let index = methods
            .iter()
            .position(|c| c.clone() == state.method.clone())
            .unwrap_or(0);
        let new = methods.get((index + 1) % methods.len()).map(|m| m.clone());
        state.method = new.unwrap_or_default();
    }

    fn command_rotate_request_tab(&mut self, state: &mut SingleRequestAppState, dir: XDirection) {
        let dir: i32 = match dir {
            XDirection::Left => -1,
            XDirection::Right => 1,
        };

        let methods = RequestTab::all();
        let index = methods
            .iter()
            .position(|c| c.clone() == state.request_tab.clone())
            .unwrap_or(0);
        let new = methods
            .get(((index as i32 + dir) % methods.len() as i32) as usize)
            .map(|m| m.clone());
        state.request_tab = new.unwrap_or_default();
    }
}
