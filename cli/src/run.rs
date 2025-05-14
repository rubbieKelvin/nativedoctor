use std::{env::current_dir, fs, path::PathBuf, process::exit};

use rustle_api::executor::runner::{Runner, ScriptEngine};
use tracing::error;

use crate::ds::RunMode;

async fn run_request(runner: &mut Runner, name: String) {
    let client = reqwest::Client::new();
    let stack = match runner.generate_call_queue(&name) {
        Ok(s) => s,
        Err(e) => {
            error!("Error generating call stack: {}", e);
            exit(1);
        }
    };

    for request in stack {
        let result = match runner.call_request(request, &client).await {
            Ok(result) => result,
            Err(e) => {
                error!("{e}");
                exit(-1);
            }
        };

        // TODO: display result
        println!("{:?}", result);
    }
}

fn get_validated_path_str(filepath: &str) -> Result<String, String> {
    let path = PathBuf::from(filepath);

    if path.is_dir() {
        let project_file_path = path.join("project.rt.yaml");
        if project_file_path.exists() && project_file_path.is_file() {
            project_file_path
                .to_str()
                .map(|s| s.to_string()) // Convert &str to String
                .ok_or_else(|| {
                    format!(
                        "Error: Project file path is not valid UTF-8: {}",
                        project_file_path.display()
                    )
                })
        } else {
            Err(format!(
                "Error: Project file not found at: {}",
                project_file_path.display()
            ))
        }
    } else if path.is_file() {
        path.to_str()
            .map(|s| s.to_string()) // Convert &str to String
            .ok_or_else(|| "Error: File path is not valid UTF-8".to_string())
    } else {
        Err("Error: Invalid file path".to_string())
    }
}

async fn run_sequence(runner: &mut Runner, name: String) {
    let client = reqwest::Client::new();
    let stack = match runner.generate_sequence_queue(&name) {
        Ok(s) => s,
        Err(e) => {
            error!("Error generating sequence stack: {}", e);
            exit(1);
        }
    };

    for request in stack {
        let result = match runner.call_request(request, &client).await {
            Ok(result) => result,
            Err(e) => {
                error!("Error calling request: {}", e);
                exit(-1);
            }
        };

        // TODO: display result
        println!("{:?}", result);
    }
}

pub async fn run(filepath: &PathBuf, env: Option<String>, mode: RunMode) {
    let cwd = match current_dir() {
        Ok(dir) => dir,
        Err(e) => {
            error!("Cannot get the current dir: {}", e);
            exit(1);
        }
    };

    // resolve path
    let filepath = filepath.to_str().unwrap();
    let path = cwd.join(filepath);
    let path = match fs::canonicalize(path) {
        Ok(p) => p,
        Err(e) => {
            error!("Cannot get absolute path of {}\nError: {}", filepath, e);
            exit(1);
        }
    };

    // resolve project or file path
    let is_project = path.is_dir();
    let path = match get_validated_path_str(path.to_str().unwrap()) {
        Ok(p) => p,
        Err(err) => {
            eprintln!("{}", err);
            exit(1); // Or handle the error differently
        }
    };

    // initiate the runner
    let mut runner = match Runner::new(&path, env, ScriptEngine::None, is_project) {
        Ok(runner) => runner,
        Err(e) => {
            eprintln!("Error creating runner: {}", e.to_string());
            exit(1);
        }
    };

    match mode {
        RunMode::Request(name) => {
            run_request(&mut runner, name).await;
        }
        RunMode::Sequence(name) => {
            run_sequence(&mut runner, name).await;
        }
        RunMode::All => {
            let seq = &runner
                .schema
                .calls
                .keys()
                .map(|k| k.to_string())
                .collect::<Vec<String>>();

            for i in seq {
                run_sequence(&mut runner, i.clone()).await;
            }
        }
    };
}
