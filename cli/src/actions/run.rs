use std::{path::PathBuf, process::exit};

use rustle_api::executor::runner::{Runner, ScriptEngine};
use tracing::error;

use super::utils::resolve_path;
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

async fn run_sequence(runner: &mut Runner, name: String) {
    let client = reqwest::Client::new();
    let stack = match runner.generate_sequence_queue(&name) {
        Ok(s) => s,
        Err(e) => {
            error!("Error generating sequence stack: {}", e);
            exit(1);
        }
    };

    for request_seq in stack {
        for request in request_seq {
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
}

pub async fn run(filepath: &PathBuf, env: Option<String>, mode: RunMode) {
    let (path, is_project) = resolve_path(filepath);

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

            match seq.len() {
                0 => {
                    eprintln!("No defined sequence in the list");
                    exit(1);
                }
                1 => {
                    run_sequence(&mut runner, seq.get(0).unwrap().clone()).await;
                }
                _ => {
                    eprintln!("Specify sequence name if there's more than one in the schema");
                    exit(1);
                }
            }
        }
    };
}
