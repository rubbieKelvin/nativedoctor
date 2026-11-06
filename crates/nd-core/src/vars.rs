//! Variable and secret resolution.
//!
//! Lowest to highest priority: collection vars, environment vars, secrets file, captured state,
//! process environment, `--var`. Only declared names can be read from the process environment.

use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::Path;

use nd_constants::REDACTED;
use serde_json::{Map, Value};

use crate::collection::Collection;
use crate::error::{Error, Result};
use crate::manifest::Environment;
use crate::state;

/// Where a variable's current value came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    Collection,
    Environment,
    SecretsFile,
    Captured,
    Process,
    Override,
}

/// One row of the desktop app's variables panel.
#[derive(Debug, Clone, PartialEq)]
pub struct VarEntry {
    pub name: String,
    pub value: Option<String>,
    pub secret: bool,
    pub source: Option<Source>,
}

#[derive(Debug, Clone, Default)]
pub struct Variables {
    values: HashMap<String, String>,
    sources: HashMap<String, Source>,
    declared: BTreeSet<String>,
    secret_names: BTreeSet<String>,
    /// Set by the process environment or `--var`, so captures can't replace them.
    pinned: HashSet<String>,
    secrets_hint: Option<String>,
}

impl Variables {
    /// Resolves variables for a run. `env` is the selected environment, if any.
    pub fn load(
        collection: Option<&Collection>,
        env: Option<&str>,
        overrides: &[(String, String)],
        process_env: &dyn Fn(&str) -> Option<String>,
    ) -> Result<Variables> {
        let mut vars = Variables::default();

        if let Some(collection) = collection {
            vars.declare_pairs(&collection.manifest.vars, Source::Collection);
            if let Some(name) = env {
                let environment = collection.load_environment(name)?;
                vars.apply_environment(&environment);
                vars.load_secrets_file(&collection.secrets_path(name))?;
                let hint = collection.secrets_path(name);
                let relative = hint.strip_prefix(&collection.root).unwrap_or(&hint);
                vars.secrets_hint = Some(relative.display().to_string());
            }
            for (name, value) in state::load(&collection.state_path(env))? {
                vars.set(name, value, Source::Captured);
            }
        }

        let declared: Vec<String> = vars.declared.iter().cloned().collect();
        for name in declared {
            if let Some(value) = process_env(&name) {
                vars.pinned.insert(name.clone());
                vars.set(name, value, Source::Process);
            }
        }

        for (name, value) in overrides {
            vars.pinned.insert(name.clone());
            vars.set(name.clone(), value.clone(), Source::Override);
        }
        return Ok(vars);
    }

    #[cfg(test)]
    pub(crate) fn from_pairs(pairs: &[(&str, &str)]) -> Variables {
        let mut vars = Variables::default();
        for (name, value) in pairs {
            vars.values.insert(name.to_string(), value.to_string());
        }
        return vars;
    }

    fn set(&mut self, name: String, value: String, source: Source) {
        self.sources.insert(name.clone(), source);
        self.values.insert(name, value);
    }

    fn unset(&mut self, name: &str) {
        self.sources.remove(name);
        self.values.remove(name);
    }

    fn declare_pairs(&mut self, pairs: &crate::pairs::Pairs, source: Source) {
        for (name, value) in &pairs.0 {
            self.declared.insert(name.clone());
            match value {
                Some(value) => self.set(name.clone(), value.clone(), source),
                None => self.unset(name),
            };
        }
    }

    fn apply_environment(&mut self, environment: &Environment) {
        self.declare_pairs(&environment.vars, Source::Environment);
        for name in &environment.secrets {
            self.declared.insert(name.clone());
            self.secret_names.insert(name.clone());
            // A secret's value must never come from a committed file.
            self.unset(name);
        }
    }

    /// Reads values for declared secrets only. A missing file is fine.
    fn load_secrets_file(&mut self, path: &Path) -> Result<()> {
        if !path.exists() {
            return Ok(());
        }
        let entries = dotenvy::from_path_iter(path)
            .map_err(|e| Error::Invalid(format!("{}: {e}", path.display())))?;
        for entry in entries {
            let (name, value) =
                entry.map_err(|e| Error::Invalid(format!("{}: {e}", path.display())))?;
            if self.secret_names.contains(&name) {
                self.set(name, value, Source::SecretsFile);
            }
        }
        return Ok(());
    }

