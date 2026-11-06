//! A request file being edited: what's on disk, and how edits get back there.

use std::fs;
use std::path::{Path, PathBuf};

use nd_constants::REQUEST_SUFFIX;
use nd_core::patch::{patch_yaml, rewrite_model, write_atomic};
use nd_core::{Error, RequestFile, Result};

#[derive(Debug, Clone, Default)]
pub struct Document {
    pub path: Option<PathBuf>,
    /// Exact file contents when last read or written.
    disk_text: Option<String>,
    disk_model: Option<RequestFile>,
}

impl Document {
    /// A request that isn't a file yet.
    pub fn scratch() -> Document {
        Document::default()
    }

    /// Reads a file. Its text is kept even if it doesn't parse, so it can be fixed by hand.
    pub fn open(path: &Path) -> Result<(Document, Result<RequestFile>)> {
        let text = fs::read_to_string(path).map_err(|e| Error::io(path, e))?;
        let parsed = RequestFile::from_yaml(&text);
        let document = Document {
            path: Some(path.to_path_buf()),
            disk_model: parsed.as_ref().ok().cloned(),
            disk_text: Some(text),
        };
        return Ok((document, parsed));
    }

    pub fn disk_text(&self) -> Option<&str> {
        self.disk_text.as_deref()
    }

    pub fn is_dirty(&self, draft: &RequestFile) -> bool {
        let Some(saved) = &self.disk_model else {
            return self.path.is_some() || !same_model(draft, &RequestFile::default());
        };
        return !same_model(saved, draft);
    }

    /// The YAML this draft would save as, with the file's comments and formatting.
    pub fn yaml_for(&self, draft: &RequestFile) -> Result<String> {
        if let (Some(text), Some(model)) = (&self.disk_text, &self.disk_model) {
            return patch_yaml(text, model, draft);
        }
        return serde_yaml_ng::to_string(draft).map_err(|e| Error::Invalid(e.to_string()));
    }

    /// Saves from the form, keeping comments. `Error::Unpreserved` means nothing was written,
    /// and the user needs to choose between the YAML tab and `save_rewriting`.
    pub fn save(&mut self, draft: &RequestFile) -> Result<()> {
        let path = self.require_path()?;
        let output = self.yaml_for(draft)?;
        if self.disk_text.as_deref() != Some(output.as_str()) {
            write_atomic(&path, output.as_bytes())?;
        }
        self.remember(output, draft.clone());
        return Ok(());
    }

    /// Replaces the file from the form, losing its comments. Only after the user has agreed.
    pub fn save_rewriting(&mut self, draft: &RequestFile) -> Result<()> {
        let path = self.require_path()?;
        rewrite_model(&path, draft)?;
        let text = fs::read_to_string(&path).map_err(|e| Error::io(&path, e))?;
        self.remember(text, draft.clone());
        return Ok(());
    }

    /// Saves the YAML tab's text byte for byte. Refuses text that doesn't parse.
    pub fn save_yaml(&mut self, text: &str) -> Result<RequestFile> {
        let path = self.require_path()?;
        let parsed = RequestFile::from_yaml(text)?;
        write_atomic(&path, text.as_bytes())?;
        self.remember(text.to_string(), parsed.clone());
        return Ok(parsed);
    }

    /// Writes the draft to a new file and keeps editing that file.
    pub fn save_as(&mut self, path: &Path, draft: &RequestFile) -> Result<PathBuf> {
        let path = with_request_suffix(path);
        rewrite_model(&path, draft)?;
        let text = fs::read_to_string(&path).map_err(|e| Error::io(&path, e))?;
        self.path = Some(path.clone());
        self.remember(text, draft.clone());
        return Ok(path);
    }

    /// True if the file no longer holds what was last read or written.
    pub fn changed_on_disk(&self) -> bool {
        let Some(path) = &self.path else {
            return false;
        };
        return fs::read_to_string(path).ok().as_deref() != self.disk_text.as_deref();
    }

