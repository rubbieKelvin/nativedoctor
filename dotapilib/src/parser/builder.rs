use super::{
    utils::{interpolate_string, interpolate_value, STRICT_INTERPOLATION},
    yaml::{load_api_file, MultipartPart, Request, RequestBody, Schema},
};
use anyhow::{Context, Result};
use async_recursion::async_recursion;
use std::{collections::HashMap, env::current_dir, path::Path};

pub struct Runtime {
    pub schema: Schema,
    filename: String,
    environment: Option<String>,
    /// Variables we override at runtime (Impossible for now)
    // NOTE: We might want to clear this per call sequence
    // Or maybe use a unique runtime for each, then run them in parallel
    overrides: HashMap<String, String>,
}

impl Runtime {
    pub fn new(filename: &str, environment: Option<String>) -> Result<Self> {
        // TODO: might need to open this from the cwd the program is runing
        let cwd = current_dir().context("Could not get the current working directorty")?;
        let path = cwd.join(filename);

        let schema = load_api_file(path.as_path())?;

        return Ok(Runtime {
            schema,
            filename: filename.to_string(),
            environment,
            overrides: HashMap::new(),
        });
    }

    /// This should resolve the env variables by the current environment, and return a clean represengtation of the env
    pub fn build_env(&self) -> HashMap<String, String> {
        let mut env_vars = HashMap::<String, String>::new();

        for (key, config) in self.schema.env.iter() {
            // let pick up the value based on the environment
            let resolved_value = match &self.environment {
                Some(env_name) => {
                    // Check if an override exists for the current environment
                    if let Some(override_value) = config.overrides.get(env_name) {
                        override_value
                    } else {
                        // No override for this environment, use the default
                        &config.default
                    }
                }
                None => &config.default,
            };

            env_vars.insert(key.clone(), resolved_value.to_string());
        }

        // TODO: Override with other variables from teh override props
        return env_vars;
    }

    #[async_recursion]
    pub async fn call(&self, name: String, parent: Option<String>) -> Result<Response> {
        let request = self.schema.requests.get(&name);
        let mut depenency_responses: HashMap<String, Response> = HashMap::new();

        return match request {
            Some(request) => {
                if let Some(config) = &request.config {
                    for dependency in config.depends_on.iter() {
                        // Check for circular dependencies (basic check)
                        // This would happend if the user as request A to depend on request B
                        // And rewuest B depends on request A.
                        if parent.as_ref() == Some(dependency) {
                            anyhow::bail!(
                                "Circular dependency detected: {} -> {}",
                                name,
                                dependency
                            );
                        }

                        // make a call on the dependency. and add it to the results
                        let dependecy_response =
                            self.call(dependency.clone(), Some(name.clone())).await?;
                        depenency_responses.insert(dependency.clone(), dependecy_response);
                    }
                }

                // now let's build the request
                let req = self.build_request(request).await?;

                Ok(Response { status: 200 })
            }
            None => panic!(),
        };
    }

