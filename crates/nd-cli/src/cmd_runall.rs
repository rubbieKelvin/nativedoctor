//! `nativedoctor runall`: multiple request or sequence files, optional shared runtime environment.

use std::path::PathBuf;

use nd_core::RunOptions;

use crate::cmd_run::{
    build_runtime_env, run_one, run_one_with_env, run_sequence, run_sequence_with_env,
};
use crate::Cli;

/// Run multiple request files or multiple sequence files (`sequence` selects the mode for every path).
pub async fn run_all(
    files: &[PathBuf],
    sequence: bool,
    retain_runtime: bool,
    quit_on_failure: bool,
    cli: &Cli,
    opts: RunOptions,
) -> Result<(), String> {
    if files.is_empty() {
        return Ok(());
    }

    let multi = files.len() > 1;
    let mut failures: Vec<(PathBuf, String)> = Vec::new();

    if retain_runtime {
        let env = build_runtime_env(cli)?;

        for path in files {
            if multi && opts.verbose {
                println!("--- runall: {} ---", path.display());
            }
            let r = if sequence {
                run_sequence_with_env(path, cli, opts.clone(), &env).await
            } else {
                run_one_with_env(path, cli, opts.clone(), &env).await
            };
            match r {
                Ok(()) => {}
                Err(e) => {
                    if quit_on_failure {
                        return Err(e);
                    }
                    failures.push((path.clone(), e));
                }
            }
        }
    } else {
        for path in files {
            if multi && opts.verbose {
                println!("--- runall: {} ---", path.display());
            }
            let r = if sequence {
                run_sequence(path, cli, opts.clone()).await
            } else {
                run_one(path, cli, opts.clone()).await
            };
            match r {
                Ok(()) => {}
                Err(e) => {
                    if quit_on_failure {
                        return Err(e);
                    }
                    failures.push((path.clone(), e));
                }
            }
        }
    }

    if failures.is_empty() {
        return Ok(());
    } else {
        let mut msg = format!("{} of {} file(s) failed:\n", failures.len(), files.len());
        for (p, e) in &failures {
            msg.push_str(&format!("  {}: {}\n", p.display(), e));
        }
        return Err(msg.trim_end_matches('\n').to_string());
    }
}
