//! Run optional Rhai `post_script` after a response.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use tracing::debug;

use super::types::RunOptions;
use crate::env::RuntimeEnv;
use crate::error::{Error, Result};
use crate::load::resolve_post_script;
use crate::model::{RequestFile, SequenceStep};
use crate::rhai::logger::Logger;
use crate::rhai::run_post_script;

/// Runs the request file’s post-script when configured and allowed by `opts`.
///
/// A **fresh** [`Logger`] is created per invocation so in-memory log capture is scoped to this
/// run only (nothing is read back today; [`tracing`] still receives every `log()` line regardless).
#[allow(clippy::too_many_arguments)]
pub(crate) fn run_request_post_script(
    doc: &RequestFile,
    base_dir: &Path,
    env: &RuntimeEnv,
    opts: &RunOptions,
    status: u16,
    resp_headers: &[(String, String)],
    body: &[u8],
    persist_file: Option<PathBuf>,
) -> Result<()> {
    if let Some(rel) = &doc.post_script {
        if !opts.no_post_script {
            let script_path = resolve_post_script(base_dir, rel);
            if !script_path.is_file() {
                return Err(Error::PostScriptNotFound(script_path));
            }
            debug!(
                script = %script_path.display(),
                http_status = status,
                "running post_script"
            );

            let logger = Arc::new(Logger::new());
            run_post_script(
                &script_path,
                env,
                status,
                resp_headers,
                body,
                Some(logger),
                persist_file,
            )?;
        }
    }
    Ok(())
}

/// Rhai scripts listed on [`SequenceStep::post_scripts`], resolved from the sequence file directory.
#[allow(clippy::too_many_arguments)]
pub(crate) fn run_sequence_flow_post_scripts(
    step: &SequenceStep,
    sequence_base_dir: &Path,
    env: &RuntimeEnv,
    opts: &RunOptions,
    status: u16,
    resp_headers: &[(String, String)],
    body: &[u8],
    persist_file: Option<PathBuf>,
) -> Result<()> {
    if opts.no_post_script {
        return Ok(());
    }

    for rel in &step.post_scripts {
        let script_path = resolve_post_script(sequence_base_dir, rel);
        if !script_path.is_file() {
            return Err(Error::PostScriptNotFound(script_path));
        }

        debug!(
            script = %script_path.display(),
            http_status = status,
            "running sequence post_script"
        );

        let logger = Arc::new(Logger::new());
        run_post_script(
            &script_path,
            env,
            status,
            resp_headers,
            body,
            Some(logger),
            persist_file.clone(),
        )?;
    }
    return Ok(());
}
