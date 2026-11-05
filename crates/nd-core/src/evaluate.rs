//! Evaluates `capture` and `assert` against a response.

use indexmap::IndexMap;
use serde_json::Value;
use serde_json_path::JsonPath;

use crate::error::{Error, Result};
use crate::pairs::scalar_to_string;
use crate::request::Assertions;
use crate::template::{expand, expand_json};
use crate::vars::Variables;

pub struct ResponseView<'a> {
    pub status: u16,
    pub headers: &'a [(String, String)],
    pub body: &'a [u8],
}

impl ResponseView<'_> {
    fn json(&self) -> Option<Value> {
        serde_json::from_slice(self.body).ok()
    }

    fn header(&self, name: &str) -> Option<&str> {
        return self
            .headers
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, value)| value.as_str());
    }
}

/// Checks that a capture source is well formed.
pub fn validate_capture(source: &str) -> Result<()> {
    if source.starts_with("header:") {
        return Ok(());
    }
    if !source.starts_with('$') {
        return Err(Error::Invalid(format!(
            "capture `{source}` must start with `$` (JSONPath) or `header:`"
        )));
    }
    return parse_path(source).map(|_| ());
}

pub fn parse_path(raw: &str) -> Result<JsonPath> {
    return JsonPath::parse(raw)
        .map_err(|e| Error::Invalid(format!("invalid JSONPath `{raw}`: {e}")));
}

/// Returns captured values in declaration order. Any capture that finds nothing is an error.
pub fn capture(
    captures: &IndexMap<String, String>,
    response: &ResponseView,
) -> Result<Vec<(String, String)>> {
    let json = if captures.is_empty() {
        None
    } else {
        response.json()
    };
    let mut values = Vec::with_capacity(captures.len());
    for (name, source) in captures {
        let missing = |why: &str| Error::Invalid(format!("capture `{name}` ({source}): {why}"));
        validate_capture(source)?;

        if let Some(header) = source.strip_prefix("header:") {
            let value = response
                .header(header)
                .ok_or_else(|| missing("header not found"))?;
            values.push((name.clone(), value.to_string()));
            continue;
        }

        let body = json
            .as_ref()
            .ok_or_else(|| missing("response isn't JSON"))?;
        let found = parse_path(source)?
            .query(body)
            .first()
            .cloned()
            .ok_or_else(|| missing("no match in response"))?;
        let value = scalar_to_string(&found).unwrap_or_else(|| found.to_string());
        values.push((name.clone(), value));
    }
    return Ok(values);
}

/// Returns one message per failed assertion.
pub fn check(
    assertions: &Assertions,
    response: &ResponseView,
    vars: &Variables,
) -> Result<Vec<String>> {
    let mut failures = Vec::new();

    if !assertions.status.is_empty() && !assertions.status.contains(&response.status) {
        let expected: Vec<String> = assertions.status.iter().map(u16::to_string).collect();
        failures.push(format!(
            "status: expected {}, got {}",
            expected.join(" or "),
            response.status
        ));
    }

    for (name, expected) in &assertions.headers {
        let expected = expand(expected, vars)?;
        match response.header(name) {
            None => failures.push(format!("header {name}: missing")),
            Some(actual) if !actual.contains(&expected) => {
                failures.push(format!(
                    "header {name}: expected `{expected}` in `{actual}`"
                ));
            }
            Some(_) => {}
        }
    }

    if !assertions.body.is_empty() {
        let json = response.json();
        for (path, expected) in &assertions.body {
            let expected = expand_json(expected, vars)?;
            let Some(body) = &json else {
                failures.push(format!("body {path}: response isn't JSON"));
                continue;
            };
            let found = parse_path(path)?.query(body).first().cloned();
            match found {
                None => failures.push(format!("body {path}: no match, expected {expected}")),
                Some(actual) if !values_equal(&actual, &expected) => {
                    failures.push(format!("body {path}: expected {expected}, got {actual}"));
                }
                Some(_) => {}
            }
        }
    }
    return Ok(failures);
}

/// Numbers compare by value, so `1.0` equals `1`.
fn values_equal(a: &Value, b: &Value) -> bool {
    if let (Value::Number(x), Value::Number(y)) = (a, b) {
        return x == y || x.as_f64() == y.as_f64();
    }
    return a == b;
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn view<'a>(body: &'a [u8], headers: &'a [(String, String)]) -> ResponseView<'a> {
        return ResponseView {
            status: 200,
            headers,
            body,
        };
    }

    fn map(entries: &[(&str, &str)]) -> IndexMap<String, String> {
        return entries
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect();
    }

    #[test]
    fn captures_json_and_headers() {
        let headers = [("X-Token".to_string(), "hdr".to_string())];
        let body = br#"{"access_token":"abc","user":{"id":7},"tags":["a"]}"#;
        let values = capture(
            &map(&[
                ("T", "$.access_token"),
                ("ID", "$.user.id"),
                ("TAGS", "$.tags"),
                ("H", "header:x-token"),
            ]),
            &view(body, &headers),
        )
        .unwrap();
        assert_eq!(values[0], ("T".to_string(), "abc".to_string()));
        assert_eq!(values[1], ("ID".to_string(), "7".to_string()));
        assert_eq!(values[2], ("TAGS".to_string(), r#"["a"]"#.to_string()));
        assert_eq!(values[3], ("H".to_string(), "hdr".to_string()));
    }

    #[test]
    fn capture_failures_name_the_variable() {
        let message = capture(&map(&[("T", "$.nope")]), &view(b"{}", &[]))
            .unwrap_err()
            .to_string();
        assert!(message.contains("`T`"), "{message}");
        assert!(capture(&map(&[("T", "$.a")]), &view(b"<html>", &[])).is_err());
        assert!(capture(&map(&[("T", "header:x")]), &view(b"{}", &[])).is_err());
        assert!(capture(&map(&[("T", "access_token")]), &view(b"{}", &[])).is_err());
    }

    #[test]
    fn passing_assertions_report_nothing() {
        let headers = [(
            "Content-Type".to_string(),
            "application/json; charset=utf-8".to_string(),
        )];
        let assertions = Assertions {
            status: vec![200, 201],
            headers: map(&[("content-type", "application/json")]),
            body: [
                ("$.name".to_string(), json!("John")),
                ("$.n".to_string(), json!(1)),
            ]
            .into_iter()
            .collect(),
        };
        let vars = Variables::default();
        let response = view(br#"{"name":"John","n":1.0}"#, &headers);
        assert!(check(&assertions, &response, &vars).unwrap().is_empty());
    }

    #[test]
    fn failing_assertions_explain_each_failure() {
        let assertions = Assertions {
            status: vec![201],
            headers: map(&[("x-missing", "a")]),
            body: [
                ("$.name".to_string(), json!("${WHO}")),
                ("$.gone".to_string(), json!(1)),
            ]
            .into_iter()
            .collect(),
        };
        let vars = Variables::from_pairs(&[("WHO", "Jane")]);
        let failures = check(&assertions, &view(br#"{"name":"John"}"#, &[]), &vars).unwrap();
        assert_eq!(failures.len(), 4, "{failures:?}");
        assert!(failures[0].contains("expected 201, got 200"));
        assert!(failures[2].contains("Jane"));
    }
}
