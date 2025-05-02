use crate::utils::{get_current_project_config_path, load_config};
use colored::*;

pub fn ls() -> Result<(), String> {
    let project_path = get_current_project_config_path()?;
    let config = load_config(&project_path)?;

    let requests = config.get_requests(&project_path.parent().unwrap())?;

    if requests.is_empty() {
        println!("{}", "No requests found.".yellow());
        return Ok(());
    }

    // Print each request
    for (name, request) in requests {
        println!(
            "{} {} {}",
            name,
            format!("[{:?}]", request.method).yellow(),
            request.url.cyan(),
        );
    }

    Ok(())
}
