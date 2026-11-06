//! The rules of a run, shared by the CLI and the desktop app: which environment and variables
//! apply, where cookies live, and what a response does to them.

use std::path::PathBuf;
use std::sync::Arc;

use nd_core::evaluate::{self, ResponseView};
use nd_core::{
    prepare, state, Collection, HttpSpec, PrepareContext, PreparedRequest, RequestFile, Variables,
};

use crate::cookies::CookieJar;
use crate::event::Error;

#[derive(Debug, Clone)]
pub struct ResponseHead {
    pub status: u16,
    pub reason: Option<String>,
    pub version: String,
    pub headers: Vec<(String, String)>,
}

/// What a complete response did.
#[derive(Debug, Default)]
pub struct Conclusion {
    /// One message per failed assertion.
    pub failures: Vec<String>,
    /// Captured values to save.
    pub saved: Vec<(String, String)>,
    /// Captures dropped because an override or the process environment set that name.
    pub ignored: Vec<String>,
    pub capture_error: Option<nd_core::Error>,
}

/// Where a request's files are looked up.
#[derive(Debug, Clone, Default)]
pub struct FileScope {
    /// Relative body files resolve against this.
    pub base_dir: PathBuf,
    /// When set, body files must be inside it.
    pub root: Option<PathBuf>,
    /// Files the user named directly, which may be outside `root`.
    pub allowed: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct Session {
    pub collection: Option<Collection>,
    pub env: Option<String>,
    pub vars: Variables,
    jar: Option<Arc<CookieJar>>,
    jar_path: Option<PathBuf>,
}

impl Session {
    /// Resolves the environment, variables and cookie jar for a run.
    pub fn open(
        collection: Option<Collection>,
        requested_env: Option<&str>,
        overrides: &[(String, String)],
        use_cookies: bool,
        process_env: &dyn Fn(&str) -> Option<String>,
    ) -> Result<Session, Error> {
        let env = select_env(collection.as_ref(), requested_env)?;
        let vars = Variables::load(collection.as_ref(), env.as_deref(), overrides, process_env)?;

        let jar_path = match (&collection, use_cookies) {
            (Some(collection), true) => Some(collection.cookies_path(env.as_deref())),
            _ => None,
        };
        let jar = match (&jar_path, use_cookies) {
            (Some(path), _) => Some(Arc::new(CookieJar::load(path)?)),
            (None, true) => Some(Arc::new(CookieJar::new())),
            (None, false) => None,
        };
        return Ok(Session {
            collection,
            env,
            vars,
            jar,
            jar_path,
        });
    }

    pub fn jar(&self) -> Option<Arc<CookieJar>> {
        self.jar.clone()
    }

    /// Uses a jar another running request already holds, so neither overwrites the other.
    pub fn share_jar(&mut self, jar: Arc<CookieJar>) {
        self.jar = Some(jar);
    }

    pub fn prepare(&self, spec: &HttpSpec, files: &FileScope) -> nd_core::Result<PreparedRequest> {
        let ctx = PrepareContext {
            vars: &self.vars,
            default_headers: self.collection.as_ref().map(|c| &c.manifest.defaults.headers),
            base_dir: files.base_dir.clone(),
            root: files.root.clone(),
            allowed_files: &files.allowed,
        };
        return prepare(spec, &ctx);
    }

    /// Saved cookies for this request, values hidden. An explicit `Cookie` header replaces them.
    pub fn masked_cookies(&self, request: &PreparedRequest) -> Option<String> {
        let explicit = request
            .headers
            .iter()
            .any(|(name, _)| name.eq_ignore_ascii_case("cookie"));
        if explicit {
            return None;
        }
        return self
            .jar
            .as_ref()
            .and_then(|jar| jar.masked_header(&request.url));
    }

    pub fn save_cookies(&self) -> Result<(), Error> {
        if let (Some(jar), Some(path)) = (&self.jar, &self.jar_path) {
            jar.save(path)?;
        }
        return Ok(());
    }

