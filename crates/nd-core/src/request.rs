//! The `*.request.yaml` format.

use std::fs;
use std::path::Path;

use indexmap::IndexMap;
use serde::de::Error as _;
use serde::ser::{SerializeMap, SerializeSeq};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde_json::{Map, Value};

use crate::error::{Error, Result};
use crate::pairs::{scalar_to_string, Pairs};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequestFile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub http: HttpSpec,
    /// Variable name -> `$.json.path` or `header:Name`.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub capture: IndexMap<String, String>,
    #[serde(
        default,
        rename = "assert",
        skip_serializing_if = "Assertions::is_empty"
    )]
    pub assertions: Assertions,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HttpSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    pub url: String,
    #[serde(default, skip_serializing_if = "Pairs::is_empty")]
    pub query: Pairs,
    #[serde(default, skip_serializing_if = "Pairs::is_empty")]
    pub headers: Pairs,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<Body>,
    /// `500ms`, `30s`, `2m`, or a number of seconds. `0` disables it.
    #[serde(
        default,
        deserialize_with = "optional_scalar",
        skip_serializing_if = "Option::is_none"
    )]
    pub timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follow_redirects: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_redirects: Option<usize>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub verify_tls: Option<bool>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Body {
    Json(Value),
    Text(String),
    Form(Pairs),
    Multipart(IndexMap<String, Part>),
    File(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Part {
    Text(String),
    File {
        file: String,
        content_type: Option<String>,
    },
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Assertions {
    /// One status or a list of accepted statuses.
    #[serde(
        default,
        deserialize_with = "one_or_many",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub status: Vec<u16>,
    /// Header name -> text its value must contain.
    #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
    pub headers: IndexMap<String, String>,
    /// `$.json.path` -> expected value.
    #[serde(
        default,
        serialize_with = "serialize_expected",
        skip_serializing_if = "IndexMap::is_empty"
    )]
    pub body: IndexMap<String, Value>,
}

/// Writes a JSON value with real numbers. `arbitrary_precision` makes a `Number` serialize as a
/// private struct, which only serde_json's own serializer understands.
struct PlainJson<'a>(&'a Value);

impl Serialize for PlainJson<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        match self.0 {
            Value::Number(number) => {
                if let Some(int) = number.as_i64() {
                    return serializer.serialize_i64(int);
                }
                if let Some(int) = number.as_u64() {
                    return serializer.serialize_u64(int);
                }
                if let Some(float) = number.as_f64().filter(|float| float.is_finite()) {
                    return serializer.serialize_f64(float);
                }
                return serializer.serialize_str(&number.to_string());
            }
            Value::Array(items) => {
                let mut seq = serializer.serialize_seq(Some(items.len()))?;
                for item in items {
                    seq.serialize_element(&PlainJson(item))?;
                }
                return seq.end();
            }
            Value::Object(fields) => {
                let mut map = serializer.serialize_map(Some(fields.len()))?;
                for (name, item) in fields {
                    map.serialize_entry(name, &PlainJson(item))?;
                }
                return map.end();
            }
            other => return other.serialize(serializer),
        }
    }
}

fn serialize_expected<S: Serializer>(
    expected: &IndexMap<String, Value>,
    serializer: S,
) -> std::result::Result<S::Ok, S::Error> {
    let mut map = serializer.serialize_map(Some(expected.len()))?;
    for (path, value) in expected {
        map.serialize_entry(path, &PlainJson(value))?;
    }
    return map.end();
}

impl Assertions {
    pub fn is_empty(&self) -> bool {
        self.status.is_empty() && self.headers.is_empty() && self.body.is_empty()
    }
}

impl RequestFile {
    pub fn load(path: &Path) -> Result<RequestFile> {
        let text = fs::read_to_string(path).map_err(|e| Error::io(path, e))?;
        return RequestFile::parse(&text).map_err(|e| match e {
            ParseError::Yaml(source) => Error::Yaml {
                path: path.to_path_buf(),
                source,
            },
            ParseError::Unsupported(protocol) => Error::Unsupported(format!(
                "{}: {protocol} requests aren't supported yet",
                path.display()
            )),
        });
    }