    /// Builds a reqwest::Request from a Request schema and resolved environment variables.
    pub async fn build_request(&self, request_schema: &Request) -> Result<reqwest::Request> {
        // Build env everytime we're building a request
        // Because overrides might have been added by some other request and we want to catch that
        let env = &self.build_env();

        // Interpolate URL
        let interpolated_url_str =
            interpolate_string(&request_schema.url, env, STRICT_INTERPOLATION)?;

        let url = reqwest::Url::parse(&interpolated_url_str)
            .context(format!("Failed to parse URL: {}", interpolated_url_str))?;

        // Determine HTTP Method
        let method: reqwest::Method = request_schema
            .method
            .parse()
            .context(format!("Invalid HTTP method: {}", request_schema.method))?;

        // Start building the request
        let client = reqwest::Client::new();
        let mut builder = client.request(method, url);

        // Add Headers
        if let Some(headers) = &request_schema.headers {
            for (key, value) in headers.iter() {
                let interpolated_value = interpolate_string(value, env, STRICT_INTERPOLATION)?;
                builder = builder.header(key, interpolated_value);
            }
        }

        // Add Query Parameters
        if let Some(query) = &request_schema.query {
            let mut interpolated_query: HashMap<String, String> = HashMap::new();
            for (key, value) in query.iter() {
                // Interpolate the value, then convert the resulting EnvValue to a string
                let interpolated_str = interpolate_string(value, env, STRICT_INTERPOLATION)?;
                interpolated_query.insert(key.clone(), interpolated_str);
            }
            builder = builder.query(&interpolated_query);
        }

        // Add Body
        if let Some(body_config) = &request_schema.body {
            match body_config {
                RequestBody::Json { content } => {
                    // Interpolate the JSON content recursively
                    let interpolated_content = interpolate_value(content, env)?;
                    // reqwest::RequestBuilder::json handles serialization
                    builder = builder.json(&interpolated_content);
                }
                RequestBody::Graphql { query, variables } => {
                    // Interpolate query string (less common, but possible)
                    let interpolated_query_str =
                        interpolate_string(query, env, STRICT_INTERPOLATION)?;

                    // Interpolate and serialize variables if present
                    let interpolated_variables = if let Some(vars) = variables {
                        Some(interpolate_value(vars, env)?)
                    } else {
                        None
                    };

                    // GraphQL bodies are typically JSON with 'query' and 'variables' keys
                    let mut graphql_body_map = serde_json::json!({
                        "query": interpolated_query_str,
                    });
                    if let Some(vars) = interpolated_variables {
                        graphql_body_map["variables"] = serde_json::to_value(vars)?;
                    }

                    builder = builder.json(&graphql_body_map);
                }
                RequestBody::Xml { content }
                | RequestBody::Text { content }
                | RequestBody::FormUrlencoded { content } => {
                    // Interpolate the raw content string
                    let interpolated_content =
                        interpolate_string(content, env, STRICT_INTERPOLATION)?;
                    builder = builder.body(interpolated_content);
                }
                RequestBody::Multipart { parts } => {
                    // let mut form = multipart::Form::new();
                    let mut form = reqwest::multipart::Form::new();
                    for part in parts {
                        match part {
                            MultipartPart::Field { name, value } => {
                                let interpolated_value =
                                    interpolate_string(value, env, STRICT_INTERPOLATION)?;
                                form = form.text(name.clone(), interpolated_value);
                            }
                            MultipartPart::File {
                                name,
                                path,
                                mime_type,
                            } => {
                                // Interpolate the file path
                                let interpolated_path_str =
                                    interpolate_string(path, env, STRICT_INTERPOLATION)?;
                                // TODO: Might need to extend current working directory or some kinda base dire
                                let file_path = Path::new(&interpolated_path_str);

                                // Read the file content
                                let file_content =
                                    tokio::fs::read(file_path).await.context(format!(
                                        "Failed to read file for multipart part '{}': {:?}",
                                        name, file_path
                                    ))?;

                                let part = reqwest::multipart::Part::bytes(file_content).file_name(
                                    file_path
                                        .file_name()
                                        .unwrap_or_default()
                                        .to_string_lossy()
                                        .into_owned(),
                                );

                                // Add MIME type if specified
                                let part = if let Some(mime) = mime_type {
                                    let interpolated_mime =
                                        interpolate_string(mime, env, STRICT_INTERPOLATION)?;
                                    part.mime_str(&interpolated_mime)?
                                } else {
                                    part
                                };

                                form = form.part(name.clone(), part);
                            }
                        }
                    }
                    builder = builder.multipart(form);
                }
            }
        }

        // Build the final request
        let reqwest_request = builder.build().context("Failed to build reqwest request")?;

        Ok(reqwest_request)
    }
}

pub struct Response {
    status: i32,
}
