//! Turns an `HttpSpec` into a concrete request: variables expanded, headers merged, files read.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

use nd_constants::{DEFAULT_ACCEPT, DEFAULT_MAX_REDIRECTS, USER_AGENT};
use url::Url;

use crate::error::{Error, Result};
use crate::pairs::Pairs;
use crate::request::{Body, HttpSpec, Part};
use crate::template::{expand, expand_json};
use crate::vars::Variables;

const CONTENT_TYPE: &str = "content-type";

pub struct PrepareContext<'a> {
    pub vars: &'a Variables,
    pub default_headers: Option<&'a Pairs>,
    /// Relative body files resolve against this.
    pub base_dir: PathBuf,
    /// When set, body files must be inside it.
    pub root: Option<PathBuf>,
    /// Files named on the command line, which may be outside `root`.
    pub allowed_files: &'a [PathBuf],
}

#[derive(Debug, Clone)]
pub struct PreparedRequest {
    pub method: String,
    pub url: Url,
    pub headers: Vec<(String, String)>,
    pub body: PreparedBody,
    /// `None` uses the default limit. Zero disables it.
    pub timeout: Option<Duration>,
    pub follow_redirects: bool,
    pub max_redirects: usize,
    pub verify_tls: bool,
}

#[derive(Debug, Clone, Default)]
pub enum PreparedBody {
    #[default]
    None,
    Bytes(Vec<u8>),
    Multipart(Vec<PreparedPart>),
}

#[derive(Debug, Clone)]
pub struct PreparedPart {
    pub name: String,
    pub data: Vec<u8>,
    pub filename: Option<String>,
    pub content_type: Option<String>,
}

pub fn prepare(spec: &HttpSpec, ctx: &PrepareContext) -> Result<PreparedRequest> {
    let mut url = parse_url(&expand(&spec.url, ctx.vars)?)?;
    append_query(&mut url, &spec.query, ctx.vars)?;

    let (body, implied_type) = prepare_body(spec.body.as_ref(), ctx)?;
    let method = match &spec.method {
        Some(method) => expand(method, ctx.vars)?.to_ascii_uppercase(),
        None if matches!(body, PreparedBody::None) => "GET".to_string(),
        None => "POST".to_string(),
    };

    return Ok(PreparedRequest {
        method,
        url,
        headers: merge_headers(spec, ctx, implied_type)?,
        body,
        timeout: parse_timeout(spec.timeout.as_deref(), ctx.vars)?,
        follow_redirects: spec.follow_redirects.unwrap_or(false),
        max_redirects: spec.max_redirects.unwrap_or(DEFAULT_MAX_REDIRECTS),
        verify_tls: spec.verify_tls.unwrap_or(true),
    });
}

/// `:3000/a` means localhost over http. Anything else without a scheme uses https.
fn parse_url(raw: &str) -> Result<Url> {
    let full = if let Some(rest) = raw.strip_prefix("://") {
        format!("https://{rest}")
    } else if let Some(rest) = raw.strip_prefix(':') {
        format!("http://localhost{}", normalize_local(rest))
    } else if has_scheme(raw) {
        raw.to_string()
    } else {
        format!("https://{raw}")
    };

    let url = Url::parse(&full).map_err(|e| Error::Invalid(format!("invalid URL `{raw}`: {e}")))?;
    if !matches!(url.scheme(), "http" | "https") {
        return Err(Error::Invalid(format!(
            "`{}` isn't an http or https URL",
            url.scheme()
        )));
    }
    return Ok(url);
}

/// Turns the text after `:` into a port and path: `3000/a` -> `:3000/a`, `/a` -> `/a`.
fn normalize_local(rest: &str) -> String {
    if rest.is_empty() {
        return "/".to_string();
    }
    if rest.starts_with('/') {
        return rest.to_string();
    }
    return format!(":{rest}");
}

fn has_scheme(raw: &str) -> bool {
    let Some((scheme, _)) = raw.split_once("://") else {
        return false;
    };
    let mut chars = scheme.chars();
    let starts_alpha = chars.next().is_some_and(|c| c.is_ascii_alphabetic());
    return starts_alpha
        && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '-' | '.'));
}

