//! CLI entry for **nativedoctor**: `run` / shorthand file path, `list`, `sequence`, `new`, and shared flags.

mod cmd_generate;
mod cmd_new;
mod logging;

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use nd_core::{
    execute_request_file, execute_sequence, format_prepared_request, list_request_paths,
    load_request_file, load_sequence_file, prepare_request_file, prepare_request_with_env,
    ExecutionResult, OutcomePolicy, RunOptions, RuntimeEnv,
};

#[derive(Parser)]
#[command(name = "nativedoctor")]
#[command(about = "File-based API request runner (JSON/YAML) with optional Rhai post-scripts.")]
#[command(args_conflicts_with_subcommands = false)]
struct Cli {
    /// Log extra detail (full request before send, response headers on stdout) and enable
    /// `nd_core=debug` tracing unless `RUST_LOG` is set.
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Do not run `post_script` from the request file.
    #[arg(long, global = true)]
    no_post_script: bool,

    /// Expand and print the request only; no network I/O.
    #[arg(long, global = true)]
    dry_run: bool,

    /// Treat HTTP 4xx/5xx as success for exit status (post-script still runs first).
    #[arg(long, global = true)]
    allow_error_status: bool,

    #[command(subcommand)]
    command: Option<Command>,

    /// If no subcommand is given, this file is executed (same as `run <FILE>`).
    #[arg(value_name = "FILE", value_hint = clap::ValueHint::FilePath)]
    file: Option<PathBuf>,
}

#[derive(Clone, Copy, Debug, Default, ValueEnum)]
enum GenerateFormat {
    #[default]
    Yaml,
    Json,
}

impl From<GenerateFormat> for ng_generate::OutputFormat {
    fn from(f: GenerateFormat) -> Self {
        match f {
            GenerateFormat::Yaml => Self::Yaml,
            GenerateFormat::Json => Self::Json,
        }
    }
}

#[derive(Subcommand)]
enum Command {
    /// Run one request definition.
    Run { path: PathBuf },
    /// Run an ordered list of request files with one shared runtime environment (see sequence JSON/YAML).
    Sequence { path: PathBuf },
    /// List `*.json` / `*.yaml` / `*.yml` in a directory (immediate children only, sorted).
    List { dir: PathBuf },
    /// Generate nativedoctor request files from an OpenAPI 3.0.x document (JSON or YAML).
    Generate {
        /// OpenAPI spec file (.json, .yaml, or .yml).
        #[arg(short = 'i', long = "input", value_name = "FILE")]
        input: PathBuf,
        /// Output directory (created if missing).
        #[arg(short = 'o', long = "output", value_name = "DIR")]
        output: PathBuf,
        /// Request file format for generated files.
        #[arg(long, value_enum, default_value_t = GenerateFormat::Yaml)]
        format: GenerateFormat,
    },
    /// Create a starter request or sequence file (e.g. `new --sequence seq-a.json`).
    New {
        /// Write a default sequence file (.json, .yaml, or .yml).
        #[arg(long, short = 's', value_name = "PATH", conflicts_with = "request")]
        sequence: Option<PathBuf>,
        /// Write a default request file (.json, .yaml, or .yml).
        #[arg(long, short = 'r', value_name = "PATH", conflicts_with = "sequence")]
        request: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();
    logging::init(cli.verbose);
    match run(cli).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{e}");
            ExitCode::FAILURE
        }
    }
}

fn run_opts(cli: &Cli) -> RunOptions {
    RunOptions {
        verbose: cli.verbose,
        no_post_script: cli.no_post_script,
        dry_run: cli.dry_run,
        allow_error_status: cli.allow_error_status,
        outcome_policy: OutcomePolicy::SingleRequest,
    }
}

async fn run(cli: Cli) -> std::result::Result<(), String> {
    match &cli.command {
        Some(Command::Generate {
            input,
            output,
            format,
        }) => {
            cmd_generate::run_generate(input, output, (*format).into())?;
        }
        Some(Command::New { sequence, request }) => {
            cmd_new::run_new(sequence.as_ref(), request.as_ref())?;
        }
        Some(Command::List { dir }) => {
            let paths = list_request_paths(dir).map_err(|e| e.to_string())?;
            if paths.is_empty() {
                println!("(no request files found)");
            } else {
                for p in paths {
                    println!("{}", p.display());
                }
            }
        }
        Some(Command::Run { path }) => {
            run_one(path, &cli, run_opts(&cli)).await?;
        }
        Some(Command::Sequence { path }) => {
            run_sequence(path, &cli, run_opts(&cli)).await?;
        }
        None => {
            let path = cli
                .file
                .as_ref()
                .ok_or_else(|| "expected a subcommand or a request file path".to_string())?;
            run_one(path, &cli, run_opts(&cli)).await?;
        }
    }
    Ok(())
}

/// Run or dry-run a single request file; prints human-readable output to stdout/stderr.
async fn run_one(
    path: &std::path::Path,
    cli: &Cli,
    opts: RunOptions,
) -> std::result::Result<(), String> {
    if opts.dry_run {
        // let (doc, _) = load_request_file(path).map_err(|e| e.to_string())?;

        // if let Some(n) = &doc.name {
        //     println!("# {}\n", n);
        // }

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
async fn run_sequence(
    path: &std::path::Path,
    cli: &Cli,
    opts: RunOptions,
) -> std::result::Result<(), String> {
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
fn print_result(result: &ExecutionResult, verbose: bool) -> std::result::Result<(), String> {
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
