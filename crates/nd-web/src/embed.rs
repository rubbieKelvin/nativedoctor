//! Vite production assets embedded at compile time via [`rust_embed::RustEmbed`].
//!
//! The crate `build.rs` script runs `pnpm build` before compilation so `frontend/dist` exists.
//! Unknown asset paths fall back to `index.html` for client-side (Vue) routing.

use axum::body::Body;
use axum::http::{header, Response, StatusCode};
use rust_embed::RustEmbed;

/// Files under `frontend/dist` (populated by `pnpm build` before compile).
#[derive(RustEmbed)]
#[folder = "frontend/dist/"]
pub struct WebAssets;

/// Serves one embedded file with a guessed `Content-Type`, or `index.html` when the path has no match (SPA fallback).
///
/// # Path rules
///
/// - Leading slashes are stripped; empty path maps to `index.html`.
/// - Path segments containing `..` return **404** (no directory traversal).
/// - Any other missing asset falls back to **`index.html`** so Vue Router paths work.
pub fn embedded_static_response(uri_path: &str) -> Response<Body> {
    let mut key = uri_path.trim().trim_start_matches('/').to_string();

    if key.contains("..") || key.contains('\\') {
        return not_found();
    }

    if key.is_empty() {
        key = "index.html".into();
    }

    let file = WebAssets::get(&key).or_else(|| {
        if key == "index.html" {
            return None;
        } else {
            return WebAssets::get("index.html");
        }
    });

    let Some(content) = file else {
        return not_found();
    };

    let mime = mime_guess::from_path(&key).first_or_octet_stream();

    return Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime.as_ref())
        .body(Body::from(content.data.into_owned()))
        .unwrap_or_else(|_| not_found());
}

fn not_found() -> Response<Body> {
    return Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap();
}
