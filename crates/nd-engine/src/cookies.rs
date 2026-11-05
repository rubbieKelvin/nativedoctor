//! A cookie jar that can be saved between runs.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::RwLock;

use cookie_store::{CookieStore, RawCookie};
use nd_constants::REDACTED;
use reqwest::header::HeaderValue;
use reqwest::Url;

use crate::event::Error;

#[derive(Debug, Default)]
pub struct CookieJar {
    store: RwLock<CookieStore>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CookieInfo {
    pub domain: String,
    pub path: String,
    pub name: String,
    /// Has no expiry of its own. Kept until the server removes it or the jar is cleared.
    pub session: bool,
}

impl CookieJar {
    pub fn new() -> Self {
        Self::default()
    }

    /// Loads a jar. A missing file gives an empty one.
    pub fn load(path: &Path) -> Result<CookieJar, Error> {
        if !path.exists() {
            return Ok(CookieJar::new());
        }
        let invalid = |e: &dyn std::fmt::Display| {
            Error::Other(format!("can't read cookies from {}: {e}", path.display()))
        };
        let file = File::open(path).map_err(|e| invalid(&e))?;
        let store =
            cookie_store::serde::json::load_all(BufReader::new(file)).map_err(|e| invalid(&e))?;
        return Ok(CookieJar {
            store: RwLock::new(store),
        });
    }

    /// Saves unexpired cookies, including session cookies, so logins survive between runs.
    pub fn save(&self, path: &Path) -> Result<(), Error> {
        let store = self.store.read().expect("cookie lock");
        let live = store
            .iter_unexpired()
            .cloned()
            .map(Ok::<_, std::convert::Infallible>);
        let live = CookieStore::from_cookies(live, false).expect("infallible");
        if live.iter_any().next().is_none() && !path.exists() {
            return Ok(());
        }

        let mut bytes = Vec::new();
        cookie_store::serde::json::save_incl_expired_and_nonpersistent(&live, &mut bytes)
            .map_err(|e| Error::Other(format!("can't encode cookies: {e}")))?;
        return nd_core::state::write_private(path, &bytes)
            .map_err(|e| Error::Other(e.to_string()));
    }

    /// What this jar would send to `url`, with values hidden.
    pub fn masked_header(&self, url: &Url) -> Option<String> {
        let store = self.store.read().expect("cookie lock");
        let parts: Vec<String> = store
            .get_request_values(url)
            .map(|(name, _)| format!("{name}={REDACTED}"))
            .collect();
        if parts.is_empty() {
            return None;
        }
        return Some(parts.join("; "));
    }

    pub fn list(&self) -> Vec<CookieInfo> {
        let store = self.store.read().expect("cookie lock");
        return store
            .iter_unexpired()
            .map(|cookie| CookieInfo {
                domain: cookie.domain.as_cow().unwrap_or_default().to_string(),
                path: cookie.path.to_string(),
                name: cookie.name().to_string(),
                session: !cookie.is_persistent(),
            })
            .collect();
    }
}

impl reqwest::cookie::CookieStore for CookieJar {
    fn set_cookies(&self, headers: &mut dyn Iterator<Item = &HeaderValue>, url: &Url) {
        let accepted = headers.filter_map(|header| accept(header, url));
        self.store
            .write()
            .expect("cookie lock")
            .store_response_cookies(accepted, url);
    }

    fn cookies(&self, url: &Url) -> Option<HeaderValue> {
        let store = self.store.read().expect("cookie lock");
        let parts: Vec<String> = store
            .get_request_values(url)
            .map(|(name, value)| format!("{name}={value}"))
            .collect();
        if parts.is_empty() {
            return None;
        }
        return HeaderValue::from_str(&parts.join("; ")).ok();
    }
}

/// Parses a cookie and applies the browser rule for public suffixes, so a server can't set
/// `Domain=com` and reach every `.com` host.
fn accept(header: &HeaderValue, url: &Url) -> Option<RawCookie<'static>> {
    let text = std::str::from_utf8(header.as_bytes()).ok()?;
    let mut cookie = RawCookie::parse(text).ok()?.into_owned();

    let Some(domain) = cookie.domain().map(normalize_domain) else {
        return Some(cookie);
    };
    if psl::suffix_str(&domain) != Some(domain.as_str()) {
        return Some(cookie);
    }
    let same_host = url
        .host_str()
        .is_some_and(|host| host.eq_ignore_ascii_case(&domain));
    if same_host {
        cookie.unset_domain();
        return Some(cookie);
    }
    return None;
}

