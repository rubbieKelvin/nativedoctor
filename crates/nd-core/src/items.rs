//! HTTPie-style request items: `Header:value`, `key==value`, `key=value`, `key:=json`, `cv@file`.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};

use crate::error::{Error, Result};
use crate::pairs::Pairs;
use crate::request::{Body, HttpSpec, Part};

/// Longest first, so `:=` wins over `:` at the same position.
const SEPARATORS: [&str; 10] = [":=@", "==@", ":=", ":@", "==", "=@", "=", ":", ";", "@"];

/// Largest explicit array index, so a typo can't allocate gigabytes.
const MAX_INDEX: usize = 65_535;

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    /// `None` removes the header.
    Header(String, Option<String>),
    HeaderFile(String, String),
    Query(String, String),
    QueryFile(String, String),
    Field(Vec<Segment>, String),
    FieldFile(Vec<Segment>, String),
    Json(Vec<Segment>, Value),
    JsonFile(Vec<Segment>, String),
    Upload {
        field: String,
        file: String,
        content_type: Option<String>,
    },
}

impl Item {
    /// True if the item goes in the request body.
    pub fn is_data(&self) -> bool {
        return matches!(
            self,
            Item::Field(..)
                | Item::FieldFile(..)
                | Item::Json(..)
                | Item::JsonFile(..)
                | Item::Upload { .. }
        );
    }

