//! Dioxus server bootstrap: static `public` dir and bind address env vars.

use std::net::SocketAddr;
use std::path::Path;

fn copy_file(src: &Path, dst: &Path) -> Result<(), String> {
    if let Some(parent) = dst.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("create parent dir {}: {e}", parent.display()))?;
    }
    std::fs::copy(src, dst)
        .map_err(|e| format!("copy {} -> {}: {e}", src.display(), dst.display()))?;
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<(), String> {
    std::fs::create_dir_all(dst).map_err(|e| format!("create dir {}: {e}", dst.display()))?;
    for entry in std::fs::read_dir(src).map_err(|e| format!("read dir {}: {e}", src.display()))? {
        let entry = entry.map_err(|e| e.to_string())?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            copy_file(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

/// Ensure a `public` directory exists, populate it with crate assets, and point Dioxus at it.
pub fn ensure_public_dir() -> Result<(), String> {
    let exe = std::env::current_exe().map_err(|e| e.to_string())?;
    let exe_dir = exe
        .parent()
        .ok_or_else(|| "could not resolve executable directory".to_string())?;
    let public_dir = exe_dir.join("public");
    std::fs::create_dir_all(&public_dir).map_err(|e| format!("create Dioxus public dir: {e}"))?;

    let crate_root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let assets_src = crate_root.join("assets");
    let assets_dst = public_dir.join("assets");
    if assets_src.exists() {
        copy_dir_recursive(&assets_src, &assets_dst)?;
    }

    let tailwind_src = crate_root.join("tailwind.css");
    if tailwind_src.exists() {
        copy_file(&tailwind_src, &public_dir.join("tailwind.css"))?;
    }

    std::env::set_var("DIOXUS_PUBLIC_PATH", &public_dir);
    Ok(())
}

pub fn set_server_listen_addr(bind: SocketAddr) {
    std::env::set_var(dioxus::cli_config::SERVER_IP_ENV, bind.ip().to_string());
    std::env::set_var(dioxus::cli_config::SERVER_PORT_ENV, bind.port().to_string());
}
