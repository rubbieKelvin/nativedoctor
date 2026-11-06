//! Edits YAML in place, so comments and formatting survive a save.
//!
//! Diffs two models rather than text, so a value the user didn't touch stays exactly as written.
//! The result is checked before it's returned: if it doesn't parse back to the new model, or it
//! lost a comment it shouldn't have, the caller gets an error and nothing has been changed.

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{Map, Value};
use yaml_edit::{AsYaml, Document, Mapping, MappingBuilder, ScalarValue, SequenceBuilder, YamlNode};

use crate::error::{Error, Result};

/// Applies the difference between `old` and `new` to `original`.
pub fn patch_yaml<T: Serialize + DeserializeOwned>(
    original: &str,
    old: &T,
    new: &T,
) -> Result<String> {
    let old_value = to_value(old)?;
    let new_value = to_value(new)?;
    if same(&old_value, &new_value) {
        return Ok(original.to_string());
    }
    let (Value::Object(old_map), Value::Object(new_map)) = (&old_value, &new_value) else {
        return Err(unpreserved("the file must be a YAML map"));
    };

    let doc = Document::from_str(original).map_err(|e| unpreserved(&e.to_string()))?;
    let root = doc
        .as_mapping()
        .ok_or_else(|| unpreserved("the file must be a YAML map"))?;

    let known = comments_of(original);
    let mut replaced = Replaced {
        known: &known,
        gone: Vec::new(),
    };
    if !patch_map(&root, old_map, new_map, &mut replaced) {
        return Err(unpreserved("top-level keys changed order"));
    }
    let output = doc.to_string();

    let parsed: T = serde_yaml_ng::from_str(&output)
        .map_err(|e| unpreserved(&format!("the edited file doesn't parse: {e}")))?;
    if !same(&to_value(&parsed)?, &new_value) {
        return Err(unpreserved("the edited file doesn't match the form"));
    }
    check_comments(&known, &output, &replaced.gone)?;
    return Ok(output);
}

/// Saves `model` to `path`, keeping the existing file's comments and formatting.
/// Writes nothing and returns `Error::Unpreserved` if that isn't possible.
pub fn save_model<T: Serialize + DeserializeOwned>(path: &Path, model: &T) -> Result<()> {
    if !path.exists() {
        return rewrite_model(path, model);
    }
    let original = fs::read_to_string(path).map_err(|e| Error::io(path, e))?;
    let old: T = serde_yaml_ng::from_str(&original)
        .map_err(|e| unpreserved(&format!("the file on disk doesn't parse: {e}")))?;
    let output = patch_yaml(&original, &old, model)?;
    if output == original {
        return Ok(());
    }
    return write_atomic(path, output.as_bytes());
}

/// Replaces the whole file, losing its comments. Only for when the user has agreed to that.
pub fn rewrite_model<T: Serialize>(path: &Path, model: &T) -> Result<()> {
    let text = serde_yaml_ng::to_string(model).map_err(|source| Error::Yaml {
        path: path.to_path_buf(),
        source,
    })?;
    return write_atomic(path, text.as_bytes());
}

/// Writes through a temporary file, so a failure can't leave half a file. Keeps its permissions.
pub fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| Error::Invalid(format!("{} has no parent", path.display())))?;
    fs::create_dir_all(parent).map_err(|e| Error::io(parent, e))?;
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let temp = parent.join(format!(".{name}.tmp"));
    fs::write(&temp, bytes).map_err(|e| Error::io(&temp, e))?;
    if let Ok(metadata) = fs::metadata(path) {
        fs::set_permissions(&temp, metadata.permissions()).map_err(|e| Error::io(&temp, e))?;
    }
    return fs::rename(&temp, path).map_err(|e| Error::io(path, e));
}

fn to_value<T: Serialize>(model: &T) -> Result<Value> {
    return serde_json::to_value(model).map_err(|e| Error::Invalid(e.to_string()));
}

fn unpreserved(reason: &str) -> Error {
    return Error::Unpreserved(reason.to_string());
}

