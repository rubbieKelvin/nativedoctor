//! Entry point for the **NativeDoctor** desktop application.
//!
//! Uses **GPUI** (GPU-accelerated UI framework from Zed) to render a
//! Postman-like API testing & development tool backed by a local SQLite
//! database (see `nd-db`).

mod app;
mod components;
mod pages;
mod project_tasks;
mod state;
mod theme;

fn main() {
    // Initialise tracing for debug logs.
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nd_app=info,warn".into()),
        )
        .init();

    tracing::info!("Starting NativeDoctor desktop application");

    // Launch the GPUI application. The `app::NativeDoctorApp` owns all
    // top-level state and decides which page to render.
    app::NativeDoctorApp::run();
}
