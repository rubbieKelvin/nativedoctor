use std::path::Path;

use crate::{
    schema::request::Request,
    utils::{get_current_project_config_path, load_config},
};

pub fn add(
    name: String,
    url: String,
    method: String,
    params: Vec<String>,
    headers: Vec<String>,
    body: Option<String>,
    form: Vec<String>,
    files: Vec<String>,
    auth: Option<String>,
    bearer: Option<String>,
    api_key: Option<String>,
    api_key_header: Option<String>,
) -> Result<(), String> {
    let config_path = get_current_project_config_path()?;
    let mut config = load_config(&config_path)?;
    let request = Request::parse_from_args(
        name,
        url,
        method,
        params,
        headers,
        body,
        form,
        files,
        auth,
        bearer,
        api_key,
        api_key_header,
    );

    config.add_request(request, Path::new(&config_path).parent().unwrap())?;
    println!("Request saved successfully!");
    Ok(())
}
