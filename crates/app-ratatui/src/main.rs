use nd_core::constants::APPLICATION_NAME;

mod app;
mod runtime;
mod style;
mod widgets;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    match app::request::SingleRequestApp::new().run(terminal) {
        Ok(_) => {
            println!("{} running", APPLICATION_NAME);
        }
        Err(e) => {
            eprintln!("Error running application. {}", e);
        }
    };
    ratatui::restore();
    return Ok(());
}