/// Equal, including key order. `Value`'s own `==` ignores order, which matters for headers.
fn same(a: &Value, b: &Value) -> bool {
    let (a, b) = (a.to_string(), b.to_string());
    return a == b;
}

/// Comments that leave with an entry being replaced or removed.
struct Replaced<'a> {
    known: &'a [String],
    gone: Vec<String>,
}

impl Replaced<'_> {
    fn record(&mut self, map: &Mapping, key: &str) {
        let Some(entry) = map.find_entry_by_key(key) else {
            return;
        };
        let text = entry.to_string();
        let inside = self.known.iter().filter(|comment| text.contains(*comment));
        self.gone.extend(inside.cloned());
    }
}

/// Patches one mapping. Returns false, having changed nothing, if it has to be replaced whole.
fn patch_map(
    map: &Mapping,
    old: &Map<String, Value>,
    new: &Map<String, Value>,
    replaced: &mut Replaced,
) -> bool {
    let kept_old = old.keys().filter(|key| new.contains_key(*key));
    let kept_new = new.keys().filter(|key| old.contains_key(*key));
    if !kept_old.eq(kept_new) {
        return false;
    }

    for key in old.keys().filter(|key| !new.contains_key(*key)) {
        replaced.record(map, key);
        map.remove(key.as_str());
    }

    let keys: Vec<&String> = new.keys().collect();
    for (index, (key, value)) in new.iter().enumerate() {
        let Some(previous) = old.get(key) else {
            let after = index.checked_sub(1).map(|i| keys[i].as_str());
            let before = keys.get(index + 1).map(|k| k.as_str());
            write(map, key, value, &Place::Insert { after, before });
            continue;
        };
        if same(previous, value) {
            continue;
        }
        if let (Value::Object(a), Value::Object(b)) = (previous, value) {
            if let (false, Some(child)) = (b.is_empty(), map.get_mapping(key.as_str())) {
                if patch_map(&child, a, b, replaced) {
                    continue;
                }
            }
        }
        replaced.record(map, key);
        write(map, key, value, &Place::Replace);
    }
    return true;
}

enum Place<'a> {
    Replace,
    Insert {
        after: Option<&'a str>,
        before: Option<&'a str>,
    },
}

fn write(map: &Mapping, key: &str, value: &Value, place: &Place) {
    match built_collection(value) {
        Some(node) => put(map, key, node, place),
        None => put(map, key, scalar(value), place),
    }
}

fn put(map: &Mapping, key: &str, value: impl AsYaml, place: &Place) {
    if let Place::Insert { after, before } = place {
        if let Some(after) = after {
            if map.contains_key(*after) && map.insert_after(*after, key, &value) {
                return;
            }
        }
        if let Some(before) = before {
            if map.contains_key(*before) && map.insert_before(*before, key, &value) {
                return;
            }
        }
    }
    map.set(key, value);
}

fn scalar(value: &Value) -> ScalarValue {
    return match value {
        Value::Null => ScalarValue::null(),
        Value::Bool(flag) => ScalarValue::parse(flag.to_string()),
        Value::Number(number) => ScalarValue::parse(number.to_string()),
        Value::String(text) => ScalarValue::string(text.clone()),
        Value::Array(_) | Value::Object(_) => unreachable!("collections are built separately"),
    };
}

/// A block mapping or sequence, keeping key order. Empty ones are written in flow style.
fn built_collection(value: &Value) -> Option<YamlNode> {
    let doc = match value {
        Value::Object(map) if map.is_empty() => return parsed_node("{}"),
        Value::Array(items) if items.is_empty() => return parsed_node("[]"),
        Value::Object(map) => build_map(MappingBuilder::new(), map).build_document(),
        Value::Array(items) => build_list(SequenceBuilder::new(), items).build_document(),
        _ => return None,
    };
    return node_of(&doc);
}

fn parsed_node(text: &str) -> Option<YamlNode> {
    return node_of(&Document::from_str(text).ok()?);
}

fn node_of(doc: &Document) -> Option<YamlNode> {
    if let Some(mapping) = doc.as_mapping() {
        return Some(YamlNode::Mapping(mapping));
    }
    return doc.as_sequence().map(YamlNode::Sequence);
}

