use std::{path::PathBuf, process::exit};

use rustle_api::executor::runner::{Runner, ScriptEngine};

use crate::ds::RunMode;

async fn run_request(runner: Runner, name: String) {
    let client = reqwest::Client::new();
    let stack = runner.generate_call_queue(&name).unwrap();

    for request in stack {
        let result = match runner.call_request(request, &client).await {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error calling request: {}", e.to_string());
                exit(-1);
            }
        };

        // TODO: display result
        println!("{:?}", result);
    }
}

pub async fn run(filepath: &PathBuf, env: Option<String>, mode: RunMode) {
    let path_str = match filepath.to_str() {
        Some(p) => p,
        None => {
            eprintln!("Error reading file path");
            exit(1);
        }
    };

    let runner = match Runner::new(path_str, env, ScriptEngine::None) {
        Ok(runner) => runner,
        Err(e) => {
            eprintln!("Error creating runner: {}", e.to_string());
            exit(1);
        }
    };

    match mode {
        RunMode::Request(name) => {
            run_request(runner, name).await;
        } // ...
          // RunMode::Sequence(name) => {}
          // RunMode::All => {}
    };
}
