use super::{
    schema::{load_api_file, MultipartPart, Request, RequestBody, Schema},
    utils::{interpolate_string, interpolate_value, STRICT_INTERPOLATION},
};
use anyhow::{Context, Ok, Result};
use std::{
    collections::{HashMap, HashSet},
    env::current_dir,
    path::Path,
};
use tracing::info;

pub struct Runner {
    pub schema: Schema,
    #[allow(unused)]
    filename: String,
    environment: Option<String>,
    /// Variables we override at runtime (Impossible for now)
    // NOTE: We might want to clear this per call sequence
    // Or maybe use a unique runtime for each, then run them in parallel
    // TODO: remove lint rule
    #[allow(unused)]
    overrides: HashMap<String, String>,
}

impl Runner {
    /// Imports the .yaml file and loads it into the schema. all the imports from the .yaml file
    /// are loaded into a list and merged with the root schema.
    /// rootpath is the path where the yaml file lives.
    /// is_root means we called this path as the root path (thte path that imports all other files)
    fn recursively_import(rootpath: &Path, is_root: bool) -> Result<Schema> {
        let mut schema = load_api_file(rootpath)?;

        // We need to ensure of all the loaded files
        // only the root optionally has project definition
        if !is_root && schema.project.is_some() {
            anyhow::bail!(
                "Project definition specified for none root file: {}",
                rootpath
                    .to_str()
                    .context("Cannot convert root path to string")
                    .unwrap()
            );
        }

        let imported_schemas = schema
            .imports
            .iter()
            .map(|name| {
                let p = rootpath
                    .parent()
                    .context("Unable to read parent directory of file path")
                    .unwrap();
                let p = p.join(name);
                return Runner::recursively_import(p.as_path(), false).unwrap();
            })
            .collect::<Vec<Schema>>();

        for i_schema in imported_schemas.iter() {
            // extend env with the imported data
            for (key, value) in i_schema.env.iter() {
                // Do not override root import env
                if schema.env.contains_key(key) {
                    anyhow::bail!(
                        "Conflicting variable names: File at {} is attempting to override env value at {}",
                        schema.filename,
                        i_schema.filename
                    );
                }

                // only extend
                schema.env.insert(key.clone(), value.clone());
            }

            // extend requests
            for (key, value) in i_schema.requests.iter() {
                // Do not override root import requests
                if schema.requests.contains_key(key) {
                    anyhow::bail!(
                        "Conflicting request names: File at {} is attempting to override request at {}",
                        schema.filename,
                        i_schema.filename
                    );
                }

                // only extend
                schema.requests.insert(key.clone(), value.clone());
            }

            // extend sequence
            for (key, value) in i_schema.calls.iter() {
                // Do not override root import sequence
                if schema.calls.contains_key(key) {
                    anyhow::bail!(
                        "Conflicting sequence names: File at {} is attempting to override call sequence at {}",
                        schema.filename,
                        i_schema.filename
                    );
                }

                // only extend
                schema.calls.insert(key.clone(), value.clone());
            }
        }

        return Ok(schema);
    }

    #[allow(unused)]
    pub fn from_schema(schema: Schema, environment: Option<String>) -> Self {
        return Runner {
            schema,
            filename: String::new(),
            environment,
            overrides: HashMap::new(),
        };
    }