    pub fn get(&self, name: &str) -> Result<String> {
        if let Some(value) = self.values.get(name) {
            return Ok(value.clone());
        }
        if self.secret_names.contains(name) {
            return Err(Error::MissingSecret {
                name: name.to_string(),
                hint: self
                    .secrets_hint
                    .clone()
                    .unwrap_or_else(|| "the secrets file".to_string()),
            });
        }
        if self.declared.contains(name) {
            return Err(Error::EmptyVar(name.to_string()));
        }
        return Err(Error::UnknownVar(name.to_string()));
    }

    pub fn has_value(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }

    pub fn is_declared(&self, name: &str) -> bool {
        self.declared.contains(name) || self.values.contains_key(name)
    }

    pub fn is_secret(&self, name: &str) -> bool {
        self.secret_names.contains(name)
    }

    /// Stores a captured value for this run. Returns false if a pinned value kept priority.
    pub fn capture(&mut self, name: &str, value: String) -> bool {
        if self.pinned.contains(name) {
            return false;
        }
        self.set(name.to_string(), value, Source::Captured);
        return true;
    }

    /// Every known variable, sorted by name. Holds real values, so redact before showing them.
    pub fn entries(&self) -> Vec<VarEntry> {
        let mut names: BTreeSet<&String> = self.declared.iter().collect();
        names.extend(self.values.keys());
        return names
            .into_iter()
            .map(|name| VarEntry {
                name: name.clone(),
                value: self.values.get(name).cloned(),
                secret: self.secret_names.contains(name),
                source: self.sources.get(name).copied(),
            })
            .collect();
    }

    pub fn redactor(&self) -> Redactor {
        let mut secrets: Vec<String> = self
            .secret_names
            .iter()
            .filter_map(|name| self.values.get(name))
            .filter(|value| !value.is_empty())
            .cloned()
            .collect();
        // Longest first, so a secret containing another is replaced whole.
        secrets.sort_by_key(|value| std::cmp::Reverse(value.len()));
        secrets.dedup();
        return Redactor { secrets };
    }
}

/// Replaces known secret values in anything shown to the user.
#[derive(Debug, Clone, Default)]
pub struct Redactor {
    secrets: Vec<String>,
}

impl Redactor {
    pub fn redact(&self, text: &str) -> String {
        let mut out = text.to_string();
        for secret in &self.secrets {
            out = out.replace(secret, REDACTED);
        }
        return out;
    }

