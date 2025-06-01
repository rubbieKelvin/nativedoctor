use std::collections::HashMap;

use anyhow::{Ok, Result};
use uuid::Uuid;

// NOTE: Since i introduced the fact that env could be any json/yaml object
// i havent tested the modifications

pub const STRICT_INTERPOLATION: bool = false;

fn create_dynammic_variables() -> HashMap<String, Box<dyn Fn() -> String>> {
    let mut record: HashMap<String, Box<dyn Fn() -> String>> = HashMap::new();

    record.insert("#uuid".to_string(), Box::new(|| Uuid::new_v4().to_string()));
    return record;
}

/// Interpolates variables in a string using the provided environment.
pub fn interpolate_string(
    template: &str,
    env: &HashMap<String, serde_yaml::Value>,
    strict: bool,
) -> Result<String> {
    let mut result = String::new();
    let mut last_end = 0;
    let dynamic_variables: HashMap<String, Box<dyn Fn() -> String>> = create_dynammic_variables();

    // Simple regex to find {{...}} patterns
    let re = regex::Regex::new(r"\{\{(.*?)\}\}").expect("Failed to compile regex"); // Regex should be valid

    for cap in re.captures_iter(template) {
        let full_match = cap.get(0).unwrap(); // The full {{...}} match
        let variable_name = cap.get(1).unwrap().as_str().trim(); // The content inside {{...}}

        // Append text before the match
        result.push_str(&template[last_end..full_match.start()]);

        // Look up the variable in the environment
        if let Some(env_value) = env.get(variable_name) {
            // atp the env value here must be string
            // TODO: i guess
            match env_value {
                serde_yaml::Value::String(s) => {
                    result.push_str(s);
                }
                _ => anyhow::bail!("This should be string??"),
            };
        // Look up dynamic variables
        } else if let Some(dynamic_func) = dynamic_variables.get(variable_name) {
            let string = dynamic_func();
            result.push_str(&string);
        } else {
            // Variable not found, decide how to handle (e.g., error, empty string, keep template)
            // For now, let's keep the original template string for the variable
            eprintln!(
                "Warning: Environment variable '{}' not found.",
                variable_name
            );

            // Couldnt find variable, if we're in strict mode, raise an err
            if strict {
                anyhow::bail!("Environment variable '{}' not found", variable_name);
            } else {
                result.push_str(full_match.as_str());
            }
        }

        last_end = full_match.end();
    }

    // Append remaining text after the last match
    result.push_str(&template[last_end..]);

    Ok(result)
}

/// Interpolates variables recursively within a serde_yaml::Value.
pub fn interpolate_value(
    value: &serde_yaml::Value,
    env: &HashMap<String, serde_yaml::Value>,
) -> Result<serde_yaml::Value> {
    match value {
        serde_yaml::Value::String(s) => Ok(serde_yaml::Value::String(interpolate_string(
            s,
            env,
            STRICT_INTERPOLATION,
        )?)),
        serde_yaml::Value::Sequence(seq) => {
            let interpolated_seq: Result<Vec<serde_yaml::Value>> =
                seq.iter().map(|v| interpolate_value(v, env)).collect();
            Ok(serde_yaml::Value::Sequence(interpolated_seq?))
        }
        serde_yaml::Value::Mapping(map) => {
            let mut interpolated_map = serde_yaml::Mapping::new();
            for (k, v) in map.iter() {
                // Interpolate both key and value (though keys are less likely to have variables)
                let interpolated_key = interpolate_value(k, env)?;
                let interpolated_value = interpolate_value(v, env)?;
                interpolated_map.insert(interpolated_key, interpolated_value);
            }
            Ok(serde_yaml::Value::Mapping(interpolated_map))
        }
        // Numbers, Booleans, and Null don't contain strings to interpolate
        _ => Ok(value.clone()),
    }
}

/// Interpolate as json value
pub fn interpolate_json(
    value: &serde_yaml::Value,
    env: HashMap<String, serde_yaml::Value>,
) -> Result<String> {
    let json = serde_json::to_value(interpolate_value(value, &env)?)?;
    return Ok(json.to_string());
}