    /// Checks assertions, then applies captures if they all passed. Captured secrets are stored
    /// here, so build the redactor afterwards to hide them.
    pub fn conclude(
        &mut self,
        file: &RequestFile,
        head: &ResponseHead,
        body: &[u8],
    ) -> nd_core::Result<Conclusion> {
        let view = ResponseView {
            status: head.status,
            headers: &head.headers,
            body,
        };
        let failures = evaluate::check(&file.assertions, &view, &self.vars)?;
        let mut conclusion = Conclusion {
            failures,
            ..Conclusion::default()
        };
        if !conclusion.failures.is_empty() {
            return Ok(conclusion);
        }
        match evaluate::capture(&file.capture, &view) {
            Ok(values) => {
                for (name, value) in values {
                    if self.vars.capture(&name, value.clone()) {
                        conclusion.saved.push((name, value));
                    } else {
                        conclusion.ignored.push(name);
                    }
                }
            }
            Err(error) => conclusion.capture_error = Some(error),
        }
        return Ok(conclusion);
    }

    /// Writes captured values to the environment's state file. Outside a collection they're
    /// only kept for this run.
    pub fn save_captures(&self, saved: &[(String, String)]) -> nd_core::Result<()> {
        if let (Some(collection), false) = (&self.collection, saved.is_empty()) {
            state::save(&collection.state_path(self.env.as_deref()), saved)?;
        }
        return Ok(());
    }
}

fn select_env(collection: Option<&Collection>, requested: Option<&str>) -> Result<Option<String>, Error> {
    let Some(collection) = collection else {
        if requested.is_some() {
            return Err(Error::Other(
                "--env needs a collection, and none was found".to_string(),
            ));
        }
        return Ok(None);
    };
    return Ok(requested
        .map(str::to_string)
        .or_else(|| collection.manifest.default_env.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn no_env(_: &str) -> Option<String> {
        None
    }

    fn head() -> ResponseHead {
        return ResponseHead {
            status: 200,
            reason: None,
            version: "HTTP/1.1".to_string(),
            headers: Vec::new(),
        };
    }

    #[test]
    fn captures_only_apply_when_assertions_pass() {
        let dir = tempfile::tempdir().unwrap();
        let collection = Collection::init(dir.path(), None).unwrap();
        let mut session = Session::open(Some(collection), None, &[], true, &no_env).unwrap();

        let mut file = RequestFile::default();
        file.capture.insert("ID".to_string(), "$.id".to_string());
        file.assertions.status = vec![201];
        let body = br#"{"id":7}"#;

        let failed = session.conclude(&file, &head(), body).unwrap();
        assert_eq!(failed.failures.len(), 1);
        assert!(failed.saved.is_empty());

        file.assertions.status = vec![200];
        let passed = session.conclude(&file, &head(), body).unwrap();
        assert_eq!(passed.saved, vec![("ID".to_string(), "7".to_string())]);
        assert_eq!(session.vars.get("ID").unwrap(), "7");

        session.save_captures(&passed.saved).unwrap();
        let reopened = Collection::open(dir.path()).unwrap();
        let session = Session::open(Some(reopened), None, &[], true, &no_env).unwrap();
        assert_eq!(session.vars.get("ID").unwrap(), "7");
    }

    #[test]
    fn overrides_win_over_captures() {
        let overrides = [("ID".to_string(), "pinned".to_string())];
        let mut session = Session::open(None, None, &overrides, true, &no_env).unwrap();
        let mut file = RequestFile::default();
        file.capture.insert("ID".to_string(), "$.id".to_string());

        let conclusion = session.conclude(&file, &head(), br#"{"id":7}"#).unwrap();
        assert_eq!(conclusion.ignored, vec!["ID"]);
        assert_eq!(session.vars.get("ID").unwrap(), "pinned");
    }

    #[test]
    fn a_bad_capture_is_reported_not_raised() {
        let mut session = Session::open(None, None, &[], true, &no_env).unwrap();
        let mut file = RequestFile::default();
        file.capture.insert("ID".to_string(), "$.nope".to_string());
        let conclusion = session.conclude(&file, &head(), b"{}").unwrap();
        assert!(conclusion.capture_error.is_some());
    }

    #[test]
    fn cookies_can_be_turned_off() {
        let session = Session::open(None, None, &[], false, &no_env).unwrap();
        assert!(session.jar().is_none());
        let session = Session::open(None, None, &[], true, &no_env).unwrap();
        assert!(session.jar().is_some());
    }

    #[test]
    fn an_environment_needs_a_collection() {
        assert!(Session::open(None, Some("dev"), &[], true, &no_env).is_err());
    }
}
