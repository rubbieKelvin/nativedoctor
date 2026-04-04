//! Builds the Vue frontend into `frontend/dist` when needed so the server can serve the SPA.
//!
//! # Behaviour
//!
//! - **Release (`PROFILE=release`):** runs **`pnpm install`** (with `--frozen-lockfile` when
//!   `pnpm-lock.yaml` exists) then **`pnpm build`**, producing `frontend/dist/`, which is **embedded**
//!   into the binary via [`rust_embed::RustEmbed`].
//! - **Debug (`PROFILE=debug`):** does **not** embed assets; the API serves files from `frontend/dist/`
//!   when present. A **debug** `nd_web::run_web` also starts **`pnpm run dev`** (Vite on port 5173, HMR)
//!   unless **`ND_WEB_SKIP_VITE_DEV=1`**; Vite proxies `/api` to the Rust server. To keep Cargo rebuilds
//!   fast, **`pnpm build` is skipped** in this script when `frontend/dist/index.html` already exists. If
//!   `dist/` is missing (fresh clone), this script runs `pnpm install` + `pnpm build` once so `cargo test`
//!   and opening the API URL without Vite still find assets.
//! - Set **`ND_WEB_SKIP_FRONTEND_BUILD=1`** to skip `pnpm` entirely. Then **`frontend/dist/index.html`**
//!   must already exist or this script panics.
//!
//! # Rebuild triggers
//!
//! Emits `cargo:rerun-if-changed` for lockfiles, config, HTML entry, and every file under `frontend/src/`
//! so edits to the UI invalidate the build without touching Rust sources.

use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let frontend = manifest_dir.join("frontend");
    let dist_index = frontend.join("dist/index.html");
    let profile = env::var("PROFILE").unwrap_or_default();
    let is_release = profile == "release";

    // rebuild if we change build
    println!(
        "cargo:rerun-if-changed={}",
        manifest_dir.join("build.rs").display()
    );

    rerun_if_changed_fe(&frontend);
    run_pnpm_frontend_build(&frontend, &dist_index, is_release);
}

fn rerun_if_changed_fe(frontend: &std::path::Path) {
    // rebuild if we change these
    for rel in [
        "package.json",
        "pnpm-lock.yaml",
        "vite.config.ts",
        "tsconfig.json",
        "components.json",
        "tsconfig.json",
        "tsconfig.node.json",
        "index.html",
    ] {
        println!("cargo:rerun-if-changed={}", frontend.join(rel).display());
    }

    let src = frontend.join("src");

    if src.is_dir() {
        for entry in walkdir::WalkDir::new(&src)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            if entry.file_type().is_file() {
                println!("cargo:rerun-if-changed={}", entry.path().display());
            }
        }
    }
}

fn run_pnpm_frontend_build(
    frontend: &std::path::Path,
    dist_index: &std::path::Path,
    is_release: bool,
) {
    if is_release {
        build_fe_on_release(frontend, dist_index);
    }
}

fn build_fe_on_release(frontend: &std::path::Path, dist_index: &std::path::Path) {
    // install front end packages with pnpm
    let mut install = Command::new("pnpm");
    install
        .current_dir(frontend)
        .arg("install")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());

    if frontend.join("pnpm-lock.yaml").is_file() {
        install.arg("--frozen-lockfile");
    }

    let status = install.status().unwrap_or_else(|e| {
        panic!(
            "failed to run `pnpm install` in {}: {e}. Is `pnpm` on PATH?",
            frontend.display()
        );
    });

    if !status.success() {
        panic!(
            "`pnpm install` failed with status {status} in {}",
            frontend.display()
        );
    }

    // build frontend
    let status = Command::new("pnpm")
        .current_dir(frontend)
        .args(["run", "build"])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .env("VITE_NATIVEDOCTOR_VERSION", env!("CARGO_PKG_VERSION"))
        .env("VITE_NATIVEDOCTOR_API_ENDPOINT", "")
        .status()
        .unwrap_or_else(|e| {
            panic!(
                "failed to run `pnpm run build` in {}: {e}. Is `pnpm` on PATH?",
                frontend.display()
            );
        });

    if !status.success() {
        panic!(
            "`pnpm run build` failed with status {status} in {}",
            frontend.display()
        );
    }

    if !dist_index.is_file() {
        panic!("expected `{}` after frontend build", dist_index.display());
    }
}