    pub fn new(filename: &str, environment: Option<String>) -> Result<Self> {
        let cwd = current_dir().context("Could not get the current working directorty")?;
        let path = cwd.join(filename);

        // Make sure the environment is not Some('default'). i regard this as absured, just set this shii to None.
        if let Some(specified_env) = &environment {
            assert_ne!(specified_env.to_lowercase(), "default");
        }

        // let schema = load_api_file(path.as_path())?;
        let schema = Runner::recursively_import(path.as_path(), true)?;

        return Ok(Runner {
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

            // env_vars.insert(key.clone(), resolved_value.to_string());
            // env values might need interpolation.
            // although there WILL be cases where an interpolated value would need a value that isnt in env yet
            // i'll handle this later
            env_vars.insert(
                key.clone(),
                interpolate_string(resolved_value, &env_vars, STRICT_INTERPOLATION).unwrap(),
            );
        }

        // TODO: Override with other variables from teh override props
        return env_vars;
    }

    pub async fn call_request(
        &self,
        name: String,
        client: &reqwest::Client,
    ) -> Result<reqwest::Response> {
        info!("Calling bare request \"{}\"", &name);
        let request = self.schema.requests.get(&name);

        return match request {
            Some(request) => {
                // now let's build the request
                let req = self.build_request(request, client).await?;
                let response = client
                    .execute(req)
                    .await
                    .context("Failed to execute request")?;

                Ok(response)
            }
            None => panic!(),
        };
    }

    /// Builds a reqwest::Request from a Request schema and resolved environment variables.
    pub async fn build_request(
        &self,
        request_schema: &Request,
        client: &reqwest::Client,
    ) -> Result<reqwest::Request> {
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

    /// NOTE: THis doens handle circular dep yet
    #[deprecated]
    pub fn generate_request_call_queue(&self, name: String) -> Result<Vec<String>> {
        // check if the request exists
        let request = match self.schema.requests.get(&name) {
            Some(request) => request,
            None => {
                anyhow::bail!("Request with name \"{name}\" does not exist");
            }
        };

        // add the original rewuesnt name
        let mut queue = vec![name];

        // now lets add dependencies
        let dependencies = match &request.config {
            Some(config) => config.depends_on.clone(),
            None => vec![],
        };

        for dependency in dependencies {
            let inner_dep = self.generate_request_call_queue(dependency)?;
            queue.extend(inner_dep);
        }

        // remove duplicate
        let mut dedup = vec![];

        for i in queue {
            if !dedup.contains(&i) {
                dedup.push(i);
            }
        }

        return Ok(dedup);
    }

    /// NOTE: THis doens handle circular dep yet
    #[deprecated]
    pub fn generate_request_call_queue_from_sequence(&self, name: String) -> Result<Vec<String>> {
        let mut queue: Vec<String> = vec![];
        let sequence = match self.schema.calls.get(&name) {
            Some(sq) => sq,
            None => {
                anyhow::bail!("Request with name \"{name}\" does not exist");
            }
        };

        for entry in sequence {
            if entry.starts_with("/") {
                let actual_name = &entry[1..];
                let dependencies =
                    self.generate_request_call_queue_from_sequence(actual_name.to_string())?;
                queue.extend(dependencies);
            } else {
                let dependencies = self.generate_request_call_queue(entry.to_string())?;
                queue.extend(dependencies);
            }
        }

        // remove duplicate
        let mut dedup = vec![];

        for i in queue {
            if !dedup.contains(&i) {
                dedup.push(i);
            }
        }

        return Ok(dedup);
    }

    pub fn generate_call_queue(&self, name: &str) -> Result<Vec<String>> {
        let dirty_stack = self.traverse_request_stack(name, HashSet::new())?;
        let mut seen: Vec<String> = vec![];
        let stack = dirty_stack
            .iter()
            .rev()
            .filter(|i| {
                if seen.contains(i) {
                    return false;
                };
                seen.push(i.to_string());
                return true;
            })
            .map(|i| i.to_string());
        return Ok(stack.collect::<Vec<String>>());
    }

    fn traverse_request_stack(
        &self,
        name: &str,
        mut dependency_trace: HashSet<String>,
    ) -> Result<Vec<String>> {
        // verify dependency trace tp check for circular dependency
        if dependency_trace.contains(name) {
            let mut trace_info = String::new();

            for dep in dependency_trace.iter() {
                trace_info.push_str(&format!("{dep} -> "));
            }

            trace_info.push_str(&format!("error({name})"));

            anyhow::bail!(
                "Circular dependency detected in request stack; attempted to call {}\nwhich has alread been called in this trace: {}",
                name,
                trace_info
            );
        }

        // add this dependency to the
        dependency_trace.insert(name.to_string());

        let mut stack: Vec<String> = vec![];
        let request = match self.schema.requests.get(name) {
            Some(request) => request,
            None => {
                anyhow::bail!("Request with name \"{name}\" does not exist");
            }
        };

        // now lets add dependencies
        let dependencies = match &request.config {
            Some(config) => config.depends_on.clone(),
            None => vec![],
        };

        // then the actual request
        stack.push(name.to_string());

        for dep in dependencies.iter() {
            let inner_dependency_for_dep =
                self.traverse_request_stack(dep, dependency_trace.clone())?;
            stack.extend(inner_dependency_for_dep);
        }

        return Ok(stack);
    }
}