fn append_query(url: &mut Url, query: &Pairs, vars: &Variables) -> Result<()> {
    for (name, value) in &query.0 {
        let Some(value) = value else { continue };
        let name = expand(name, vars)?;
        let value = expand(value, vars)?;
        url.query_pairs_mut().append_pair(&name, &value);
    }
    return Ok(());
}

/// Built-in defaults, then collection defaults, then the request. Null removes a header.
fn merge_headers(
    spec: &HttpSpec,
    ctx: &PrepareContext,
    implied_type: Option<String>,
) -> Result<Vec<(String, String)>> {
    let mut merged = Pairs::default();
    merged.push("user-agent", Some(USER_AGENT.to_string()));
    merged.push("accept", Some(DEFAULT_ACCEPT.to_string()));
    if let Some(defaults) = ctx.default_headers {
        merged.overlay(defaults, true);
    }

    let names_content_type = merged
        .0
        .iter()
        .chain(spec.headers.0.iter())
        .any(|(name, _)| name.eq_ignore_ascii_case(CONTENT_TYPE));
    if let (Some(content_type), false) = (implied_type, names_content_type) {
        merged.push(CONTENT_TYPE, Some(content_type));
    }
    merged.overlay(&spec.headers, true);

    let mut headers = Vec::with_capacity(merged.0.len());
    for (name, value) in &merged.0 {
        if let Some(value) = value {
            headers.push((name.clone(), expand(value, ctx.vars)?));
        }
    }
    return Ok(headers);
}

fn prepare_body(
    body: Option<&Body>,
    ctx: &PrepareContext,
) -> Result<(PreparedBody, Option<String>)> {
    let Some(body) = body else {
        return Ok((PreparedBody::None, None));
    };
    return match body {
        Body::Json(value) => {
            let expanded = expand_json(value, ctx.vars)?;
            let bytes = serde_json::to_vec(&expanded)
                .map_err(|e| Error::Invalid(format!("can't encode JSON body: {e}")))?;
            Ok((
                PreparedBody::Bytes(bytes),
                Some("application/json".to_string()),
            ))
        }
        Body::Text(text) => {
            let bytes = expand(text, ctx.vars)?.into_bytes();
            Ok((
                PreparedBody::Bytes(bytes),
                Some("text/plain; charset=utf-8".to_string()),
            ))
        }
        Body::Form(pairs) => {
            let mut encoder = url::form_urlencoded::Serializer::new(String::new());
            for (name, value) in &pairs.0 {
                let Some(value) = value else { continue };
                encoder.append_pair(&expand(name, ctx.vars)?, &expand(value, ctx.vars)?);
            }
            let bytes = encoder.finish().into_bytes();
            Ok((
                PreparedBody::Bytes(bytes),
                Some("application/x-www-form-urlencoded; charset=utf-8".to_string()),
            ))
        }
        Body::File(path) => {
            let resolved = resolve_file(path, ctx)?;
            let bytes = fs::read(&resolved).map_err(|e| Error::io(&resolved, e))?;
            let guessed = mime_guess::from_path(&resolved)
                .first_raw()
                .map(str::to_string);
            Ok((PreparedBody::Bytes(bytes), guessed))
        }
        Body::Multipart(parts) => {
            let mut prepared = Vec::with_capacity(parts.len());
            for (name, part) in parts {
                prepared.push(prepare_part(name, part, ctx)?);
            }
            // The engine adds the content type, since it includes the boundary.
            Ok((PreparedBody::Multipart(prepared), None))
        }
    };
}

fn prepare_part(name: &str, part: &Part, ctx: &PrepareContext) -> Result<PreparedPart> {
    return match part {
        Part::Text(text) => Ok(PreparedPart {
            name: name.to_string(),
            data: expand(text, ctx.vars)?.into_bytes(),
            filename: None,
            content_type: None,
        }),
        Part::File { file, content_type } => {
            let resolved = resolve_file(file, ctx)?;
            let filename = resolved
                .file_name()
                .map(|n| n.to_string_lossy().to_string());
            let guessed = mime_guess::from_path(&resolved)
                .first_raw()
                .map(str::to_string);
            Ok(PreparedPart {
                name: name.to_string(),
                data: fs::read(&resolved).map_err(|e| Error::io(&resolved, e))?,
                filename,
                content_type: content_type.clone().or(guessed),
            })
        }
    };
}

