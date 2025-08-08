use std::io;

mod app;
mod views;
mod commands;
mod runtime;
mod state;

fn main() -> io::Result<()> {
    let terminal = ratatui::init();
    let result = app::App::new().run(terminal);
    ratatui::restore();
    return result;
}