fn build_map(mut builder: MappingBuilder, map: &Map<String, Value>) -> MappingBuilder {
    for (key, value) in map {
        builder = match value {
            Value::Object(child) if !child.is_empty() => {
                builder.mapping(key.clone(), |inner| build_map(inner, child))
            }
            Value::Array(items) if !items.is_empty() => {
                builder.sequence(key.clone(), |inner| build_list(inner, items))
            }
            Value::Object(_) | Value::Array(_) => {
                let empty = built_collection(value).expect("empty collections parse");
                builder.pair(key.clone(), empty)
            }
            other => builder.pair(key.clone(), scalar(other)),
        };
    }
    return builder;
}

fn build_list(mut builder: SequenceBuilder, items: &[Value]) -> SequenceBuilder {
    for item in items {
        builder = match item {
            Value::Object(child) if !child.is_empty() => {
                builder.mapping(|inner| build_map(inner, child))
            }
            Value::Array(nested) if !nested.is_empty() => {
                builder.sequence(|inner| build_list(inner, nested))
            }
            Value::Object(_) | Value::Array(_) => {
                builder.item(built_collection(item).expect("empty collections parse"))
            }
            other => builder.item(scalar(other)),
        };
    }
    return builder;
}

fn comments_of(text: &str) -> Vec<String> {
    let Ok(doc) = Document::from_str(text) else {
        return Vec::new();
    };
    return doc.comments().map(|c| c.text().to_string()).collect();
}

