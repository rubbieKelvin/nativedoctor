//! Visual tokens for semantic HTTP artefacts (methods and status badges).
//!
//! Structural chrome inherits hues from gpui-component’s [`gpui_component::Theme`].

use gpui::{hsla, white};
use gpui_component::{Theme, ThemeMode};

/// Apply NativeDoctor tweaks on top of gpui-component’s bundled dark palettes.
///
/// Must run immediately after [`gpui_component::init`] so globals are bootstrapped.
pub fn apply_native_doctor_theme(cx: &mut gpui::App) {
    Theme::change(ThemeMode::Dark, None, cx);
    Theme::sync_scrollbar_appearance(cx);

    let colors = Theme::global_mut(cx);
    colors.primary = hsla(158. / 360., 1., 0.26, 1.);
    colors.primary_hover = hsla(152. / 360., 0.92, 0.36, 1.);
    colors.primary_active = hsla(152. / 360., 1., 0.28, 1.);
    colors.primary_foreground = white();
}

#[inline(always)]
pub fn muted_neutral() -> gpui::Hsla {
    hsla(210. / 360., 0.10, 0.55, 1.)
}

/// Map HTTP verbs to deterministic accent hues for rows and badges.
pub fn method_color(method: &str) -> gpui::Hsla {
    return match method.to_uppercase().as_str() {
        "GET" => hsla(210. / 360., 0.90, 0.61, 1.),
        "POST" => hsla(137. / 360., 0.45, 0.48, 1.),
        "PUT" => hsla(41. / 360., 0.86, 0.57, 1.),
        "PATCH" => hsla(41. / 360., 0.86, 0.57, 1.),
        "DELETE" => hsla(3. / 360., 0.93, 0.63, 1.),
        "HEAD" => hsla(210. / 360., 0.10, 0.65, 1.),
        "OPTIONS" => hsla(210. / 360., 0.10, 0.65, 1.),
        _ => hsla(210. / 360., 0.10, 0.65, 1.),
    };
}

/// Map HTTP statuses into traffic-light hues for banner chips.
pub fn status_color(status: u16) -> gpui::Hsla {
    return match status {
        0 => muted_neutral(),
        100..=199 => muted_neutral(),
        200..=299 => hsla(137. / 360., 0.45, 0.48, 1.),
        300..=399 => hsla(207. / 360., 0.90, 0.61, 1.),
        400..=499 => hsla(41. / 360., 0.86, 0.57, 1.),
        500..=599 => hsla(3. / 360., 0.93, 0.63, 1.),
        _ => muted_neutral(),
    };
}