    pub fn redact_json(&self, value: &Value) -> Value {
        return match value {
            Value::String(text) => Value::String(self.redact(text)),
            Value::Array(items) => {
                Value::Array(items.iter().map(|item| self.redact_json(item)).collect())
            }
            Value::Object(map) => {
                let mut out = Map::new();
                for (key, item) in map {
                    out.insert(self.redact(key), self.redact_json(item));
                }
                Value::Object(out)
            }
            other => other.clone(),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn collection(env: &str) -> (tempfile::TempDir, Collection) {
        let dir = tempfile::tempdir().unwrap();
        let collection = Collection::init(dir.path(), None).unwrap();
        fs::write(dir.path().join("environments/dev.yaml"), env).unwrap();
        return (dir, collection);
    }

    fn no_env(_: &str) -> Option<String> {
        None
    }

    #[test]
    fn entries_report_where_each_value_came_from() {
        let (dir, mut collection) = collection("vars: { A: env, B: }\nsecrets: [TOKEN, MISSING]\n");
        collection.manifest.vars.push("C", Some("manifest".into()));
        let secrets = collection.secrets_path("dev");
        fs::create_dir_all(secrets.parent().unwrap()).unwrap();
        fs::write(&secrets, "TOKEN=abc\n").unwrap();
        let _keep = dir;
        let overrides = [("D".to_string(), "cli".to_string())];
        let vars = Variables::load(Some(&collection), Some("dev"), &overrides, &no_env).unwrap();

        let source = |name: &str| {
            let entries = vars.entries();
            entries.iter().find(|e| e.name == name).unwrap().source
        };
        assert_eq!(source("A"), Some(Source::Environment));
        assert_eq!(source("B"), None);
        assert_eq!(source("C"), Some(Source::Collection));
        assert_eq!(source("D"), Some(Source::Override));
        assert_eq!(source("TOKEN"), Some(Source::SecretsFile));
        assert_eq!(source("MISSING"), None);
        assert!(vars.entries().iter().find(|e| e.name == "TOKEN").unwrap().secret);
    }

    #[test]
    fn process_env_only_reaches_declared_names() {
        let (_dir, collection) = collection("vars: { BASE_URL: a }\nsecrets: [TOKEN]\n");
        let process = |name: &str| Some(format!("env-{name}"));
        let vars = Variables::load(Some(&collection), Some("dev"), &[], &process).unwrap();

        assert_eq!(vars.get("BASE_URL").unwrap(), "env-BASE_URL");
        assert_eq!(vars.get("TOKEN").unwrap(), "env-TOKEN");
        assert!(matches!(
            vars.get("AWS_SECRET_ACCESS_KEY"),
            Err(Error::UnknownVar(_))
        ));
    }

    #[test]
    fn priority_runs_from_defaults_to_command_line() {
        let (dir, mut collection) = collection("vars: { A: env, B: env, C: env }\n");
        for name in ["A", "B", "C", "D"] {
            collection.manifest.vars.push(name, Some("manifest".into()));
        }
        state::save(
            &collection.state_path(Some("dev")),
            &[("B".into(), "state".into()), ("C".into(), "state".into())],
        )
        .unwrap();
        let process = |name: &str| (name == "C").then(|| "process".to_string());
        let overrides = [("Z".to_string(), "cli".to_string())];

        let vars = Variables::load(Some(&collection), Some("dev"), &overrides, &process).unwrap();
        assert_eq!(vars.get("D").unwrap(), "manifest");
        assert_eq!(vars.get("A").unwrap(), "env");
        assert_eq!(vars.get("B").unwrap(), "state");
        assert_eq!(vars.get("C").unwrap(), "process");
        assert_eq!(vars.get("Z").unwrap(), "cli");
        drop(dir);
    }

    #[test]
    fn secrets_come_from_the_secrets_file_only_when_declared() {
        let (dir, collection) = collection("secrets: [TOKEN]\n");
        fs::create_dir_all(dir.path().join(".nativedoctor/secrets")).unwrap();
        fs::write(
            dir.path().join(".nativedoctor/secrets/dev.env"),
            "TOKEN=abc\nOTHER=nope\n",
        )
        .unwrap();
        let vars = Variables::load(Some(&collection), Some("dev"), &[], &no_env).unwrap();
        assert_eq!(vars.get("TOKEN").unwrap(), "abc");
        assert!(vars.get("OTHER").is_err());
    }

    #[test]
    fn missing_secret_says_where_to_put_it() {
        let (_dir, collection) = collection("secrets: [TOKEN]\n");
        let vars = Variables::load(Some(&collection), Some("dev"), &[], &no_env).unwrap();
        let message = vars.get("TOKEN").unwrap_err().to_string();
        assert!(message.contains("TOKEN"), "{message}");
        assert!(
            message.contains(".nativedoctor/secrets/dev.env"),
            "{message}"
        );
    }

    #[test]
    fn a_secret_in_vars_is_never_used() {
        let (_dir, collection) = collection("vars: { TOKEN: leaked }\nsecrets: [TOKEN]\n");
        let vars = Variables::load(Some(&collection), Some("dev"), &[], &no_env).unwrap();
        assert!(matches!(
            vars.get("TOKEN"),
            Err(Error::MissingSecret { .. })
        ));
    }

    #[test]
    fn captures_dont_replace_pinned_values() {
        let overrides = [("A".to_string(), "cli".to_string())];
        let mut vars = Variables::load(None, None, &overrides, &no_env).unwrap();
        assert!(!vars.capture("A", "captured".into()));
        assert_eq!(vars.get("A").unwrap(), "cli");
        assert!(vars.capture("B", "captured".into()));
        assert_eq!(vars.get("B").unwrap(), "captured");
    }

    #[test]
    fn redacts_secrets_including_captured_ones() {
        let (_dir, collection) = collection("secrets: [TOKEN, SHORT]\n");
        let mut vars = Variables::load(Some(&collection), Some("dev"), &[], &no_env).unwrap();
        assert_eq!(vars.redactor().redact("abcdef"), "abcdef");

        vars.capture("TOKEN", "abcdef".into());
        vars.capture("SHORT", "abc".into());
        vars.capture("PUBLIC", "visible".into());
        let redactor = vars.redactor();
        assert_eq!(
            redactor.redact("x abcdef y abc visible"),
            "x *** y *** visible"
        );
        assert_eq!(
            redactor.redact_json(&serde_json::json!({ "a": ["abc", 1] })),
            serde_json::json!({ "a": ["***", 1] })
        );
    }
}
