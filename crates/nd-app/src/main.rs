mod app;
mod files;
mod ui;

fn main() {
    // Initialise tracing for debug logs.
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "nd_app=info,warn".into()),
        )
        .init();

    tracing::info!("Starting NativeDoctor desktop application");

    app::setup();
}
