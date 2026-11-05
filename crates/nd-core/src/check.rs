//! Offline validation behind `nativedoctor check`.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use crate::collection::Collection;
use crate::evaluate::{parse_path, validate_capture};
use crate::request::RequestFile;
use crate::template::{self, Reference};
use crate::vars::Variables;

#[derive(Debug, Default)]
pub struct Report {
    pub checked: usize,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn check_collection(collection: &Collection, vars: &Variables) -> crate::Result<Report> {
    let mut report = Report::default();
    let paths = collection.requests()?;

    let mut loaded: Vec<(PathBuf, RequestFile)> = Vec::new();
    for path in paths {
        report.checked += 1;
        match RequestFile::load(&path) {
            Ok(file) => loaded.push((path, file)),
            Err(error) => report.errors.push(error.to_string()),
        }
    }

    // Any request may capture a value for a later one, so captures count as declared.
    let captured: BTreeSet<&str> = loaded
        .iter()
        .flat_map(|(_, file)| file.capture.keys().map(String::as_str))
        .collect();

    for (path, file) in &loaded {
        let relative = path.strip_prefix(&collection.root).unwrap_or(path);
        check_request(relative, file, vars, &captured, &mut report);
    }
    return Ok(report);
}

fn check_request(
    path: &Path,
    file: &RequestFile,
    vars: &Variables,
    captured: &BTreeSet<&str>,
    report: &mut Report,
) {
    let label = path.display();

    for source in file.capture.values() {
        if let Err(error) = validate_capture(source) {
            report.errors.push(format!("{label}: {error}"));
        }
    }
    for json_path in file.assertions.body.keys() {
        if let Err(error) = parse_path(json_path) {
            report.errors.push(format!("{label}: {error}"));
        }
    }

    let mut seen = BTreeSet::new();
    for reference in collect_references(file) {
        let name = match reference {
            Reference::Dynamic(name) => {
                if crate::dynamic::invoke(&name).is_err() {
                    report
                        .errors
                        .push(format!("{label}: unknown dynamic variable `${{!{name}}}`"));
                }
                continue;
            }
            Reference::Var(name) => name,
        };
        if !seen.insert(name.clone()) || vars.has_value(&name) || captured.contains(name.as_str()) {
            continue;
        }
        if vars.is_secret(&name) {
            report
                .warnings
                .push(format!("{label}: secret `{name}` has no value here"));
        } else if vars.is_declared(&name) {
            report
                .warnings
                .push(format!("{label}: variable `{name}` has no value here"));
        } else {
            report
                .errors
                .push(format!("{label}: variable `{name}` isn't declared"));
        }
    }
}

fn collect_references(file: &RequestFile) -> Vec<Reference> {
    let mut references = Vec::new();
    if let Ok(http) = serde_json::to_value(&file.http) {
        template::references_in_json(&http, &mut references);
    }
    if let Ok(assertions) = serde_json::to_value(&file.assertions) {
        template::references_in_json(&assertions, &mut references);
    }
    return references;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn no_env(_: &str) -> Option<String> {
        None
    }

    fn setup(env: &str, requests: &[(&str, &str)]) -> (tempfile::TempDir, Report) {
        let dir = tempfile::tempdir().unwrap();
        let collection = Collection::init(dir.path(), None).unwrap();
        fs::write(dir.path().join("environments/dev.yaml"), env).unwrap();
        fs::remove_file(dir.path().join("ping.request.yaml")).unwrap();
        for (name, text) in requests {
            fs::write(dir.path().join(name), text).unwrap();
        }
        let vars = Variables::load(Some(&collection), Some("dev"), &[], &no_env).unwrap();
        let report = check_collection(&collection, &vars).unwrap();
        return (dir, report);
    }

    #[test]
    fn clean_collection_passes() {
        let (_dir, report) = setup(
            "vars: { BASE: http://x }\n",
            &[("a.request.yaml", "http: { url: '${BASE}/${!uuidv4}' }\n")],
        );
        assert_eq!(report.checked, 1);
        assert!(report.errors.is_empty(), "{:?}", report.errors);
        assert!(report.warnings.is_empty(), "{:?}", report.warnings);
    }

    #[test]
    fn captured_values_count_as_declared() {
        let (_dir, report) = setup(
            "secrets: [TOKEN]\n",
            &[
                (
                    "a.request.yaml",
                    "http: { url: 'http://x' }\ncapture: { TOKEN: $.t }\n",
                ),
                (
                    "b.request.yaml",
                    "http: { url: 'http://x', headers: { a: '${TOKEN}' } }\n",
                ),
            ],
        );
        assert!(report.errors.is_empty(), "{:?}", report.errors);
        assert!(report.warnings.is_empty(), "{:?}", report.warnings);
    }

    #[test]
    fn reports_each_kind_of_problem() {
        let (_dir, report) = setup(
            "secrets: [TOKEN]\n",
            &[
                ("broken.request.yaml", "http: { metod: GET }\n"),
                (
                    "c.request.yaml",
                    "http: { url: 'http://${NOPE}/${!nope}', headers: { a: '${TOKEN}' } }\n\
                     capture: { X: 'not a path' }\nassert: { body: { '$[': 1 } }\n",
                ),
            ],
        );
        assert_eq!(report.checked, 2);
        assert_eq!(report.errors.len(), 5, "{:?}", report.errors);
        assert_eq!(report.warnings.len(), 1, "{:?}", report.warnings);
        assert!(report.warnings[0].contains("TOKEN"));
    }
}
