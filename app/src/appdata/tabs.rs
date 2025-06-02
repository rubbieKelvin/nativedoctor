use dioxus::{
    hooks::{use_context, use_context_provider},
    signals::Signal,
};
use uuid::Uuid;

#[allow(unused)]
#[derive(Clone, Debug, PartialEq)]
pub enum TabType {
    Request,
    Sequence,
    WelcomePage,
    EditEnvironment,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TabItem {
    pub id: String,
    pub name: String,
    pub tab_type: TabType,
    pub payload: Option<String>,
}

impl TabItem {
    pub fn new(name: String, tab_type: TabType, payload: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            tab_type,
            payload,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TabItemManager {
    pub current_tab: Option<usize>,
    pub tabs: Vec<TabItem>,
}

impl TabItemManager {
    pub fn provide() {
        use_context_provider::<Signal<TabItemManager>>(|| {
            Signal::new(TabItemManager {
                current_tab: Some(0),
                tabs: vec![TabItem::new(
                    "Welcome".to_string(),
                    TabType::WelcomePage,
                    None,
                )],
            })
        });
    }

    pub fn inject() -> Signal<TabItemManager> {
        return use_context::<Signal<TabItemManager>>();
    }

    pub fn add_tab(&mut self, tab: TabItem) {
        for t in &self.tabs {
            if t.payload == tab.payload && t.tab_type == tab.tab_type {
                self.set_current_tab(t.id.clone());
                return;
            }
        }

        // if payload is None, and the type is in the list, don't add it again
        if tab.payload.is_none() {
            for t in &self.tabs {
                if t.tab_type == tab.tab_type {
                    self.set_current_tab(t.id.clone());
                    return;
                }
            }
        }

        let id = tab.id.clone();
        self.tabs.push(tab);
        self.set_current_tab(id);
    }

    pub fn get_current_tab(&self) -> Option<TabItem> {
        if self.current_tab.is_none() {
            return None;
        }

        return match self.tabs.get(self.current_tab.unwrap()) {
            Some(tab) => Some(tab.clone()),
            None => None,
        };
    }

    pub fn remove_tab(&mut self, id: String) {
        let removed_tab_index = self.tabs.iter().position(|t| t.id == id);
        if removed_tab_index.is_none() {
            return; // Tab not found
        }
        let removed_tab_index = removed_tab_index.unwrap();

        self.tabs.retain(|t| t.id != id);


        match self.current_tab {
            Some(current_idx) => {
                if removed_tab_index == current_idx {
                    // Active tab was removed
                    if current_idx > 0 && current_idx <= self.tabs.len() {
                        // If there was a tab to the left, make it active
                        self.current_tab = Some(current_idx - 1);
                    } else if !self.tabs.is_empty() {
                        // Otherwise, if there are still tabs, make the new first tab active
                        self.current_tab = Some(0);
                    } else {
                        // This case should be covered by the is_empty check above,
                        // but as a fallback, set to None.
                        self.current_tab = None;
                    }
                } else if removed_tab_index < current_idx {
                    // A tab to the left of the active tab was removed
                    self.current_tab = Some(current_idx - 1);
                }
                // If a tab to the right of the active tab was removed, current_tab index remains valid
            }
            None => {
                // No active tab was set, or tabs list was empty before.
                // If there are tabs now, make the first one active.
                if !self.tabs.is_empty() {
                    self.current_tab = Some(0);
                }
            }
        }
    }

    pub fn set_current_tab(&mut self, id: String) {
        self.current_tab = self.tabs.iter().position(|t| t.id == id);
    }
}