    /// Makes a relative file path relative to `base`, for callers with no useful working folder.
    pub fn rebase(&mut self, base: &Path) {
        let file = match self {
            Item::HeaderFile(_, file)
            | Item::QueryFile(_, file)
            | Item::FieldFile(_, file)
            | Item::JsonFile(_, file)
            | Item::Upload { file, .. } => file,
            _ => return,
        };
        if Path::new(file.as_str()).is_relative() {
            *file = base.join(file.as_str()).display().to_string();
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Segment {
    Key(String),
    Index(usize),
    Append,
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum BodyMode {
    #[default]
    Auto,
    Form,
    Multipart,
}

pub fn parse(raw: &str) -> Result<Item> {
    let fail = |reason: String| Error::Item {
        item: raw.to_string(),
        reason,
    };
    let (key, separator, value) =
        split(raw).ok_or_else(|| fail("no separator found (`:`, `==`, `=`, `:=`, `@`)".into()))?;
    let path = || parse_path(key).map_err(&fail);
    let name = || {
        if key.is_empty() {
            return Err(fail("the name is empty".into()));
        }
        return Ok(unescape(key));
    };

    return match separator {
        ":" if value.is_empty() => Ok(Item::Header(name()?, None)),
        ":" => Ok(Item::Header(name()?, Some(value.to_string()))),
        ";" => Ok(Item::Header(name()?, Some(String::new()))),
        ":@" => Ok(Item::HeaderFile(name()?, value.to_string())),
        "==" => Ok(Item::Query(name()?, value.to_string())),
        "==@" => Ok(Item::QueryFile(name()?, value.to_string())),
        "=" => Ok(Item::Field(path()?, unescape_at(value))),
        "=@" => Ok(Item::FieldFile(path()?, value.to_string())),
        ":=" => {
            let parsed = serde_json::from_str(value)
                .map_err(|e| fail(format!("value isn't valid JSON: {e}")))?;
            Ok(Item::Json(path()?, parsed))
        }
        ":=@" => Ok(Item::JsonFile(path()?, value.to_string())),
        "@" => {
            let (file, content_type) = match value.rsplit_once(";type=") {
                Some((file, content_type)) => (file, Some(content_type.to_string())),
                None => (value, None),
            };
            Ok(Item::Upload {
                field: name()?,
                file: file.to_string(),
                content_type,
            })
        }
        _ => unreachable!("every separator is handled"),
    };
}

/// Finds the first unescaped separator. Returns the raw key, separator and value.
fn split(raw: &str) -> Option<(&str, &'static str, &str)> {
    let mut chars = raw.char_indices();
    while let Some((index, c)) = chars.next() {
        if c == '\\' {
            chars.next();
            continue;
        }
        let rest = &raw[index..];
        let Some(separator) = SEPARATORS.iter().find(|s| rest.starts_with(**s)) else {
            continue;
        };
        let value = &rest[separator.len()..];
        // `Name;` is the only form of `;`, so `a;b=c` keeps scanning.
        if *separator == ";" && !value.is_empty() {
            continue;
        }
        return Some((&raw[..index], separator, value));
    }
    return None;
}

fn unescape(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars();
    while let Some(c) = chars.next() {
        if c == '\\' {
            if let Some(next) = chars.next() {
                out.push(next);
                continue;
            }
        }
        out.push(c);
    }
    return out;
}

/// `name=\@text` sends a literal `@text`.
fn unescape_at(value: &str) -> String {
    return match value.strip_prefix("\\@") {
        Some(rest) => format!("@{rest}"),
        None => value.to_string(),
    };
}

/// Parses `user[tags][0]`, `a[]` and `[]` into path segments.
fn parse_path(raw: &str) -> std::result::Result<Vec<Segment>, String> {
    let mut segments = Vec::new();
    let mut chars = raw.chars().peekable();

    let mut base = String::new();
    while let Some(c) = chars.peek().copied() {
        if c == '[' {
            break;
        }
        chars.next();
        if c == '\\' {
            base.extend(chars.next());
        } else {
            base.push(c);
        }
    }
    if !base.is_empty() {
        segments.push(Segment::Key(base));
    }

    while let Some(open) = chars.next() {
        if open != '[' {
            return Err(format!("expected `[` but found `{open}`"));
        }
        let mut inner = String::new();
        let mut forced_key = false;
        loop {
            match chars.next() {
                None => return Err("expected `]`".to_string()),
                Some(']') => break,
                Some('\\') => {
                    forced_key = true;
                    inner.extend(chars.next());
                }
                Some(c) => inner.push(c),
            }
        }
        let is_number = !inner.is_empty() && inner.chars().all(|c| c.is_ascii_digit());
        let segment = if forced_key {
            Segment::Key(inner)
        } else if inner.is_empty() {
            Segment::Append
        } else if is_number {
            let index: usize = inner.parse().map_err(|_| format!("bad index `{inner}`"))?;
            if index > MAX_INDEX {
                return Err(format!("index {index} is too large"));
            }
            Segment::Index(index)
        } else {
            Segment::Key(inner)
        };
        segments.push(segment);
    }

    if segments.is_empty() {
        return Err("the name is empty".to_string());
    }
    return Ok(segments);
}

fn insert(node: &mut Value, path: &[Segment], value: Value) -> std::result::Result<(), String> {
    let Some((segment, rest)) = path.split_first() else {
        *node = value;
        return Ok(());
    };
    match segment {
        Segment::Key(key) => {
            if node.is_null() {
                *node = json!({});
            }
            let Value::Object(map) = node else {
                return Err(format!(
                    "can't set key `{key}` on a non-object; use `:=` to replace it"
                ));
            };
            let child = map.entry(key.clone()).or_insert(Value::Null);
            return insert(child, rest, value);
        }
        Segment::Index(index) => {
            let list = as_list(node)?;
            if list.len() <= *index {
                list.resize(*index + 1, Value::Null);
            }
            return insert(&mut list[*index], rest, value);
        }
        Segment::Append => {
            let list = as_list(node)?;
            list.push(Value::Null);
            let last = list.last_mut().expect("just pushed");
            return insert(last, rest, value);
        }
    }
}

fn as_list(node: &mut Value) -> std::result::Result<&mut Vec<Value>, String> {
    if node.is_null() {
        *node = json!([]);
    }
    return match node {
        Value::Array(list) => Ok(list),
        _ => Err("can't index into a non-array; use `:=` to replace it".to_string()),
    };
}

/// Applies items to `spec`. The first use of a header or query name replaces the file's values,
/// and repeats add to it. Returns uploaded files, which `prepare` allows outside the collection.
pub fn apply(spec: &mut HttpSpec, items: &[Item], mode: BodyMode) -> Result<Vec<PathBuf>> {
    let has_data = items.iter().any(Item::is_data);
    let wants_upload = items.iter().any(|i| matches!(i, Item::Upload { .. }));
    if has_data {
        prepare_body(spec, mode, wants_upload)?;
    }

    let mut seen_headers = HashSet::new();
    let mut seen_query = HashSet::new();
    let mut seen_form = HashSet::new();
    let mut uploads = Vec::new();

    for item in items {
        match item {
            Item::Header(name, value) => {
                set_pair(
                    &mut spec.headers,
                    &mut seen_headers,
                    name,
                    value.clone(),
                    true,
                );
            }
            Item::HeaderFile(name, file) => {
                let value = read_text(file)?;
                set_pair(
                    &mut spec.headers,
                    &mut seen_headers,
                    name,
                    Some(value),
                    true,
                );
            }
            Item::Query(name, value) => {
                set_pair(
                    &mut spec.query,
                    &mut seen_query,
                    name,
                    Some(value.clone()),
                    false,
                );
            }
            Item::QueryFile(name, file) => {
                let value = read_text(file)?;
                set_pair(&mut spec.query, &mut seen_query, name, Some(value), false);
            }
            Item::Field(path, value) => {
                set_field(spec, &mut seen_form, path, Value::String(value.clone()))?;
            }
            Item::FieldFile(path, file) => {
                set_field(spec, &mut seen_form, path, Value::String(read_text(file)?))?;
            }
            Item::Json(path, value) => set_json(spec, path, value.clone())?,
            Item::JsonFile(path, file) => {
                let text = read_text(file)?;
                let value = serde_json::from_str(&text)
                    .map_err(|e| Error::Invalid(format!("{file}: invalid JSON: {e}")))?;
                set_json(spec, path, value)?;
            }
            Item::Upload {
                field,
                file,
                content_type,
            } => {
                let canonical = Path::new(file)
                    .canonicalize()
                    .map_err(|e| Error::io(file, e))?;
                let part = Part::File {
                    file: canonical.display().to_string(),
                    content_type: content_type.clone(),
                };
                if let Some(Body::Multipart(parts)) = &mut spec.body {
                    parts.insert(field.clone(), part);
                }
                uploads.push(canonical);
            }
        }
    }
    return Ok(uploads);
}

/// Picks the body type data items go into, creating or converting the body if needed.
fn prepare_body(spec: &mut HttpSpec, mode: BodyMode, wants_upload: bool) -> Result<()> {
    let body = spec.body.take();
    let existing_kind = body.as_ref().map(body_kind);

    if wants_upload
        && mode == BodyMode::Auto
        && !matches!(body, Some(Body::Form(_) | Body::Multipart(_)))
    {
        return Err(Error::Invalid(
            "file uploads need --form or --multipart".to_string(),
        ));
    }

    let wants_multipart = mode == BodyMode::Multipart || wants_upload;
    let new_body = match (body, mode) {
        (Some(Body::Multipart(parts)), _) => Body::Multipart(parts),
        (Some(Body::Form(pairs)), _) if wants_multipart => Body::Multipart(form_to_parts(pairs)),
        (Some(Body::Form(pairs)), _) => Body::Form(pairs),
        (None, _) if wants_multipart => Body::Multipart(Default::default()),
        (None, BodyMode::Form) => Body::Form(Pairs::default()),
        (None, _) => Body::Json(Value::Null),
        (Some(Body::Json(value)), BodyMode::Auto) => Body::Json(value),
        _ => {
            return Err(Error::Invalid(format!(
                "can't add data items to this request's `{}` body",
                existing_kind.unwrap_or("unknown")
            )));
        }
    };
    spec.body = Some(new_body);
    return Ok(());
}

fn body_kind(body: &Body) -> &'static str {
    return match body {
        Body::Json(_) => "json",
        Body::Text(_) => "text",
        Body::Form(_) => "form",
        Body::Multipart(_) => "multipart",
        Body::File(_) => "file",
    };
}

fn form_to_parts(pairs: Pairs) -> indexmap::IndexMap<String, Part> {
    return pairs
        .0
        .into_iter()
        .filter_map(|(name, value)| value.map(|v| (name, Part::Text(v))))
        .collect();
}

fn set_pair(
    pairs: &mut Pairs,
    seen: &mut HashSet<String>,
    name: &str,
    value: Option<String>,
    ignore_case: bool,
) {
    let key = if ignore_case {
        name.to_ascii_lowercase()
    } else {
        name.to_string()
    };
    if seen.insert(key) {
        pairs.remove(name, ignore_case);
    }
    pairs.push(name, value);
}

fn set_field(
    spec: &mut HttpSpec,
    seen_form: &mut HashSet<String>,
    path: &[Segment],
    value: Value,
) -> Result<()> {
    if matches!(spec.body, Some(Body::Json(_))) {
        return set_json(spec, path, value);
    }
    let [Segment::Key(name)] = path else {
        return Err(Error::Invalid(
            "nested fields only work with JSON bodies".to_string(),
        ));
    };
    let Value::String(text) = value else {
        unreachable!("fields are always strings");
    };
    match &mut spec.body {
        Some(Body::Form(pairs)) => set_pair(pairs, seen_form, name, Some(text), false),
        Some(Body::Multipart(parts)) => {
            parts.insert(name.clone(), Part::Text(text));
        }
        _ => unreachable!("prepare_body created the body"),
    }
    return Ok(());
}

fn set_json(spec: &mut HttpSpec, path: &[Segment], value: Value) -> Result<()> {
    let Some(Body::Json(root)) = &mut spec.body else {
        return Err(Error::Invalid(
            "`:=` values only work with JSON bodies".to_string(),
        ));
    };
    return insert(root, path, value).map_err(Error::Invalid);
}

/// Reads a file named on the command line, dropping trailing newlines.
fn read_text(file: &str) -> Result<String> {
    let text = fs::read_to_string(file).map_err(|e| Error::io(file, e))?;
    return Ok(text.trim_end_matches(['\n', '\r']).to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn key(name: &str) -> Vec<Segment> {
        vec![Segment::Key(name.to_string())]
    }

    fn parse_all(raw: &[&str]) -> Vec<Item> {
        raw.iter().map(|r| parse(r).unwrap()).collect()
    }

    fn json_body(raw: &[&str]) -> Value {
        let mut spec = HttpSpec::default();
        apply(&mut spec, &parse_all(raw), BodyMode::Auto).unwrap();
        let Some(Body::Json(value)) = spec.body else {
            panic!("expected a JSON body");
        };
        return value;
    }

    #[test]
    fn parses_each_separator() {
        assert_eq!(
            parse("X-A:1").unwrap(),
            Item::Header("X-A".into(), Some("1".into()))
        );
        assert_eq!(parse("X-A:").unwrap(), Item::Header("X-A".into(), None));
        assert_eq!(
            parse("X-A;").unwrap(),
            Item::Header("X-A".into(), Some("".into()))
        );
        assert_eq!(
            parse("q==a b").unwrap(),
            Item::Query("q".into(), "a b".into())
        );
        assert_eq!(parse("a=1").unwrap(), Item::Field(key("a"), "1".into()));
        assert_eq!(parse("a:=1").unwrap(), Item::Json(key("a"), json!(1)));
        assert_eq!(
            parse("a=@f").unwrap(),
            Item::FieldFile(key("a"), "f".into())
        );
        assert_eq!(
            parse("a:=@f").unwrap(),
            Item::JsonFile(key("a"), "f".into())
        );
        assert_eq!(
            parse("a==@f").unwrap(),
            Item::QueryFile("a".into(), "f".into())
        );
        assert_eq!(
            parse("a:@f").unwrap(),
            Item::HeaderFile("a".into(), "f".into())
        );
        assert_eq!(
            parse("cv@a.pdf;type=application/pdf").unwrap(),
            Item::Upload {
                field: "cv".into(),
                file: "a.pdf".into(),
                content_type: Some("application/pdf".into()),
            }
        );
    }

    #[test]
    fn first_separator_wins_and_values_are_untouched() {
        assert_eq!(
            parse("url=http://x?a=1").unwrap(),
            Item::Field(key("url"), "http://x?a=1".into())
        );
        assert_eq!(
            parse("Authorization:Bearer a=b").unwrap(),
            Item::Header("Authorization".into(), Some("Bearer a=b".into()))
        );
    }

    #[test]
    fn escapes_keep_separators_literal() {
        assert_eq!(
            parse(r"foo\==bar").unwrap(),
            Item::Field(key("foo="), "bar".into())
        );
        assert_eq!(parse(r"a=\@b").unwrap(), Item::Field(key("a"), "@b".into()));
        assert_eq!(
            parse(r"a\[b\]:=1").unwrap(),
            Item::Json(key("a[b]"), json!(1))
        );
    }

    #[test]
    fn invalid_items_explain_themselves() {
        for raw in [
            "nothing",
            "a:=not json",
            "=x",
            "a[b=1",
            "a[b]c=1",
            "a[99999999]=1",
        ] {
            assert!(parse(raw).is_err(), "{raw}");
        }
    }

    #[test]
    fn builds_nested_json() {
        let value = json_body(&[
            "name=John",
            "age:=29",
            "user[address][city]=Lagos",
            "tags[]=a",
            "tags[]=b",
            "list[2]=x",
            r"obj[\1]=string-key",
        ]);
        assert_eq!(
            value,
            json!({
                "name": "John",
                "age": 29,
                "user": { "address": { "city": "Lagos" } },
                "tags": ["a", "b"],
                "list": [null, null, "x"],
                "obj": { "1": "string-key" },
            })
        );
    }

    #[test]
    fn builds_top_level_arrays() {
        assert_eq!(json_body(&["[]:=1", "[]:=2"]), json!([1, 2]));
    }

    #[test]
    fn type_conflicts_fail_and_values_can_be_replaced() {
        let mut spec = HttpSpec::default();
        let items = parse_all(&["a[]=x", "a[key]=y"]);
        assert!(apply(&mut spec, &items, BodyMode::Auto).is_err());
        assert_eq!(json_body(&["a:=1", "a=two"]), json!({ "a": "two" }));
    }

    #[test]
    fn items_override_the_file_but_repeats_accumulate() {
        let mut spec = HttpSpec::default();
        spec.headers.push("Accept", Some("text/plain".into()));
        spec.headers.push("X-Keep", Some("1".into()));
        spec.query.push("page", Some("1".into()));
        spec.body = Some(Body::Json(json!({ "name": "old", "keep": true })));

        let items = parse_all(&["accept:json", "page==2", "page==3", "name=new", "X-Keep:"]);
        apply(&mut spec, &items, BodyMode::Auto).unwrap();

        assert_eq!(
            spec.headers.0[0],
            ("accept".to_string(), Some("json".to_string()))
        );
        assert_eq!(spec.headers.0[1], ("X-Keep".to_string(), None));
        assert_eq!(
            spec.query.0,
            vec![
                ("page".to_string(), Some("2".to_string())),
                ("page".to_string(), Some("3".to_string())),
            ]
        );
        assert_eq!(
            spec.body,
            Some(Body::Json(json!({ "name": "new", "keep": true })))
        );
    }

    #[test]
    fn form_mode_collects_pairs_and_rejects_json_values() {
        let mut spec = HttpSpec::default();
        apply(&mut spec, &parse_all(&["a=1"]), BodyMode::Form).unwrap();
        assert!(matches!(spec.body, Some(Body::Form(_))));

        let mut spec = HttpSpec::default();
        assert!(apply(&mut spec, &parse_all(&["a:=1"]), BodyMode::Form).is_err());
        let mut spec = HttpSpec::default();
        assert!(apply(&mut spec, &parse_all(&["a[b]=1"]), BodyMode::Form).is_err());
    }

    #[test]
    fn uploads_need_a_form_mode_and_are_reported() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("a.txt");
        fs::write(&file, "hi").unwrap();
        let raw = format!("cv@{}", file.display());

        let mut spec = HttpSpec::default();
        assert!(apply(&mut spec, &parse_all(&[&raw]), BodyMode::Auto).is_err());

        let mut spec = HttpSpec::default();
        let items = parse_all(&[&raw, "title=hello"]);
        let uploads = apply(&mut spec, &items, BodyMode::Form).unwrap();
        assert_eq!(uploads, vec![file.canonicalize().unwrap()]);
        let Some(Body::Multipart(parts)) = spec.body else {
            panic!("expected multipart");
        };
        assert_eq!(parts["title"], Part::Text("hello".into()));
    }

    #[test]
    fn data_items_cant_merge_into_text_bodies() {
        let mut spec = HttpSpec {
            body: Some(Body::Text("raw".into())),
            ..HttpSpec::default()
        };
        assert!(apply(&mut spec, &parse_all(&["a=1"]), BodyMode::Auto).is_err());
    }
}
