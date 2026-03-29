//! `nativedoctor run <FILE>` and top-level `FILE` shorthand: single request, or `run --sequence <FILE>` for sequences.

use std::path::Path;

use nd_core::{
    execute_request_post_script, execute_request_with_env, execute_sequence,
    format_prepared_request, load_request_file, load_sequence_file, prepare_request_file,
    prepare_request_with_env, OutcomePolicy, RunOptions, RuntimeEnv,
};

use crate::{print::print_result, Cli, Command};

pub fn run_opts(cli: &Cli) -> RunOptions {
    return match cli.command {
        Some(Command::Run {
            no_post,
            dry_run,
            allow_error_status,
            ..
        }) => RunOptions {
            verbose: cli.verbose,
            no_post_script: no_post,
            dry_run,
            allow_error_status: allow_error_status,
            outcome_policy: OutcomePolicy::SingleRequest,
        },
        _ => RunOptions {
            verbose: cli.verbose,
            no_post_script: false,
            dry_run: false,
            allow_error_status: false,
            outcome_policy: OutcomePolicy::SingleRequest,
        },
    };
}

/// Run or dry-run a single request file; prints human-readable output to stdout/stderr.
pub async fn run_one(path: &Path, cli: &Cli, opts: RunOptions) -> Result<(), String> {
    // Print a header if we're on verbose and no dry run
    if opts.verbose && !opts.dry_run {
        println!("--- request ---");
    }

    // print request summary on dry run or verbose
    if opts.dry_run || opts.verbose {
        // Print the request summary and return. no need to do aything further
        let (prep, _) = prepare_request_file(path).map_err(|e| e.to_string())?;
        let summary = format_prepared_request(&prep).map_err(|e| e.to_string())?;
        println!("{summary}");

        // End execuation (no net I/O)
        if opts.dry_run {
            return Ok(());
        }
    }

    if cli.verbose {
        println!("\n--- response ---");
    }

    // create runtime env
    let env = RuntimeEnv::from_process_env();

    // execute request file
    let output = execute_request_with_env(path, &opts, &env)
        .await
        .map_err(|e| e.to_string())?;

    // print response output
    print_result(&output, cli.verbose)?;

    // execute post request script
    execute_request_post_script(&output, &opts, &env).map_err(|e| e.to_string())?;

    return Ok(());
}

/// Run a sequence file: shared [`RuntimeEnv`], [`OutcomePolicy::SequenceStep`] per step.
pub async fn run_sequence(path: &Path, cli: &Cli, opts: RunOptions) -> Result<(), String> {
    if opts.dry_run {
        let (seq, base_dir) = load_sequence_file(path).map_err(|e| e.to_string())?;
        if seq.steps.is_empty() {
            return Err("sequence must contain at least one step".to_string());
        }
        if let Some(n) = &seq.name {
            println!("# sequence: {}\n", n);
        }
        let env = RuntimeEnv::from_process_env();
        let n = seq.steps.len();

        for (i, step) in seq.steps.iter().enumerate() {
            let step_path = base_dir.join(&step.file);

            if !step_path.is_file() {
                return Err(format!(
                    "sequence step request file not found: {}",
                    step_path.display()
                ));
            }

            let (step_doc, _) = load_request_file(&step_path).map_err(|e| e.to_string())?;
            let step_label = step_doc
                .name
                .as_deref()
                .map(|s| format!(" [{}]", s))
                .unwrap_or_default();

            let (prep, _) =
                prepare_request_with_env(&step_path, &env).map_err(|e| e.to_string())?;

            let summary = format_prepared_request(&prep).map_err(|e| e.to_string())?;

            println!(
                "--- dry-run step {} / {}{} ({}) ---\n{}",
                i + 1,
                n,
                step_label,
                step_path.display(),
                summary
            );
        }
        return Ok(());
    }

    let out = execute_sequence(path, &opts)
        .await
        .map_err(|e| e.to_string())?;

    for sum in &out.steps {
        let label = sum
            .result
            .request_name
            .as_deref()
            .map(|s| format!(" [{}]", s))
            .unwrap_or_default();
        println!(
            "step {}/{}{} {} -> {} ({:?})",
            sum.index,
            sum.total,
            label,
            sum.path.display(),
            sum.result.status,
            sum.result.duration
        );
        if cli.verbose {
            print_result(&sum.result, true)?;
        }
    }
    return Ok(());
}
