use std::{env::current_dir, fs, path::PathBuf, process::exit};

use tracing::error;

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

pub fn resolve_path(filepath: &PathBuf) -> (String, bool) {
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
    return (
        match get_validated_path_str(path.to_str().unwrap()) {
            Ok(p) => p,
            Err(err) => {
                eprintln!("{}", err);
                exit(1); // Or handle the error differently
            }
        },
        path.is_dir(),
    );
}

pub fn quiet_or_print(string: String, quiet: bool) {
    if !quiet {
        println!("{string}");
    }
}