    fn require_path(&self) -> Result<PathBuf> {
        return self
            .path
            .clone()
            .ok_or_else(|| Error::Invalid("this request isn't a file yet".to_string()));
    }

    fn remember(&mut self, text: String, model: RequestFile) {
        self.disk_text = Some(text);
        self.disk_model = Some(model);
    }
}

fn same_model(a: &RequestFile, b: &RequestFile) -> bool {
    let (a, b) = (serde_json::to_string(a), serde_json::to_string(b));
    return matches!((a, b), (Ok(a), Ok(b)) if a == b);
}

/// Adds `.request.yaml` to a name the user typed without it.
pub fn with_request_suffix(path: &Path) -> PathBuf {
    let text = path.to_string_lossy();
    if text.ends_with(REQUEST_SUFFIX) {
        return path.to_path_buf();
    }
    let base = text.strip_suffix(".yaml").unwrap_or(&text);
    return PathBuf::from(format!("{base}{REQUEST_SUFFIX}"));
}

/// Splits a command line into words. Quotes group words, since there's no shell here to do it.
pub fn split_command_line(line: &str) -> Result<Vec<String>> {
    let mut words = Vec::new();
    let mut word = String::new();
    let mut in_word = false;
    let mut quote: Option<char> = None;
    let mut chars = line.chars();
    while let Some(c) = chars.next() {
        match (quote, c) {
            (Some(open), c) if c == open => quote = None,
            (Some('"'), '\\') => {
                if let Some(next) = chars.next() {
                    if !matches!(next, '"' | '\\') {
                        word.push('\\');
                    }
                    word.push(next);
                }
            }
            (Some(_), c) => word.push(c),
            (None, '"' | '\'') => {
                quote = Some(c);
                in_word = true;
            }
            (None, '\\') => {
                // Kept, because `foo\==bar` means something to the item parser.
                word.push('\\');
                if let Some(next) = chars.next() {
                    word.push(next);
                }
                in_word = true;
            }
            (None, c) if c.is_whitespace() => {
                if in_word {
                    words.push(std::mem::take(&mut word));
                    in_word = false;
                }
            }
            (None, c) => {
                word.push(c);
                in_word = true;
            }
        }
    }
    if quote.is_some() {
        return Err(Error::Invalid("a quote is never closed".to_string()));
    }
    if in_word {
        words.push(word);
    }
    return Ok(words);
}

/// A pasted one-off request, split like `[METHOD] URL [ITEM...]`.
#[derive(Debug, Clone, PartialEq)]
pub struct CommandLine {
    pub method: Option<String>,
    pub url: String,
    pub items: Vec<String>,
}

