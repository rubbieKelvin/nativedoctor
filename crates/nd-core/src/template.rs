//! `${VAR}` and `${!dynamic}` expansion. `$${VAR}` produces a literal `${VAR}`.

use std::sync::OnceLock;

use regex::Regex;
use serde_json::{Map, Value};

use crate::dynamic;
use crate::error::Result;
use crate::vars::Variables;

#[derive(Debug, Clone, PartialEq)]
pub enum Reference {
    Var(String),
    Dynamic(String),
}

fn placeholder() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    return RE.get_or_init(|| {
        Regex::new(r"(\$?)\$\{(!?)([A-Za-z_][A-Za-z0-9_]*)\}").expect("valid regex")
    });
}

/// Expands once, so a variable's value is never expanded again.
pub fn expand(input: &str, vars: &Variables) -> Result<String> {
    let mut out = String::with_capacity(input.len());
    let mut last = 0;
    for caps in placeholder().captures_iter(input) {
        let whole = caps.get(0).expect("whole match");
        out.push_str(&input[last..whole.start()]);
        last = whole.end();

        let name = &caps[3];
        if !caps[1].is_empty() {
            out.push_str(&whole.as_str()[1..]);
        } else if !caps[2].is_empty() {
            out.push_str(&dynamic::invoke(name)?);
        } else {
            out.push_str(&vars.get(name)?);
        }
    }
    out.push_str(&input[last..]);
    return Ok(out);
}

/// Expands strings and object keys throughout a JSON value.
pub fn expand_json(value: &Value, vars: &Variables) -> Result<Value> {
    return match value {
        Value::String(text) => Ok(Value::String(expand(text, vars)?)),
        Value::Array(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                out.push(expand_json(item, vars)?);
            }
            Ok(Value::Array(out))
        }
        Value::Object(map) => {
            let mut out = Map::new();
            for (key, item) in map {
                out.insert(expand(key, vars)?, expand_json(item, vars)?);
            }
            Ok(Value::Object(out))
        }
        other => Ok(other.clone()),
    };
}

pub fn references(input: &str) -> Vec<Reference> {
    return placeholder()
        .captures_iter(input)
        .filter(|caps| caps[1].is_empty())
        .map(|caps| {
            let name = caps[3].to_string();
            if caps[2].is_empty() {
                Reference::Var(name)
            } else {
                Reference::Dynamic(name)
            }
        })
        .collect();
}

/// Collects references from every string and key in a JSON value.
pub fn references_in_json(value: &Value, out: &mut Vec<Reference>) {
    match value {
        Value::String(text) => out.extend(references(text)),
        Value::Array(items) => {
            for item in items {
                references_in_json(item, out);
            }
        }
        Value::Object(map) => {
            for (key, item) in map {
                out.extend(references(key));
                references_in_json(item, out);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use serde_json::json;

    fn vars() -> Variables {
        Variables::from_pairs(&[("HOST", "example.org"), ("LOOP", "${HOST}")])
    }

    #[test]
    fn expands_variables() {
        assert_eq!(
            expand("https://${HOST}/a", &vars()).unwrap(),
            "https://example.org/a"
        );
        assert_eq!(
            expand("no placeholders", &vars()).unwrap(),
            "no placeholders"
        );
    }

    #[test]
    fn values_arent_expanded_twice() {
        assert_eq!(expand("${LOOP}", &vars()).unwrap(), "${HOST}");
    }

    #[test]
    fn double_dollar_escapes() {
        assert_eq!(expand("$${HOST}", &vars()).unwrap(), "${HOST}");
        assert!(references("$${HOST}").is_empty());
    }

    #[test]
    fn leaves_non_placeholders_alone() {
        assert_eq!(
            expand("${a b} $HOST ${", &vars()).unwrap(),
            "${a b} $HOST ${"
        );
    }

    #[test]
    fn unknown_names_fail() {
        assert!(matches!(
            expand("${NOPE}", &vars()),
            Err(Error::UnknownVar(_))
        ));
        assert!(matches!(
            expand("${!nope}", &vars()),
            Err(Error::UnknownDynamic(_))
        ));
    }

    #[test]
    fn dynamic_values_are_fresh() {
        let a = expand("${!uuidv4}", &vars()).unwrap();
        let b = expand("${!uuidv4}", &vars()).unwrap();
        assert_eq!(a.len(), 36);
        assert_ne!(a, b);
    }

    #[test]
    fn expands_json_strings_and_keys_only() {
        let value = json!({ "${HOST}": ["${HOST}", 1, true, null] });
        assert_eq!(
            expand_json(&value, &vars()).unwrap(),
            json!({ "example.org": ["example.org", 1, true, null] })
        );
    }
}
