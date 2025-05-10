use std::{path::PathBuf, process::exit};

use librustle::parser::runner::Runner;

use crate::ds::RunMode;

async fn run_request(runner: Runner, name: String) {
    let client = reqwest::Client::new();
    let result = match runner.call_request(name, &client, None).await {
        Ok(result) => result,
        Err(e) => {
            eprintln!("Error calling request: {}", e.to_string());
            exit(-1);
        }
    };

    // display result
}

pub async fn run(filepath: &PathBuf, env: Option<String>, mode: RunMode) {
    let path_str = match filepath.to_str() {
        Some(p) => p,
        None => {
            eprintln!("Error reading file path");
            exit(1);
        }
    };

    let runner = match Runner::new(path_str, env) {
        Ok(runner) => runner,
        Err(e) => {
            eprintln!("Error creating runner: {}", e.to_string());
            exit(1);
        }
    };

    match mode {
        RunMode::Request(name) => {
            run_request(runner, name).await;
        }
        RunMode::Sequence(name) => {}
        RunMode::All => {}
    };
}
