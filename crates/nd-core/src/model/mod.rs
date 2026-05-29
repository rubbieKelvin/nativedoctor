use std::path::Path;

use crate::error::Result;

pub mod defaults;
pub mod project;
pub mod request;
pub mod sequence;

/// inserts "$schema" at the root of a JSON object, before other keys, so editors and validators resolve the public schema URL
pub fn with_root_schema_url(root: serde_json::Value, schema_url: &str) -> serde_json::Value {
    return match root {
        serde_json::Value::Object(mut obj) => {
            obj.remove("$schema");
            let mut out = serde_json::Map::with_capacity(obj.len() + 1);
            out.insert(
                "$schema".to_string(),
                serde_json::Value::String(schema_url.to_string()),
            );
            out.extend(obj);
            serde_json::Value::Object(out)
        }
        other => other,
    };
}