fn normalize_domain(domain: &str) -> String {
    domain.trim_start_matches('.').to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::cookie::CookieStore as _;

    fn url(raw: &str) -> Url {
        raw.parse().unwrap()
    }

    fn receive(jar: &CookieJar, from: &str, set_cookie: &str) {
        let header = HeaderValue::from_str(set_cookie).unwrap();
        jar.set_cookies(&mut std::iter::once(&header), &url(from));
    }

    fn sent(jar: &CookieJar, to: &str) -> Option<String> {
        return jar
            .cookies(&url(to))
            .map(|value| value.to_str().unwrap().to_string());
    }

    #[test]
    fn cookies_stay_on_the_host_that_set_them() {
        let jar = CookieJar::new();
        receive(&jar, "https://api.example.com/", "s=1");
        assert_eq!(
            sent(&jar, "https://api.example.com/x").as_deref(),
            Some("s=1")
        );
        assert_eq!(sent(&jar, "https://other.example.com/"), None);
        assert_eq!(sent(&jar, "https://evil.com/"), None);
    }

    #[test]
    fn a_domain_attribute_shares_across_subdomains_only() {
        let jar = CookieJar::new();
        receive(&jar, "https://auth.example.com/", "s=1; Domain=example.com");
        assert_eq!(
            sent(&jar, "https://api.example.com/").as_deref(),
            Some("s=1")
        );
        assert_eq!(sent(&jar, "https://notexample.com/"), None);
    }

    #[test]
    fn public_suffixes_are_rejected() {
        let jar = CookieJar::new();
        receive(&jar, "https://evil.com/", "s=1; Domain=com");
        receive(&jar, "https://evil.co.uk/", "s=2; Domain=co.uk");
        receive(&jar, "https://evil.com/", "s=3; Domain=.COM");
        assert!(jar.list().is_empty(), "{:?}", jar.list());
    }

    #[test]
    fn a_host_that_is_a_suffix_gets_a_host_only_cookie() {
        let jar = CookieJar::new();
        receive(&jar, "http://localhost:3000/", "s=1; Domain=localhost");
        assert_eq!(sent(&jar, "http://localhost:3000/").as_deref(), Some("s=1"));
    }

    #[test]
    fn domains_that_dont_match_are_rejected() {
        let jar = CookieJar::new();
        receive(&jar, "https://evil.com/", "s=1; Domain=bank.com");
        assert_eq!(sent(&jar, "https://bank.com/"), None);
    }

    #[test]
    fn secure_and_path_are_respected() {
        let jar = CookieJar::new();
        receive(&jar, "https://example.com/", "s=1; Secure");
        receive(&jar, "https://example.com/", "p=2; Path=/admin");
        assert_eq!(sent(&jar, "http://example.com/"), None);
        assert_eq!(sent(&jar, "https://example.com/").as_deref(), Some("s=1"));
        let admin = sent(&jar, "https://example.com/admin/x").unwrap();
        assert!(admin.contains("s=1") && admin.contains("p=2"), "{admin}");
    }

    #[test]
    fn servers_can_remove_cookies() {
        let jar = CookieJar::new();
        receive(&jar, "https://example.com/", "s=1");
        receive(&jar, "https://example.com/", "s=; Max-Age=0");
        assert_eq!(sent(&jar, "https://example.com/"), None);
    }

    #[test]
    fn save_and_load_keep_session_cookies_and_drop_expired_ones() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".nativedoctor/state/dev.cookies.json");

        let jar = CookieJar::new();
        receive(&jar, "https://example.com/", "session=abc");
        receive(&jar, "https://example.com/", "lasting=def; Max-Age=3600");
        receive(&jar, "https://example.com/", "gone=x; Max-Age=3600");
        receive(&jar, "https://example.com/", "gone=; Max-Age=0");
        jar.save(&path).unwrap();

        let text = std::fs::read_to_string(&path).unwrap();
        assert!(!text.contains("gone"), "{text}");

        let loaded = CookieJar::load(&path).unwrap();
        let header = sent(&loaded, "https://example.com/").unwrap();
        assert!(header.contains("session=abc"), "{header}");
        assert!(header.contains("lasting=def"), "{header}");

        let sessions: Vec<bool> = loaded.list().iter().map(|c| c.session).collect();
        assert_eq!(sessions.iter().filter(|s| **s).count(), 1);
    }

    #[test]
    fn an_empty_jar_writes_nothing_and_a_missing_file_loads_empty() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("state/dev.cookies.json");
        CookieJar::new().save(&path).unwrap();
        assert!(!path.exists());
        assert!(CookieJar::load(&path).unwrap().list().is_empty());
    }

    #[test]
    fn a_corrupt_file_is_an_error_that_names_it() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("dev.cookies.json");
        std::fs::write(&path, "not json").unwrap();
        let message = CookieJar::load(&path).unwrap_err().to_string();
        assert!(message.contains("dev.cookies.json"), "{message}");
    }

    #[cfg(unix)]
    #[test]
    fn the_saved_file_is_owner_only() {
        use std::os::unix::fs::PermissionsExt;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(".nativedoctor/state/dev.cookies.json");
        let jar = CookieJar::new();
        receive(&jar, "https://example.com/", "s=1");
        jar.save(&path).unwrap();
        let mode = std::fs::metadata(&path).unwrap().permissions().mode();
        assert_eq!(mode & 0o777, 0o600);
    }

    #[test]
    fn masked_header_hides_values() {
        let jar = CookieJar::new();
        receive(&jar, "https://example.com/", "session=abc");
        let masked = jar.masked_header(&url("https://example.com/")).unwrap();
        assert_eq!(masked, "session=***");
        assert_eq!(jar.masked_header(&url("https://other.com/")), None);
    }
}
