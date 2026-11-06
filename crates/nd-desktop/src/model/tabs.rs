//! The open request tabs.

use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub struct OpenTab {
    pub path: PathBuf,
    pub title: String,
}

#[derive(Debug, Default)]
pub struct Tabs {
    pub items: Vec<OpenTab>,
    pub active: Option<usize>,
}

impl Tabs {
    /// Shows `path`, opening a tab only if it isn't open already.
    pub fn open(&mut self, path: &Path, title: String) {
        if let Some(index) = self.items.iter().position(|tab| tab.path == path) {
            self.active = Some(index);
            return;
        }
        self.items.push(OpenTab {
            path: path.to_path_buf(),
            title,
        });
        self.active = Some(self.items.len() - 1);
    }

    pub fn select(&mut self, index: usize) {
        if index < self.items.len() {
            self.active = Some(index);
        }
    }

    /// Closes a tab. The one before it takes over, as in a browser.
    pub fn close(&mut self, index: usize) {
        if index >= self.items.len() {
            return;
        }
        self.items.remove(index);
        self.active = match self.active {
            _ if self.items.is_empty() => None,
            Some(active) if active > index => Some(active - 1),
            Some(active) if active == index => Some(index.saturating_sub(1)),
            other => other,
        };
    }

    pub fn active_tab(&self) -> Option<&OpenTab> {
        return self.active.and_then(|index| self.items.get(index));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tabs(names: &[&str]) -> Tabs {
        let mut tabs = Tabs::default();
        for name in names {
            tabs.open(Path::new(name), name.to_string());
        }
        return tabs;
    }

    #[test]
    fn opening_twice_reuses_the_tab() {
        let mut tabs = tabs(&["a", "b"]);
        tabs.open(Path::new("a"), "a".to_string());
        assert_eq!(tabs.items.len(), 2);
        assert_eq!(tabs.active, Some(0));
    }

    #[test]
    fn closing_keeps_a_sensible_selection() {
        let mut tabs = tabs(&["a", "b", "c"]);
        tabs.close(2);
        assert_eq!(tabs.active_tab().unwrap().title, "b");
        tabs.select(0);
        tabs.close(1);
        assert_eq!(tabs.active_tab().unwrap().title, "a");
        tabs.close(0);
        assert_eq!(tabs.active, None);
        tabs.close(5);
    }

    #[test]
    fn closing_an_earlier_tab_keeps_the_same_one_active() {
        let mut tabs = tabs(&["a", "b", "c"]);
        tabs.close(0);
        assert_eq!(tabs.active_tab().unwrap().title, "c");
    }
}
