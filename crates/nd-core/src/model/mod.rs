mod request;
mod sequence;

/// Inserts [`$schema`](https://json-schema.org/draft/2020-12/json-schema-core.html#name-the-schema-keyword)
/// at the root of a JSON object, before other keys, so editors and validators resolve the public
/// schema URL (see [`nd_constants::urls`]).
pub fn with_root_schema_url(root: serde_json::Value, schema_url: &str) -> serde_json::Value {
    match root {
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
    }
}

pub use request::{
    content_type_for_body_kind, request_file_json_schema, HttpRequestSpec, RequestBody,
    RequestBodyKind, RequestBodyStructured, RequestFile,
};
pub use sequence::{sequence_file_json_schema, SequenceFile, SequenceStep};
