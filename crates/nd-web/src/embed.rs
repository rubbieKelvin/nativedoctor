//! SPA static files: **release** builds embed `frontend/dist` with [`rust_embed::RustEmbed`]; **debug**
//! builds read the same tree from disk at runtime (under [`CARGO_MANIFEST_DIR`]) so dev binaries stay
//! small and `cargo build` does not pack asset bytes into the executable.
//!
//! Unknown asset paths fall back to `index.html` for client-side (Vue) routing.

use axum::body::Body;
use axum::http::{header, Response, StatusCode};

#[cfg(not(debug_assertions))]
use rust_embed::RustEmbed;

#[cfg(not(debug_assertions))]
#[derive(RustEmbed)]
#[folder = "frontend/dist/"]
struct WebAssets;

/// Serves one static file with a guessed `Content-Type`, or `index.html` when the path has no match (SPA fallback).
///
/// # Path rules
///
/// - Leading slashes are stripped; empty path maps to `index.html`.
/// - Path segments containing `..` return **404** (no directory traversal).
/// - Any other missing asset falls back to **`index.html`** so Vue Router paths work.
pub fn embedded_static_response(uri_path: &str) -> Response<Body> {
    #[cfg(not(debug_assertions))]
    {
        serve_embedded(uri_path)
    }
    #[cfg(debug_assertions)]
    {
        serve_from_disk(uri_path)
    }
}

fn normalize_key(uri_path: &str) -> Result<String, ()> {
    let mut key = uri_path.trim().trim_start_matches('/').to_string();

    if key.contains("..") || key.contains('\\') {
        return Err(());
    }

    if key.is_empty() {
        key = "index.html".into();
    }

    Ok(key)
}

#[cfg(not(debug_assertions))]
fn serve_embedded(uri_path: &str) -> Response<Body> {
    let Ok(key) = normalize_key(uri_path) else {
        return not_found();
    };

    let file = WebAssets::get(&key).or_else(|| {
        if key == "index.html" {
            None
        } else {
            WebAssets::get("index.html")
        }
    });

    let Some(content) = file else {
        return not_found();
    };

    let mime = mime_guess::from_path(&key).first_or_octet_stream();

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime.as_ref())
        .body(Body::from(content.data.into_owned()))
        .unwrap_or_else(|_| not_found())
}

#[cfg(debug_assertions)]
fn dist_root() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("frontend/dist")
}

#[cfg(debug_assertions)]
fn serve_from_disk(uri_path: &str) -> Response<Body> {
    let Ok(key) = normalize_key(uri_path) else {
        return not_found();
    };

    let root = dist_root();
    let path = root.join(&key);

    let bytes = std::fs::read(&path).or_else(|_| {
        if key == "index.html" {
            Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "missing index",
            ))
        } else {
            std::fs::read(root.join("index.html"))
        }
    });

    let Ok(bytes) = bytes else {
        return not_found();
    };

    let mime = mime_guess::from_path(&key).first_or_octet_stream();

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, mime.as_ref())
        .body(Body::from(bytes))
        .unwrap_or_else(|_| not_found())
}

fn not_found() -> Response<Body> {
    Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::empty())
        .unwrap()
}
