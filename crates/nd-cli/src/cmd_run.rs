//! `nativedoctor run` (and default file path): single request execution and `sequence` runs.

use std::path::Path;

use nd_core::{
    execute_request_file, execute_sequence, format_prepared_request, load_request_file,
    load_sequence_file, prepare_request_file, prepare_request_with_env, ExecutionResult,
    OutcomePolicy, RunOptions, RuntimeEnv,
};

use crate::Cli;

pub fn run_opts(cli: &Cli) -> RunOptions {
    RunOptions {
        verbose: cli.verbose,
        no_post_script: cli.no_post_script,
        dry_run: cli.dry_run,
        allow_error_status: cli.allow_error_status,
        outcome_policy: OutcomePolicy::SingleRequest,
    }
}

/// Run or dry-run a single request file; prints human-readable output to stdout/stderr.
pub async fn run_one(
    path: &Path,
    cli: &Cli,
    opts: RunOptions,
) -> Result<(), String> {
    if opts.dry_run {
        let (prep, _) = prepare_request_file(path).map_err(|e| e.to_string())?;
        let summary = format_prepared_request(&prep).map_err(|e| e.to_string())?;
        print!("{summary}");
        return Ok(());
    }

    if cli.verbose {
        // Second load: keeps `nd-core` free of `eprintln!` while still showing the resolved request.
        let (prep, _) = prepare_request_file(path).map_err(|e| e.to_string())?;
        let summary = format_prepared_request(&prep).map_err(|e| e.to_string())?;
        eprint!("--- request ---\n{summary}\n--- response ---\n");
    }

    let result = execute_request_file(path, opts)
        .await
        .map_err(|e| e.to_string())?;

    print_result(&result, cli.verbose)?;

    Ok(())
}

/// Run a sequence file: shared [`RuntimeEnv`], [`OutcomePolicy::SequenceStep`] per step.
pub async fn run_sequence(
    path: &Path,
    cli: &Cli,
    opts: RunOptions,
) -> Result<(), String> {
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
    Ok(())
}

fn redact_headers(headers: &[(String, String)]) -> Vec<(String, String)> {
    headers
        .iter()
        .map(|(k, v)| {
            if k.eq_ignore_ascii_case("authorization") {
                (k.clone(), "<redacted>".to_string())
            } else {
                (k.clone(), v.clone())
            }
        })
        .collect()
}

/// Status line always; verbose adds headers; body is pretty-printed JSON when valid UTF-8 JSON.
fn print_result(result: &ExecutionResult, verbose: bool) -> Result<(), String> {
    let label = result
        .request_name
        .as_deref()
        .map(|s| format!(" [{}]", s))
        .unwrap_or_default();
    println!(
        "{}{} {} -> {} ({:?})",
        result.method, label, result.final_url, result.status, result.duration
    );
    if verbose {
        let hdrs = redact_headers(&result.headers);
        for (k, v) in hdrs {
            println!("{k}: {v}");
        }
        println!();
    }
    let body = &result.body;
    if body.is_empty() {
        return Ok(());
    }
    if let Ok(text) = std::str::from_utf8(body) {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(text) {
            println!(
                "{}",
                serde_json::to_string_pretty(&v).unwrap_or_else(|_| text.to_string())
            );
        } else {
            print!("{text}");
            if !text.ends_with('\n') {
                println!();
            }
        }
    } else {
        println!("<{} bytes binary>", body.len());
    }
    Ok(())
}
