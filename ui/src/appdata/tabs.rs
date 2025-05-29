use uuid::Uuid;

#[allow(unused)]
#[derive(Clone, Debug, PartialEq)]
pub enum TabType {
    Request,
    Sequence,
    WelcomePage,
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
    pub fn add_tab(&mut self, tab: TabItem) {
        for t in &self.tabs {
            if t.payload == tab.payload && t.tab_type == tab.tab_type {
                self.set_current_tab(t.id.clone());
                return;
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
        self.tabs.retain(|t| t.id != id);
        if self.tabs.len() == 0 {
            self.add_tab(TabItem::new(
                "Welcome".to_string(),
                TabType::WelcomePage,
                None,
            ));
        }
    }

    pub fn set_current_tab(&mut self, id: String) {
        self.current_tab = self.tabs.iter().position(|t| t.id == id);
    }
}
