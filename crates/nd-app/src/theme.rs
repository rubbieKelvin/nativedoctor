//! Colour palette functions for the NativeDoctor dark theme.
//! All return `gpui::Rgba` and are callable directly (e.g. `bg_dark()`).

use gpui::{rgb, Rgba};

// ── Brand colours ────────────────────────────────────────────────────────

pub fn green() -> Rgba {
    rgb(0x008751)
}
pub fn green_light() -> Rgba {
    rgb(0x00a864)
}
pub fn green_dark() -> Rgba {
    rgb(0x00663e)
}

// ── Neutral palette (dark theme) ─────────────────────────────────────────

pub fn bg_darkest() -> Rgba {
    rgb(0x0d1117)
}
pub fn bg_dark() -> Rgba {
    rgb(0x161b22)
}
pub fn bg_mid() -> Rgba {
    rgb(0x21262d)
}
pub fn bg_light() -> Rgba {
    rgb(0x30363d)
}
pub fn border() -> Rgba {
    rgb(0x30363d)
}
pub fn border_focus() -> Rgba {
    rgb(0x008751)
}

pub fn text_primary() -> Rgba {
    rgb(0xe6edf3)
}
pub fn text_secondary() -> Rgba {
    rgb(0x8b949e)
}
pub fn text_muted() -> Rgba {
    rgb(0x6e7681)
}

// ── Semantic colours ─────────────────────────────────────────────────────

pub fn status_success() -> Rgba {
    rgb(0x3fb950)
}
pub fn status_redirect() -> Rgba {
    rgb(0x58a6ff)
}
pub fn status_client_error() -> Rgba {
    rgb(0xd29922)
}
pub fn status_server_error() -> Rgba {
    rgb(0xf85149)
}
pub fn status_neutral() -> Rgba {
    rgb(0x8b949e)
}

/// Map an HTTP status code to a semantic colour.
pub fn status_color(status: u16) -> Rgba {
    match status {
        0 => status_neutral(),
        100..=199 => status_neutral(),
        200..=299 => status_success(),
        300..=399 => status_redirect(),
        400..=499 => status_client_error(),
        500..=599 => status_server_error(),
        _ => status_neutral(),
    }
}

/// Map an HTTP method to a colour for visual distinction.
pub fn method_color(method: &str) -> Rgba {
    match method.to_uppercase().as_str() {
        "GET" => rgb(0x58a6ff),
        "POST" => rgb(0x3fb950),
        "PUT" => rgb(0xd29922),
        "PATCH" => rgb(0xd29922),
        "DELETE" => rgb(0xf85149),
        "HEAD" => rgb(0x8b949e),
        "OPTIONS" => rgb(0x8b949e),
        _ => rgb(0x8b949e),
    }
}