fn check_comments(known: &[String], output: &str, replaced: &[String]) -> Result<()> {
    let mut remaining: HashMap<String, usize> = HashMap::new();
    for comment in comments_of(output) {
        *remaining.entry(comment).or_default() += 1;
    }
    let mut allowed: HashMap<&str, usize> = HashMap::new();
    for comment in replaced {
        *allowed.entry(comment).or_default() += 1;
    }
    for comment in known {
        if let Some(count) = remaining.get_mut(comment).filter(|count| **count > 0) {
            *count -= 1;
            continue;
        }
        if let Some(count) = allowed.get_mut(comment.as_str()).filter(|count| **count > 0) {
            *count -= 1;
            continue;
        }
        return Err(unpreserved(&format!("the comment `{comment}` would be lost")));
    }
    return Ok(());
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{Body, RequestFile};

    const FILE: &str = "# top\nname: Log in # trailing\nhttp:\n  # above url\n  url: http://a # url note\n  query: { page: 2 } # flow\n  headers:\n    # auth\n    x-debug: true # keep a\n    x-b: 2 # keep b\n  # above timeout\n  timeout: 5s\n";

    fn parse(text: &str) -> RequestFile {
        serde_yaml_ng::from_str(text).unwrap()
    }

    /// Applies `edit` to the parsed `text` and returns the patched YAML.
    fn edited(text: &str, edit: impl Fn(&mut RequestFile)) -> String {
        let old = parse(text);
        let mut new = old.clone();
        edit(&mut new);
        let output = patch_yaml(text, &old, &new).unwrap();
        let again = parse(&output);
        assert_eq!(
            to_value(&again).unwrap().to_string(),
            to_value(&new).unwrap().to_string(),
            "{output}"
        );
        return output;
    }

    #[test]
    fn no_change_returns_the_file_untouched() {
        assert_eq!(edited(FILE, |_| {}), FILE);
    }

    #[test]
    fn changing_a_value_changes_only_that_line() {
        let output = edited(FILE, |file| file.http.url = "http://b".to_string());
        assert_eq!(output, FILE.replace("http://a", "http://b"));
    }

    #[test]
    fn untouched_values_keep_their_spelling() {
        let output = edited(FILE, |file| file.name = Some("New".to_string()));
        assert!(output.contains("x-debug: true # keep a"), "{output}");
        assert!(output.contains("query: { page: 2 } # flow"), "{output}");
    }

    #[test]
    fn adding_and_removing_keeps_other_comments() {
        let output = edited(FILE, |file| {
            file.http.follow_redirects = Some(true);
            file.http.headers.push("x-c", Some("3".to_string()));
            file.http.headers.remove("x-b", true);
        });
        for comment in ["# top", "# trailing", "# url note", "# auth", "# keep a"] {
            assert!(output.contains(comment), "{comment} in {output}");
        }
        assert!(!output.contains("x-b"), "{output}");
    }

    #[test]
    fn a_new_first_field_keeps_comments_valid() {
        let output = edited(FILE, |file| file.http.method = Some("POST".to_string()));
        assert!(output.contains("method: POST"), "{output}");
        assert!(output.contains("# above url"), "{output}");
    }

    #[test]
    fn edits_inside_flow_maps() {
        let output = edited(FILE, |file| file.http.query.push("tag", Some("a".to_string())));
        assert!(output.contains("page: 2"), "{output}");
        assert!(output.contains("# flow"), "{output}");
    }

    #[test]
    fn writes_nested_bodies_in_order_with_real_numbers() {
        let output = edited(FILE, |file| {
            file.http.body = Some(Body::Json(serde_json::json!({
                "z": 1, "a": { "y": [true, null, "s"], "b": 2.5 }, "e": {}, "l": []
            })));
        });
        assert!(!output.contains("serde_json"), "{output}");
        assert!(output.find("z:").unwrap() < output.find("a:").unwrap(), "{output}");
    }

    #[test]
    fn scalars_that_look_like_other_types_stay_strings() {
        for value in ["true", "42", "null", "a: b # c", "", "one\ntwo", "${BASE_URL}/x"] {
            let output = edited(FILE, |file| file.name = Some(value.to_string()));
            assert_eq!(parse(&output).name.as_deref(), Some(value), "{output}");
        }
    }

    #[test]
    fn repeated_names_round_trip() {
        let output = edited(FILE, |file| {
            file.http.query.push("tag", Some("a".to_string()));
            file.http.query.push("tag", Some("b".to_string()));
        });
        assert_eq!(parse(&output).http.query.0.len(), 3, "{output}");
    }

    #[test]
    fn reordering_pairs_replaces_only_that_map() {
        let output = edited(FILE, |file| file.http.headers.0.reverse());
        assert!(output.contains("# url note"), "{output}");
        assert!(output.contains("# above timeout"), "{output}");
    }

    #[test]
    fn keeps_crlf_line_endings() {
        let text = "# top\r\nhttp:\r\n  url: a # u\r\n  method: GET\r\n";
        let output = edited(text, |file| file.http.timeout = Some("5s".to_string()));
        assert!(!output.replace("\r\n", "").contains('\n'), "{output:?}");
    }

    #[test]
    fn a_value_it_cant_reproduce_is_refused_not_written() {
        let text = "base: &b { url: http://a }\nhttp: *b\n";
        #[derive(Clone, Serialize, serde::Deserialize)]
        struct Loose {
            http: serde_json::Value,
            base: serde_json::Value,
        }
        let old: Loose = serde_yaml_ng::from_str(text).unwrap();
        let mut new = old.clone();
        new.http["url"] = Value::String("http://b".to_string());
        if let Ok(output) = patch_yaml(text, &old, &new) {
            let parsed: Loose = serde_yaml_ng::from_str(&output).unwrap();
            assert_eq!(parsed.http["url"], "http://b", "{output}");
        }
    }

    #[test]
    fn save_model_keeps_comments_and_a_refusal_writes_nothing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("a.request.yaml");
        fs::write(&path, FILE).unwrap();
        let mut file = parse(FILE);
        file.http.url = "http://b".to_string();
        save_model(&path, &file).unwrap();
        assert_eq!(fs::read_to_string(&path).unwrap(), FILE.replace("http://a", "http://b"));

        fs::write(&path, "- not a map\n").unwrap();
        assert!(save_model(&path, &file).is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), "- not a map\n");
    }

    #[test]
    fn a_broken_file_is_refused() {
        let old = RequestFile::default();
        let mut new = old.clone();
        new.name = Some("x".to_string());
        assert!(matches!(
            patch_yaml("- not\n- a map\n", &old, &new),
            Err(Error::Unpreserved(_))
        ));
    }
}
