use std::fs;
use std::io::{self, IsTerminal, Read, Write};
use std::path::{Path, PathBuf};

use colored::Colorize;
use nd_constants::REQUEST_SUFFIX;
use nd_core::check::check_collection;
use nd_core::collection::discover;
use nd_core::items::{self, BodyMode, Item};
use nd_core::{
    prepare, state, Body, Collection, HttpSpec, Part, PrepareContext, PreparedBody,
    PreparedRequest, RequestFile, Variables,
};

use crate::args::{Options, Pretty, RunArgs, SendArgs};
use crate::exchange::{self, Output};
use crate::failure::Failure;
use crate::render::Print;

pub fn init(dir: &Path, name: Option<&str>) -> Result<(), Failure> {
    let collection = Collection::init(dir, name)?;
    println!(
        "Created collection `{}` in {}",
        collection.manifest.name,
        collection.root.display()
    );
    println!(
        "Try: nativedoctor run {}",
        dir.join(format!("ping{REQUEST_SUFFIX}")).display()
    );
    return Ok(());
}

pub fn list(path: &Path) -> Result<(), Failure> {
    let collection = require_collection(path)?;
    let mut invalid = 0;
    for file in collection.requests()? {
        let relative = file
            .strip_prefix(&collection.root)
            .unwrap_or(&file)
            .display();
        match RequestFile::load(&file) {
            Ok(request) => {
                let method = request.http.method.clone().unwrap_or_else(|| {
                    let default = if request.http.body.is_some() {
                        "POST"
                    } else {
                        "GET"
                    };
                    default.to_string()
                });
                println!("{method:<7} {relative}  {}", request.display_name(&file));
            }
            Err(error) => {
                invalid += 1;
                eprintln!("error: {error}");
            }
        }
    }
    if invalid > 0 {
        return Err(Failure::new(format!("{invalid} invalid request file(s)")));
    }
    return Ok(());
}

pub fn check(
    path: &Path,
    env: Option<&str>,
    overrides: &[(String, String)],
) -> Result<(), Failure> {
    let collection = require_collection(path)?;
    let env = env
        .map(str::to_string)
        .or_else(|| collection.manifest.default_env.clone());
    let vars = load_vars(Some(&collection), env.as_deref(), overrides)?;
    let report = check_collection(&collection, &vars)?;

    for warning in &report.warnings {
        eprintln!("warning: {warning}");
    }
    for error in &report.errors {
        eprintln!("error: {error}");
    }
    if !report.errors.is_empty() {
        return Err(Failure::new(format!(
            "{} problem(s) in {} request(s)",
            report.errors.len(),
            report.checked
        )));
    }
    println!("{} request(s) OK", report.checked);
    return Ok(());
}

pub async fn run(args: RunArgs) -> Result<(), Failure> {
    let is_dir = args.target.is_dir();
    if !is_dir && !args.target.is_file() {
        return Err(Failure::new(format!("{} not found", args.target.display())));
    }
    let target = args
        .target
        .canonicalize()
        .map_err(|e| Failure::new(format!("{}: {e}", args.target.display())))?;
    let start = if is_dir {
        target.as_path()
    } else {
        parent_of(&target)
    };

    let collection = Collection::find(start)?;
    let env = select_env(collection.as_ref(), args.options.env.as_deref())?;
    let mut vars = load_vars(collection.as_ref(), env.as_deref(), &args.options.vars)?;

    let paths = match (is_dir, &collection) {
        (false, _) => vec![target.clone()],
        (true, Some(collection)) => collection.requests_under(&target)?,
        (true, None) => discover(&target)?,
    };
    if paths.is_empty() {
        return Err(Failure::new(format!(
            "no *{REQUEST_SUFFIX} files in {}",
            target.display()
        )));
    }
    if is_dir && (!args.items.is_empty() || args.options.body.is_some()) {
        return Err(Failure::new(
            "request items and --body only work when running a single file",
        ));
    }

    let parsed = parse_items(&args.items)?;
    let mut output = output_for(&args.options)?;
    output.show_titles = paths.len() > 1 && output.pretty && !output.json;
    colored::control::set_override(output.pretty);

    for path in &paths {
        let outcome = run_file(
            path,
            collection.as_ref(),
            env.as_deref(),
            &parsed,
            &args.options,
            &output,
            &mut vars,
        )
        .await;
        outcome.map_err(|failure| failure.redacted(&vars.redactor()))?;
    }
    return Ok(());
}

