use gpui::{App, SharedString};
use gpui_component::{Theme, ThemeRegistry};
use std::path::PathBuf;

pub fn init(cx: &mut App) {
    let theme_name = SharedString::from("Catppuccin Frappe");
    if let Err(err) = ThemeRegistry::watch_dir(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("themes"),
        cx,
        move |cx| {
            if let Some(theme) = ThemeRegistry::global(cx).themes().get(&theme_name).cloned() {
                Theme::global_mut(cx).apply_config(&theme);
            }
        },
    ) {
        tracing::error!("Failed to watch themes directory: {}", err);
    }
}