    /// Parses text that isn't in a file yet, like the desktop app's YAML tab.
    pub fn from_yaml(text: &str) -> Result<RequestFile> {
        return RequestFile::parse(text).map_err(|e| match e {
            ParseError::Yaml(source) => Error::Invalid(source.to_string()),
            ParseError::Unsupported(protocol) => {
                Error::Unsupported(format!("{protocol} requests aren't supported yet"))
            }
        });
    }

    /// Saves over `path`, keeping its comments and formatting. See [`crate::patch`].
    pub fn save(&self, path: &Path) -> Result<()> {
        return crate::patch::save_model(path, self);
    }

    fn parse(text: &str) -> std::result::Result<RequestFile, ParseError> {
        let error = match serde_yaml_ng::from_str(text) {
            Ok(file) => return Ok(file),
            Err(error) => error,
        };
        // Give planned protocols a clearer message than "unknown field".
        if let Ok(raw) = serde_yaml_ng::from_str::<Map<String, Value>>(text) {
            for protocol in ["ws", "grpc"] {
                if raw.contains_key(protocol) {
                    return Err(ParseError::Unsupported(protocol));
                }
            }
        }
        return Err(ParseError::Yaml(error));
    }

    /// Display name: `name`, or the file name without its suffix.
    pub fn display_name(&self, path: &Path) -> String {
        if let Some(name) = &self.name {
            return name.clone();
        }
        let file = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("request");
        return file
            .trim_end_matches(nd_constants::REQUEST_SUFFIX)
            .to_string();
    }
}

enum ParseError {
    Yaml(serde_yaml_ng::Error),
    Unsupported(&'static str),
}

fn optional_scalar<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<Option<String>, D::Error> {
    let value = Value::deserialize(deserializer)?;
    if value.is_null() {
        return Ok(None);
    }
    return scalar_to_string(&value)
        .map(Some)
        .ok_or_else(|| D::Error::custom("expected a string or number"));
}

fn one_or_many<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> std::result::Result<Vec<u16>, D::Error> {
    let value = Value::deserialize(deserializer)?;
    let values = match value {
        Value::Array(values) => values,
        single => vec![single],
    };
    return values
        .iter()
        .map(|v| {
            v.as_u64()
                .and_then(|n| u16::try_from(n).ok())
                .ok_or_else(|| D::Error::custom("status must be a number or a list of numbers"))
        })
        .collect();
}

// Hand-written because the file format is always a one-key map, like `body: { json: ... }`.
impl<'de> Deserialize<'de> for Body {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        const EXPECTED: &str = "body must have exactly one of: json, text, form, multipart, file";
        let map = Map::deserialize(deserializer)?;
        if map.len() != 1 {
            return Err(D::Error::custom(EXPECTED));
        }
        let (kind, value) = map.into_iter().next().expect("map has one entry");
        return match kind.as_str() {
            "json" => Ok(Body::Json(value)),
            "text" => scalar_to_string(&value)
                .map(Body::Text)
                .ok_or_else(|| D::Error::custom("`text` must be a string")),
            "form" => serde_json::from_value(value)
                .map(Body::Form)
                .map_err(D::Error::custom),
            "multipart" => parse_multipart::<D::Error>(value).map(Body::Multipart),
            "file" => value
                .as_str()
                .map(|path| Body::File(path.to_string()))
                .ok_or_else(|| D::Error::custom("`file` must be a path")),
            _ => Err(D::Error::custom(EXPECTED)),
        };
    }
}

fn parse_multipart<E: serde::de::Error>(
    value: Value,
) -> std::result::Result<IndexMap<String, Part>, E> {
    let Value::Object(map) = value else {
        return Err(E::custom("`multipart` must be a map"));
    };
    let mut parts = IndexMap::new();
    for (name, value) in map {
        let part = match &value {
            Value::Object(fields) => {
                let file = fields.get("file").and_then(Value::as_str).ok_or_else(|| {
                    E::custom(format!("multipart field `{name}` needs a `file` path"))
                })?;
                Part::File {
                    file: file.to_string(),
                    content_type: fields
                        .get("type")
                        .and_then(Value::as_str)
                        .map(str::to_string),
                }
            }
            other => Part::Text(scalar_to_string(other).ok_or_else(|| {
                E::custom(format!(
                    "multipart field `{name}` must be a scalar or a file"
                ))
            })?),
        };
        parts.insert(name, part);
    }
    return Ok(parts);
}

