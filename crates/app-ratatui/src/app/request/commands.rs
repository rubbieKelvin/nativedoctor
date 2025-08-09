use crate::{
    app::request::{SingleRequestApp, SingleRequestAppState},
    commands::Command,
};

impl SingleRequestApp {
    pub fn run_command(
        &mut self,
        command: Command,
        state: &mut SingleRequestAppState,
    ) -> anyhow::Result<()> {
        match command {
            Command::Quit => self.command_quit(state),
        }

        return Ok(());
    }

    fn command_quit(&mut self, state: &mut SingleRequestAppState) {
        state.running = false;
    }
}
