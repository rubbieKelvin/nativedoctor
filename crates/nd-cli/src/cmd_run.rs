//! `nativedoctor run <FILE>` and top-level `FILE` shorthand: single request, or `run --sequence <FILE>` for sequences.

use std::path::Path;

use colored::Colorize;
use nd_core::{
    execute_request_post_script, execute_request_with_env, expand_hashmap_values,
    format_prepared_request, load_request_file, load_sequence_file, prepare_request_with_env,
    OutcomePolicy, RunOptions, RuntimeEnv,
};

use crate::{print::print_result, Cli, Command};

/// Build [`RuntimeEnv`] from global CLI flags: optional process snapshot, persisted JSON in the
/// **current working directory** (`runtime.nativedoctor.json`), then each `--env` file (later
/// overrides earlier for duplicate keys).
pub fn build_runtime_env(cli: &Cli) -> Result<RuntimeEnv, String> {
    RuntimeEnv::from_cli_options(cli.no_default_system_env, &cli.env).map_err(|e| e.to_string())
}

pub fn run_opts(cli: &Cli) -> RunOptions {
    return match &cli.command {
        Some(Command::Run {
            no_post,
            dry_run,
            allow_error_status,
            ..
        })
        | Some(Command::Runall {
            no_post,
            dry_run,
            allow_error_status,
            ..
        }) => RunOptions {
            verbose: cli.verbose,
            no_post_script: *no_post,
            dry_run: *dry_run,
            allow_error_status: *allow_error_status,
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
    let env = build_runtime_env(cli)?;
    return run_one_with_env(path, cli, opts, &env).await;
}

/// Same as [`run_one`] but uses an existing [`RuntimeEnv`] (no `build_runtime_env`).
pub async fn run_one_with_env(
    path: &Path,
    cli: &Cli,
    opts: RunOptions,
    env: &RuntimeEnv,
) -> Result<(), String> {
    if opts.verbose && !opts.dry_run {
        println!("--- request ---");
    }

    if opts.dry_run || opts.verbose {
        let (prep, _) = prepare_request_with_env(path, env).map_err(|e| e.to_string())?;
        let summary = format_prepared_request(&prep).map_err(|e| e.to_string())?;
        println!("{summary}");

        if opts.dry_run {
            return Ok(());
        }
    }

    if cli.verbose {
        println!("--- response ---");
    }

    let output = execute_request_with_env(path, &opts, env)
        .await
        .map_err(|e| e.to_string())?;

    print_result(&output, cli.verbose)?;

    if cli.verbose {
        println!("--- post-script ---");
    }

    execute_request_post_script(&output, &opts, env, None).map_err(|e| e.to_string())?;

    return Ok(());
}

/// Run a sequence file: shared [`RuntimeEnv`], [`OutcomePolicy::SequenceStep`] per step.
pub async fn run_sequence(path: &Path, cli: &Cli, opts: RunOptions) -> Result<(), String> {
    let env = build_runtime_env(cli)?;
    return run_sequence_with_env(path, cli, opts, &env).await;
}

/// Same as [`run_sequence`] but uses an existing [`RuntimeEnv`] (no `build_runtime_env`).
pub async fn run_sequence_with_env(
    path: &Path,
    cli: &Cli,
    opts: RunOptions,
    env: &RuntimeEnv,
) -> Result<(), String> {
    if opts.dry_run {
        run_dry_sequence(path, env)?;
        return Ok(());
    }

    let (seq, base_dir) = load_sequence_file(path).map_err(|e| e.to_string())?;

    if seq.steps.is_empty() {
        return Err("sequence must contain at least one step".to_string());
    }

    let expanded_initial_vars =
        expand_hashmap_values(env, &seq.initial_variables).map_err(|e| e.to_string())?;

    env.merge_runtime_map(&expanded_initial_vars);

    let mut step_opts = opts.clone();
    step_opts.outcome_policy = OutcomePolicy::SequenceStep;

    for step in seq.steps.iter() {
        let step_path = base_dir.join(&step.file);

        if !step_path.is_file() {
            return Err(format!(
                "sequence step request file not found: {}",
                step_path.display()
            ));
        }

        if opts.verbose {
            println!("--- request: [{}] ---", step.file);
        }

        if opts.verbose {
            let (prep, _) = prepare_request_with_env(&step_path, env).map_err(|e| e.to_string())?;
            let summary = format_prepared_request(&prep).map_err(|e| e.to_string())?;
            println!("{summary}");
        }

        if cli.verbose {
            println!("--- response: [{}] ---", step.file);
        }

        let output = execute_request_with_env(&step_path, &step_opts, env)
            .await
            .map_err(|e| e.to_string())?;

        if cli.verbose {
            print_result(&output, cli.verbose)?;
            println!("--- post-script: [{}] ---", step.file);
        } else {
            let status_s = output.status.to_string();
            let status_colored = match output.status / 100 {
                1 => status_s.cyan(),
                2 => status_s.green(),
                3 => status_s.blue(),
                4 => status_s.yellow(),
                5 => status_s.red(),
                _ => status_s.normal(),
            };

            println!(
                "[{}・{}] {} {} ",
                &output.method.as_str().blue(),
                status_colored,
                &output.final_url,
                format!("{}ms", &output.duration.as_millis().to_string()).yellow()
            )
        }

        execute_request_post_script(&output, &step_opts, env, Some((step, base_dir.as_path())))
            .map_err(|e| e.to_string())?;
    }

    return Ok(());
}

fn run_dry_sequence(path: &Path, env: &RuntimeEnv) -> Result<(), String> {
    let (seq, base_dir) = load_sequence_file(path).map_err(|e| e.to_string())?;

    if seq.steps.is_empty() {
        return Err("sequence must contain at least one step".to_string());
    }

    let expanded_initial_vars =
        expand_hashmap_values(env, &seq.initial_variables).map_err(|e| e.to_string())?;
    env.merge_runtime_map(&expanded_initial_vars);

    if let Some(n) = &seq.name {
        println!("Running dry sequence (No network I/O): {n}\n");
    }

    for step in seq.steps.iter() {
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
            .map(|s| format!("[{}]", s))
            .unwrap_or_default();

        let (prep, _) = prepare_request_with_env(&step_path, env).map_err(|e| e.to_string())?;

        let summary = format_prepared_request(&prep).map_err(|e| e.to_string())?;

        println!(
            "--- {} ({}) ---\n{}",
            step_label,
            step_path.display(),
            summary
        );
    }
    return Ok(());
}
