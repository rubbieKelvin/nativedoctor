use crate::utils::{get_current_project_config_path, load_config};
use colored::*;
use reqwest::Response;
use std::io;

pub async fn call(name: &str) -> Result<(), String> {
    let project_path = get_current_project_config_path()?;
    let config = load_config(&project_path)?;

    let requests = config.get_requests(&project_path.parent().unwrap())?;
    match requests.get(name) {
        Some(request_schema) => {
            let client = reqwest::Client::new();
            let request = request_schema.to_reqwest(&client).await?;
            return match client.execute(request).await {
                Ok(response) => print_response(response).await.map_err(|e| e.to_string()),
                Err(err) => {
                    eprintln!("{} {}", "Error:".red(), err.to_string().white());
                    return Err(err.to_string());
                }
            };
        }
        None => {
            return Err("Could not get request with such name".to_string());
        }
    }
}

async fn print_response(response: Response) -> io::Result<()> {
    // Store status and headers before consuming the response
    let status = response.status();
    let headers = response.headers().clone();
    let content_type = headers.get("content-type").cloned();

    // print status line
    let status_line = format!(
        "{} {} {}",
        "HTTP/1.1".bright_blue(),
        status.as_u16().to_string().yellow(),
        status.canonical_reason().unwrap_or("Unknown")
    );
    println!("{}\n", status_line);

    // print response headers
    println!("{}", "Response Headers:".bright_blue());
    for (name, value) in headers.iter() {
        println!(
            "{}: {}",
            name.to_string().green(),
            value.to_str().unwrap_or("").white()
        );
    }
    println!();

    // print response body
    println!("{}", "Response Body:".bright_blue());
    let body = response.text().await.map_err(|e| {
        io::Error::new(
            io::ErrorKind::Other,
            format!("Failed to read response body: {}", e),
        )
    })?;

    // try to pretty print json if the content type is json
    if let Some(content_type) = content_type {
        if content_type
            .to_str()
            .unwrap_or("")
            .contains("application/json")
        {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                println!("{}", serde_json::to_string_pretty(&json).unwrap().white());
                return Ok(());
            }
        }
    }

    // if not json or json parsing failed, print raw body
    println!("{}", body.white());
    Ok(())
}
