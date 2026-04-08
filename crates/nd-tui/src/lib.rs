//! Terminal UI for running nativedoctor request files and Rhai scripts ([ratatui](https://ratatui.rs/)).

mod app;
mod runner;
mod ui;

pub use runner::{run_tui, TuiOptions};