async fn run_file(
    path: &Path,
    collection: Option<&Collection>,
    env: Option<&str>,
    parsed: &[Item],
    options: &Options,
    output: &Output,
    vars: &mut Variables,
) -> Result<(), Failure> {
    let mut file = RequestFile::load(path)?;
    let name = file.display_name(path);
    if output.show_titles {
        let mut out = io::stdout().lock();
        writeln!(out, "{}", format!("── {name} ──").bold())?;
    }

    let base_dir = parent_of(path).to_path_buf();
    let root = collection.map_or_else(|| base_dir.clone(), |c| c.root.clone());
    let scope = Scope {
        base_dir,
        root: Some(root),
        collection,
        save: None,
    };
    let saved = execute(&name, &mut file, parsed, options, output, vars, &scope).await?;

    if let (Some(collection), false) = (collection, saved.is_empty()) {
        state::save(&collection.state_path(env), &saved)?;
    }
    return Ok(());
}

pub async fn send(args: SendArgs) -> Result<(), Failure> {
    let (method, url, raw_items) = split_send_args(&args.args)?;
    let cwd = std::env::current_dir().map_err(|e| Failure::new(e.to_string()))?;
    let collection = Collection::find(&cwd)?;
    let env = select_env(collection.as_ref(), args.options.env.as_deref())?;
    let mut vars = load_vars(collection.as_ref(), env.as_deref(), &args.options.vars)?;

    let parsed = parse_items(raw_items)?;
    let output = output_for(&args.options)?;
    colored::control::set_override(output.pretty);

    let name = args
        .save
        .as_deref()
        .map_or_else(|| "request".to_string(), request_name_from_path);
    let mut file = RequestFile {
        name: args.save.as_ref().map(|_| name.clone()),
        http: HttpSpec {
            method,
            url,
            ..HttpSpec::default()
        },
        ..RequestFile::default()
    };

    let scope = Scope {
        base_dir: cwd,
        root: None,
        collection: collection.as_ref(),
        save: args.save.as_deref(),
    };
    let outcome = execute(
        &name,
        &mut file,
        &parsed,
        &args.options,
        &output,
        &mut vars,
        &scope,
    )
    .await;
    outcome.map_err(|failure| failure.redacted(&vars.redactor()))?;
    return Ok(());
}

struct Scope<'a> {
    base_dir: PathBuf,
    /// When set, body files in the request must be inside it.
    root: Option<PathBuf>,
    collection: Option<&'a Collection>,
    save: Option<&'a Path>,
}

/// Applies items and flags, prepares the request, then saves, prints or sends it.
async fn execute(
    name: &str,
    file: &mut RequestFile,
    parsed: &[Item],
    options: &Options,
    output: &Output,
    vars: &mut Variables,
    scope: &Scope<'_>,
) -> Result<Vec<(String, String)>, Failure> {
    let mode = if options.multipart {
        BodyMode::Multipart
    } else if options.form {
        BodyMode::Form
    } else {
        BodyMode::Auto
    };
    let mut allowed_files = items::apply(&mut file.http, parsed, mode)?;

    let has_data = parsed.iter().any(Item::is_data);
    let stdin_body = apply_body_flag(&mut file.http, options, has_data, &mut allowed_files)?;
    apply_flags(&mut file.http, options);

    if let Some(path) = scope.save {
        if stdin_body.is_some() {
            return Err(Failure::new("--save can't store a body read from stdin"));
        }
        save_request(path, file)?;
    }

    let ctx = PrepareContext {
        vars,
        default_headers: scope.collection.map(|c| &c.manifest.defaults.headers),
        base_dir: scope.base_dir.clone(),
        root: scope.root.clone(),
        allowed_files: &allowed_files,
    };
    let mut request = prepare(&file.http, &ctx)?;
    if let Some(bytes) = stdin_body {
        use_stdin_body(&mut request, &file.http, bytes);
    }

    if options.offline {
        exchange::print_offline(name, &request, vars, output)?;
        return Ok(Vec::new());
    }
    return exchange::perform(name, file, request, vars, output).await;
}

/// Handles --body. Returns stdin's bytes, which skip variable expansion.
fn apply_body_flag(
    spec: &mut HttpSpec,
    options: &Options,
    has_data: bool,
    allowed_files: &mut Vec<PathBuf>,
) -> Result<Option<Vec<u8>>, Failure> {
    let Some(raw) = &options.body else {
        return Ok(None);
    };
    if has_data {
        return Err(Failure::new("--body can't be combined with data items"));
    }
    if raw == "-" {
        let mut bytes = Vec::new();
        io::stdin().read_to_end(&mut bytes)?;
        spec.body = None;
        return Ok(Some(bytes));
    }
    if let Some(path) = raw.strip_prefix('@') {
        let canonical = Path::new(path)
            .canonicalize()
            .map_err(|e| Failure::new(format!("{path}: {e}")))?;
        spec.body = Some(Body::File(canonical.display().to_string()));
        allowed_files.push(canonical);
        return Ok(None);
    }
    spec.body = Some(Body::Text(raw.clone()));
    return Ok(None);
}