/// Stops a request file from reading outside its collection, like `~/.ssh/id_rsa`.
fn resolve_file(raw: &str, ctx: &PrepareContext) -> Result<PathBuf> {
    let expanded = expand(raw, ctx.vars)?;
    let path = Path::new(&expanded);
    let joined = if path.is_absolute() {
        path.to_path_buf()
    } else {
        ctx.base_dir.join(path)
    };
    let canonical = joined.canonicalize().map_err(|e| Error::io(&joined, e))?;

    let Some(root) = &ctx.root else {
        return Ok(canonical);
    };
    let root = root.canonicalize().map_err(|e| Error::io(root, e))?;
    if canonical.starts_with(&root) || ctx.allowed_files.contains(&canonical) {
        return Ok(canonical);
    }
    return Err(Error::FileOutsideRoot(canonical));
}

fn parse_timeout(raw: Option<&str>, vars: &Variables) -> Result<Option<Duration>> {
    let Some(raw) = raw else {
        return Ok(None);
    };
    let text = expand(raw, vars)?;
    let text = text.trim();
    let invalid = || Error::Invalid(format!("invalid timeout `{text}`; use 500ms, 30s or 2m"));

    let (number, scale) = if let Some(number) = text.strip_suffix("ms") {
        (number, 0.001)
    } else if let Some(number) = text.strip_suffix('s') {
        (number, 1.0)
    } else if let Some(number) = text.strip_suffix('m') {
        (number, 60.0)
    } else {
        (text, 1.0)
    };
    let value: f64 = number.trim().parse().map_err(|_| invalid())?;
    let duration = Duration::try_from_secs_f64(value * scale).map_err(|_| invalid())?;
    return Ok(Some(duration));
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn vars() -> Variables {
        Variables::from_pairs(&[("HOST", "example.org"), ("NAME", "John")])
    }

    fn run(spec: &HttpSpec) -> Result<PreparedRequest> {
        let vars = vars();
        let ctx = PrepareContext {
            vars: &vars,
            default_headers: None,
            base_dir: PathBuf::from("."),
            root: None,
            allowed_files: &[],
        };
        return prepare(spec, &ctx);
    }

    fn spec(url: &str) -> HttpSpec {
        return HttpSpec {
            url: url.to_string(),
            ..HttpSpec::default()
        };
    }

    fn header<'a>(request: &'a PreparedRequest, name: &str) -> Option<&'a str> {
        return request
            .headers
            .iter()
            .find(|(n, _)| n.eq_ignore_ascii_case(name))
            .map(|(_, v)| v.as_str());
    }

    #[test]
    fn url_shortcuts() {
        let cases = [
            (":3000/foo", "http://localhost:3000/foo"),
            (":/foo", "http://localhost/foo"),
            (":", "http://localhost/"),
            ("example.org/a", "https://example.org/a"),
            ("://example.org", "https://example.org/"),
            ("http://example.org", "http://example.org/"),
            ("example.org/?r=http://x", "https://example.org/?r=http://x"),
        ];
        for (input, expected) in cases {
            assert_eq!(run(&spec(input)).unwrap().url.as_str(), expected, "{input}");
        }
        assert!(run(&spec("ftp://example.org")).is_err());
    }

    #[test]
    fn method_defaults_depend_on_body() {
        assert_eq!(run(&spec("x.org")).unwrap().method, "GET");
        let mut with_body = spec("x.org");
        with_body.body = Some(Body::Text("hi".into()));
        assert_eq!(run(&with_body).unwrap().method, "POST");
        with_body.method = Some("put".into());
        assert_eq!(run(&with_body).unwrap().method, "PUT");
    }

    #[test]
    fn expands_url_query_headers_and_body() {
        let mut request = spec("https://${HOST}/a?keep=1");
        request.query.push("who", Some("${NAME} D".into()));
        request.headers.push("x-name", Some("${NAME}".into()));
        request.body = Some(Body::Json(json!({ "name": "${NAME}", "n": 1 })));

        let prepared = run(&request).unwrap();
        assert_eq!(
            prepared.url.as_str(),
            "https://example.org/a?keep=1&who=John+D"
        );
        assert_eq!(header(&prepared, "x-name"), Some("John"));
        assert_eq!(header(&prepared, "content-type"), Some("application/json"));
        let PreparedBody::Bytes(bytes) = prepared.body else {
            panic!("expected bytes");
        };
        assert_eq!(
            String::from_utf8(bytes).unwrap(),
            r#"{"name":"John","n":1}"#
        );
    }

    #[test]
    fn header_layers_merge_and_null_removes() {
        let vars = vars();
        let mut defaults = Pairs::default();
        defaults.push("x-team", Some("core".into()));
        defaults.push("Accept", Some("text/html".into()));
        let ctx = PrepareContext {
            vars: &vars,
            default_headers: Some(&defaults),
            base_dir: PathBuf::from("."),
            root: None,
            allowed_files: &[],
        };

        let mut request = spec("x.org");
        request.headers.push("USER-AGENT", None);
        request.headers.push("x-team", Some("web".into()));
        let prepared = prepare(&request, &ctx).unwrap();

        assert_eq!(header(&prepared, "user-agent"), None);
        assert_eq!(header(&prepared, "accept"), Some("text/html"));
        assert_eq!(header(&prepared, "x-team"), Some("web"));
    }

    #[test]
    fn explicit_content_type_wins_and_can_be_removed() {
        let mut request = spec("x.org");
        request.body = Some(Body::Json(json!({})));
        request
            .headers
            .push("Content-Type", Some("application/vnd.x+json".into()));
        assert_eq!(
            header(&run(&request).unwrap(), "content-type"),
            Some("application/vnd.x+json")
        );

        let mut request = spec("x.org");
        request.body = Some(Body::Json(json!({})));
        request.headers.push("Content-Type", None);
        assert_eq!(header(&run(&request).unwrap(), "content-type"), None);
    }

    #[test]
    fn encodes_form_bodies() {
        let mut pairs = Pairs::default();
        pairs.push("a", Some("x y".into()));
        pairs.push("b", Some("${NAME}".into()));
        let mut request = spec("x.org");
        request.body = Some(Body::Form(pairs));
        let PreparedBody::Bytes(bytes) = run(&request).unwrap().body else {
            panic!("expected bytes");
        };
        assert_eq!(String::from_utf8(bytes).unwrap(), "a=x+y&b=John");
    }

    #[test]
    fn body_files_cant_leave_the_collection() {
        let outside = tempfile::tempdir().unwrap();
        let secret = outside.path().join("id_rsa");
        fs::write(&secret, "key").unwrap();
        let root = tempfile::tempdir().unwrap();
        fs::write(root.path().join("ok.json"), "{}").unwrap();

        let vars = vars();
        let allowed = [secret.canonicalize().unwrap()];
        let context = |allowed_files| PrepareContext {
            vars: &vars,
            default_headers: None,
            base_dir: root.path().to_path_buf(),
            root: Some(root.path().to_path_buf()),
            allowed_files,
        };

        let mut request = spec("x.org");
        request.body = Some(Body::File("ok.json".into()));
        let prepared = prepare(&request, &context(&[])).unwrap();
        assert_eq!(header(&prepared, "content-type"), Some("application/json"));

        request.body = Some(Body::File(secret.display().to_string()));
        assert!(matches!(
            prepare(&request, &context(&[])),
            Err(Error::FileOutsideRoot(_))
        ));
        assert!(prepare(&request, &context(&allowed)).is_ok());
    }

    #[test]
    fn parses_timeouts() {
        let vars = vars();
        let parse = |raw| parse_timeout(Some(raw), &vars).unwrap().unwrap();
        assert_eq!(parse("500ms"), Duration::from_millis(500));
        assert_eq!(parse("30s"), Duration::from_secs(30));
        assert_eq!(parse("2m"), Duration::from_secs(120));
        assert_eq!(parse("1.5"), Duration::from_millis(1500));
        assert_eq!(parse("0"), Duration::ZERO);
        assert!(parse_timeout(Some("soon"), &vars).is_err());
        assert!(parse_timeout(Some("-1"), &vars).is_err());
    }
}