impl Serialize for Body {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(1))?;
        match self {
            Body::Json(value) => map.serialize_entry("json", &PlainJson(value))?,
            Body::Text(text) => map.serialize_entry("text", text)?,
            Body::Form(pairs) => map.serialize_entry("form", pairs)?,
            Body::File(path) => map.serialize_entry("file", path)?,
            Body::Multipart(parts) => {
                let mut fields = Map::new();
                for (name, part) in parts {
                    fields.insert(name.clone(), part.to_value());
                }
                map.serialize_entry("multipart", &fields)?;
            }
        }
        return map.end();
    }
}

impl Part {
    fn to_value(&self) -> Value {
        match self {
            Part::Text(text) => return Value::String(text.clone()),
            Part::File { file, content_type } => {
                let mut fields = Map::new();
                fields.insert("file".to_string(), Value::String(file.clone()));
                if let Some(content_type) = content_type {
                    fields.insert("type".to_string(), Value::String(content_type.clone()));
                }
                return Value::Object(fields);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(text: &str) -> RequestFile {
        serde_yaml_ng::from_str(text).unwrap()
    }

    #[test]
    fn parses_a_full_request() {
        let file = parse(
            r#"
name: Create user
http:
  method: POST
  url: ${BASE_URL}/users
  query: { page: 2, tag: [a, b] }
  headers: { x-debug: true, accept: null }
  body:
    json: { name: John, age: 29 }
  timeout: 5
capture:
  USER_ID: $.id
assert:
  status: [200, 201]
  body:
    $.name: John
"#,
        );
        assert_eq!(file.http.query.0.len(), 3);
        assert_eq!(file.http.query.0[0], ("page".into(), Some("2".into())));
        assert_eq!(file.http.headers.0[1], ("accept".into(), None));
        assert_eq!(file.http.timeout.as_deref(), Some("5"));
        assert!(matches!(file.http.body, Some(Body::Json(_))));
        assert_eq!(file.assertions.status, vec![200, 201]);
        assert_eq!(file.capture["USER_ID"], "$.id");
    }

    #[test]
    fn body_kinds_round_trip_through_yaml() {
        let original = parse(
            r#"
http:
  url: http://x
  query: { a: [1, 2] }
  body:
    multipart:
      title: hello
      cv: { file: ./cv.pdf, type: application/pdf }
"#,
        );
        let text = serde_yaml_ng::to_string(&original).unwrap();
        let again = parse(&text);
        assert_eq!(again.http.body, original.http.body);
        assert_eq!(again.http.query, original.http.query);
    }

    #[test]
    fn numbers_are_written_as_numbers() {
        let original = parse(
            "http:\n  url: x\n  body:\n    json: { age: 29, price: 1.5, list: [1] }\nassert:\n  body:\n    $.age: 29\n",
        );
        let text = serde_yaml_ng::to_string(&original).unwrap();
        assert!(!text.contains("serde_json"), "{text}");
        let again = parse(&text);
        assert_eq!(again.http.body, original.http.body);
        assert_eq!(again.assertions.body, original.assertions.body);
    }

    #[test]
    fn rejects_typos_and_ambiguous_bodies() {
        let bad = [
            "http: { url: x, metod: GET }",
            "http: { url: x, body: { json: {}, text: y } }",
            "http: { url: x, body: { yaml: y } }",
            "http: { url: x }\ncaptur: {}",
        ];
        for text in bad {
            assert!(
                serde_yaml_ng::from_str::<RequestFile>(text).is_err(),
                "{text}"
            );
        }
    }

    #[test]
    fn planned_protocols_get_a_clear_error() {
        let result = RequestFile::parse("name: Feed\nws:\n  url: wss://x\n");
        assert!(matches!(result, Err(ParseError::Unsupported("ws"))));
    }

    #[test]
    fn preserves_key_order() {
        let file = parse("http: { url: x, body: { json: { z: 1.10, a: 2 } } }");
        let Some(Body::Json(value)) = file.http.body else {
            panic!("expected json");
        };
        assert_eq!(value.to_string(), r#"{"z":1.1,"a":2}"#);
    }
}
