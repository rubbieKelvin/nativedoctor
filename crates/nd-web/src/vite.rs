//! spawn `pnpm run dev` on debug for Vite HMR.

use std::path::PathBuf;
use std::process::{Child, Command, Stdio};

use nd_constants::VERSION;

const SKIP_ENV: &str = "ND_WEB_SKIP_VITE_DEV";

/// Kills the Vite child process when dropped
pub struct ViteDevGuard(Option<Child>);

impl Drop for ViteDevGuard {
    fn drop(&mut self) {
        if let Some(mut child) = self.0.take() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

fn frontend_dir() -> PathBuf {
    return PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("frontend");
}

fn spawn_pnpm_dev(api_endpoint: &str) -> anyhow::Result<Child> {
    let frontend = frontend_dir();
    // return Command::new(format!("VITE_NATIVEDOCTOR_VERSION={} pnpm", VERSION))
    return Command::new("pnpm")
        .current_dir(&frontend)
        .args(["run", "dev"])
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .env("VITE_NATIVEDOCTOR_VERSION", VERSION)
        .env("VITE_NATIVEDOCTOR_API_ENDPOINT", api_endpoint)
        .spawn()
        .map_err(|e| {
            anyhow::anyhow!(
                "failed to spawn `pnpm run dev` in {}: {e}. Is `pnpm` on PATH?",
                frontend.display()
            )
        });
}

/// Spawns Vite unless `ND_WEB_SKIP_VITE_DEV=1`.
pub fn maybe_start_vite_dev(api_endpoint: &str) -> anyhow::Result<Option<ViteDevGuard>> {
    if std::env::var(SKIP_ENV).ok().as_deref() == Some("1") {
        return Ok(None);
    }
    let child = spawn_pnpm_dev(api_endpoint)?;
    return Ok(Some(ViteDevGuard(Some(child))));
}
