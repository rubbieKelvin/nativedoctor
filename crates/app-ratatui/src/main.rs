mod app;
mod commands;
mod runtime;
mod state;

fn main() -> anyhow::Result<()> {
    let terminal = ratatui::init();
    let result = app::request::SingleRequestApp::new().run(terminal);
    ratatui::restore();
    return result;
}