/// Reads a pasted command. Returns `None` for a plain URL, which needs no special handling.
pub fn parse_command_line(line: &str) -> Result<Option<CommandLine>> {
    let mut words = split_command_line(line)?;
    if words.first().is_some_and(|word| word == "nativedoctor") {
        words.remove(0);
    }
    if words.first().is_some_and(|word| word == "send") {
        words.remove(0);
    }
    if words.len() < 2 {
        return Ok(None);
    }
    // Same rule as the CLI: only an all-caps first word with more after it is a method.
    let is_method = words[0].chars().all(|c| c.is_ascii_uppercase());
    let method = is_method.then(|| words.remove(0));
    let url = words.remove(0);
    return Ok(Some(CommandLine {
        method,
        url,
        items: words,
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "# about this request\nname: Ping # short\nhttp:\n  url: http://a # note\n";

    fn saved(text: &str) -> (tempfile::TempDir, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("a.request.yaml");
        fs::write(&path, text).unwrap();
        return (dir, path);
    }

    fn opened(path: &Path) -> (Document, RequestFile) {
        let (document, parsed) = Document::open(path).unwrap();
        return (document, parsed.unwrap());
    }

    #[test]
    fn form_saves_change_only_what_changed() {
        let (_dir, path) = saved(FILE);
        let (mut document, mut draft) = opened(&path);
        assert!(!document.is_dirty(&draft));

        draft.http.url = "http://b".to_string();
        assert!(document.is_dirty(&draft));
        document.save(&draft).unwrap();

        assert_eq!(fs::read_to_string(&path).unwrap(), FILE.replace("http://a", "http://b"));
        assert!(!document.is_dirty(&draft));
        assert!(!document.changed_on_disk());
    }

    #[test]
    fn saving_twice_builds_on_the_first_save() {
        let (_dir, path) = saved(FILE);
        let (mut document, mut draft) = opened(&path);
        draft.http.url = "http://b".to_string();
        document.save(&draft).unwrap();
        draft.name = Some("Pong".to_string());
        document.save(&draft).unwrap();
        let text = fs::read_to_string(&path).unwrap();
        assert!(text.contains("# about this request") && text.contains("Pong # short"), "{text}");
    }

    #[test]
    fn yaml_saves_are_byte_for_byte() {
        let (_dir, path) = saved(FILE);
        let (mut document, _) = opened(&path);
        let text = "http: {url: 'http://c'}   # odd   spacing\n";
        let parsed = document.save_yaml(text).unwrap();
        assert_eq!(parsed.http.url, "http://c");
        assert_eq!(fs::read_to_string(&path).unwrap(), text);
    }

    #[test]
    fn invalid_yaml_is_refused_and_leaves_the_file_alone() {
        let (_dir, path) = saved(FILE);
        let (mut document, _) = opened(&path);
        assert!(document.save_yaml("http: [nope").is_err());
        assert_eq!(fs::read_to_string(&path).unwrap(), FILE);
    }

    #[test]
    fn an_unparseable_file_keeps_its_text() {
        let (_dir, path) = saved("http: [broken\n");
        let (document, parsed) = Document::open(&path).unwrap();
        assert!(parsed.is_err());
        assert_eq!(document.disk_text(), Some("http: [broken\n"));
    }

    #[test]
    fn notices_changes_on_disk() {
        let (_dir, path) = saved(FILE);
        let (document, _) = opened(&path);
        assert!(!document.changed_on_disk());
        fs::write(&path, "http: { url: changed }\n").unwrap();
        assert!(document.changed_on_disk());
    }

    #[test]
    fn scratch_requests_save_as_a_new_file_with_the_suffix() {
        let dir = tempfile::tempdir().unwrap();
        let mut document = Document::scratch();
        let mut draft = RequestFile::default();
        assert!(!document.is_dirty(&draft));
        draft.http.url = "http://a".to_string();
        assert!(document.is_dirty(&draft));
        assert!(document.save(&draft).is_err());

        let path = document.save_as(&dir.path().join("new"), &draft).unwrap();
        assert!(path.ends_with("new.request.yaml"));
        assert!(!document.is_dirty(&draft));
        assert!(RequestFile::load(&path).is_ok());
    }

    #[test]
    fn splits_like_a_shell() {
        let words = split_command_line(r#"POST :3000 name="John Smith" 'a b'=c x\==y "#).unwrap();
        assert_eq!(words, vec!["POST", ":3000", "name=John Smith", "a b=c", r"x\==y"]);
        assert!(split_command_line("a \"b").is_err());
    }

    #[test]
    fn reads_pasted_commands() {
        let parsed = parse_command_line("nativedoctor send POST :3000/users name=John").unwrap();
        assert_eq!(
            parsed,
            Some(CommandLine {
                method: Some("POST".to_string()),
                url: ":3000/users".to_string(),
                items: vec!["name=John".to_string()],
            })
        );
        let no_method = parse_command_line(":3000/a x==1").unwrap().unwrap();
        assert_eq!(no_method.method, None);
        assert_eq!(no_method.url, ":3000/a");
        assert_eq!(parse_command_line("https://example.com/a").unwrap(), None);
    }
}