fn use_stdin_body(request: &mut PreparedRequest, spec: &HttpSpec, bytes: Vec<u8>) {
    if spec.method.is_none() {
        request.method = "POST".to_string();
    }
    let is_content_type = |name: &String| name.eq_ignore_ascii_case("content-type");
    let names_type = spec.headers.0.iter().any(|(name, _)| is_content_type(name))
        || request
            .headers
            .iter()
            .any(|(name, _)| is_content_type(name));
    if !names_type && serde_json::from_slice::<serde_json::Value>(&bytes).is_ok() {
        request
            .headers
            .push(("content-type".to_string(), "application/json".to_string()));
    }
    request.body = PreparedBody::Bytes(bytes);
}

fn apply_flags(spec: &mut HttpSpec, options: &Options) {
    if options.follow {
        spec.follow_redirects = Some(true);
    }
    if options.max_redirects.is_some() {
        spec.max_redirects = options.max_redirects;
    }
    if options.timeout.is_some() {
        spec.timeout = options.timeout.clone();
    }
    if options.insecure {
        spec.verify_tls = Some(false);
    }
}

/// `[METHOD] URL ITEM...`. Only an all-caps first word followed by more args is a method.
fn split_send_args(args: &[String]) -> Result<(Option<String>, String, &[String]), Failure> {
    let first = args
        .first()
        .ok_or_else(|| Failure::new("a URL is required"))?;
    let is_method =
        args.len() > 1 && !first.is_empty() && first.chars().all(|c| c.is_ascii_uppercase());
    if is_method {
        return Ok((Some(first.clone()), args[1].clone(), &args[2..]));
    }
    return Ok((None, first.clone(), &args[1..]));
}

fn parse_items(raw: &[String]) -> Result<Vec<Item>, Failure> {
    let mut parsed = Vec::with_capacity(raw.len());
    for item in raw {
        parsed.push(items::parse(item)?);
    }
    return Ok(parsed);
}

fn output_for(options: &Options) -> Result<Output, Failure> {
    let pretty = match options.pretty {
        Some(choice) => choice == Pretty::All,
        None => io::stdout().is_terminal() && !options.json,
    };
    let print = if let Some(letters) = &options.print {
        Print::parse(letters)?
    } else if options.verbose {
        Print::parse("HBhbm")?
    } else if options.offline {
        Print::parse("HB")?
    } else if pretty {
        Print::parse("hb")?
    } else {
        Print::parse("b")?
    };
    return Ok(Output {
        print,
        pretty,
        json: options.json,
        stream: options.stream,
        fail: options.fail,
        show_titles: false,
    });
}

fn select_env(
    collection: Option<&Collection>,
    requested: Option<&str>,
) -> Result<Option<String>, Failure> {
    let Some(collection) = collection else {
        if requested.is_some() {
            return Err(Failure::new("--env needs a collection, and none was found"));
        }
        return Ok(None);
    };
    return Ok(requested
        .map(str::to_string)
        .or_else(|| collection.manifest.default_env.clone()));
}

fn load_vars(
    collection: Option<&Collection>,
    env: Option<&str>,
    overrides: &[(String, String)],
) -> Result<Variables, Failure> {
    let process_env = |name: &str| std::env::var(name).ok();
    return Ok(Variables::load(collection, env, overrides, &process_env)?);
}

fn require_collection(path: &Path) -> Result<Collection, Failure> {
    return Collection::find(path)?.ok_or_else(|| {
        Failure::new(format!(
            "no {} found in {} or its parents",
            nd_constants::MANIFEST_FILE,
            path.display()
        ))
    });
}

fn parent_of(path: &Path) -> &Path {
    path.parent().unwrap_or(Path::new("."))
}

fn request_name_from_path(path: &Path) -> String {
    let file = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("request");
    return file.trim_end_matches(REQUEST_SUFFIX).to_string();
}

/// Writes a new request file. Refuses to overwrite, and refuses bodies that reference local files.
fn save_request(path: &Path, file: &RequestFile) -> Result<(), Failure> {
    if !path.to_string_lossy().ends_with(REQUEST_SUFFIX) {
        return Err(Failure::new(format!(
            "--save needs a *{REQUEST_SUFFIX} path"
        )));
    }
    let uses_files = match &file.http.body {
        Some(Body::File(_)) => true,
        Some(Body::Multipart(parts)) => {
            parts.values().any(|part| matches!(part, Part::File { .. }))
        }
        _ => false,
    };
    if uses_files {
        return Err(Failure::new(
            "--save doesn't support file bodies yet; add the path to the request file by hand",
        ));
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| Failure::new(format!("{}: {e}", parent.display())))?;
    }
    let text = serde_yaml_ng::to_string(file).map_err(|e| Failure::new(e.to_string()))?;
    let mut out = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map_err(|e| Failure::new(format!("{}: {e}", path.display())))?;
    out.write_all(text.as_bytes())?;
    return Ok(());
}
