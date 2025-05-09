use super::yaml::{load_api_file, Request, Schema};
use anyhow::{Context, Result};
use async_recursion::async_recursion;
use serde::Deserialize;
use std::{collections::HashMap, env::current_dir};

#[derive(Debug, PartialEq, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    PATCH,
    OPTION,
    DELETE,
    TRACE,
    HEAD,
    CONNECT,
}

impl HttpMethod {
    fn from_str(value: &str) -> Result<Self> {
        let method = value.to_lowercase();
        return match method.as_str() {
            "get" => Ok(HttpMethod::GET),
            "post" => Ok(HttpMethod::POST),
            "put" => Ok(HttpMethod::PUT),
            "patch" => Ok(HttpMethod::PATCH),
            "delete" => Ok(HttpMethod::DELETE),
            "option" => Ok(HttpMethod::OPTION),
            "trace" => Ok(HttpMethod::TRACE),
            "connect" => Ok(HttpMethod::CONNECT),
            "head" => Ok(HttpMethod::HEAD),
            _ => anyhow::bail!("Invalid http method: {}", value),
        };
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(untagged)] // Use untagged so serde tries variants in order
pub enum EnvValue {
    String(String),
    Number(serde_yaml::Number), // Use serde_yaml::Number to preserve integer/float distinction
    Boolean(bool),
    Array(Vec<EnvValue>),              // Nested EnvValue for arrays
    Object(HashMap<String, EnvValue>), // Nested EnvValue for objects
    Null,
}

pub struct Runtime {
    pub schema: Schema,
    filename: String,
    environment: Option<String>,
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
        });
    }

    /// This should resolve the env variables by the current environment, and return a clean represengtation of the env
    pub fn build_env(&self) -> HashMap<String, EnvValue> {
        let mut env_vars = HashMap::<String, EnvValue>::new();

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

            // Convert the resolved serde_yaml::Value into our EnvValue enum
            // Using serde_yaml::from_value to deserialize directly
            let env_value: EnvValue = serde_yaml::from_value(resolved_value.clone()).unwrap_or_else(|_| {
                // Handle cases where the Value doesn't match EnvValue variants
                eprintln!("Warning: Failed to convert environment variable '{}' value {:?} to EnvValue. Treating as Null.", key, resolved_value);
                EnvValue::Null // Default to Null on conversion failure
            });

            env_vars.insert(key.clone(), env_value);
        }
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

                Ok(Response { status: 200 })
            }
            None => panic!(),
        };
    }

    pub fn build_request(&self, request_schema: Request) {}
}

pub struct Response {
    status: i32,
}

/// Interpolates variables in a string using the provided environment.
fn interpolate_string(template: &str, env: &HashMap<String, EnvValue>) -> Result<String> {
    let mut result = String::new();
    let mut last_end = 0;

    // Simple regex to find {{...}} patterns
    let re = regex::Regex::new(r"\{\{(.*?)\}\}").expect("Failed to compile regex"); // Regex should be valid

    for cap in re.captures_iter(template) {
        let full_match = cap.get(0).unwrap(); // The full {{...}} match
        let variable_name = cap.get(1).unwrap().as_str().trim(); // The content inside {{...}}

        // Append text before the match
        result.push_str(&template[last_end..full_match.start()]);

        // Look up the variable in the environment
        if let Some(env_value) = env.get(variable_name) {
            // Convert the EnvValue to a string representation
            let value_str = match env_value {
                EnvValue::String(s) => s.clone(),
                EnvValue::Number(n) => n.to_string(),
                EnvValue::Boolean(b) => b.to_string(),
                EnvValue::Null => "null".to_string(),
                // Handle Array and Object by serializing to YAML string
                EnvValue::Array(_) | EnvValue::Object(_) => serde_yaml::to_string(env_value)
                    .unwrap_or_else(|_| "[Serialization Error]".to_string())
                    .trim_end_matches('\n')
                    .to_string(),
            };
            result.push_str(&value_str);
        } else {
            // Variable not found, decide how to handle (e.g., error, empty string, keep template)
            // For now, let's keep the original template string for the variable
            eprintln!(
                "Warning: Environment variable '{}' not found.",
                variable_name
            );
            result.push_str(full_match.as_str());
            // Or you could return an error:
            // bail!("Environment variable '{}' not found", variable_name);
        }

        last_end = full_match.end();
    }

    // Append remaining text after the last match
    result.push_str(&template[last_end..]);

    Ok(result)
}
