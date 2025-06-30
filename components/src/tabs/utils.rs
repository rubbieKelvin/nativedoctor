use dioxus::prelude::*;
use uuid::Uuid;

// defined these marker trait here so i dont have to do T: ..
// i can just do T: Tab
pub trait Tab: PartialEq + Clone + Into<TabString> {}
impl<T> Tab for T where T: PartialEq + Clone + Into<TabString> {}

// Tab string so we can render text on the tab button
#[derive(Clone, Default, PartialEq)]
pub struct TabString(pub String);

impl From<String> for TabString {
    fn from(s: String) -> Self {
        TabString(s)
    }
}

impl Into<String> for TabString {
    fn into(self) -> String {
        self.0
    }
}

// data that the tab can work with. this would wrap a payload that holds the actual data
#[derive(Clone, PartialEq)]
pub struct TabItemData<T: Tab> {
    pub id: Uuid,
    pub item: T,
    pub closable: bool,
}

impl<T: Tab> TabItemData<T> {
    pub fn new(item: T) -> Self {
        TabItemData {
            id: Uuid::new_v4(),
            item,
            closable: true,
        }
    }
}

/// Context passed to each tab pill and its children.
#[derive(Clone, PartialEq)]
pub struct TabState<T: Tab + 'static> {
    pub tab: TabItemData<T>,
    pub tabs: Signal<Vec<TabItemData<T>>>, // TODO: this should be a set
    pub selected_tab: Signal<Option<Uuid>>,
}

impl<T: Tab + 'static> TabState<T> {
    /// Removes the current tab from the list and handles re-selection.
    pub fn remove(&self) {
        let tab_id_to_remove = self.tab.id;
        let mut tabs: Signal<Vec<TabItemData<T>>> = self.tabs;
        let mut selected_tab: Signal<Option<Uuid>> = self.selected_tab;

        let currently_selected_id = selected_tab.cloned();

        tabs.with_mut(|tabs_vec| {
            let Some(index_to_remove) = tabs_vec.iter().position(|t| t.id == tab_id_to_remove)
            else {
                tracing::warn!("Tried to remove a tab that does not exist.");
                return;
            };

            // Remove the tab.
            tabs_vec.remove(index_to_remove);
            tracing::info!(
                "Removed tab at index {}. New count: {}",
                index_to_remove,
                tabs_vec.len()
            );

            // If the removed tab was the selected one, we need to select a new one.
            if currently_selected_id == Some(tab_id_to_remove) {
                if tabs_vec.is_empty() {
                    // No tabs left, so clear selection.
                    selected_tab.set(None);
                } else {
                    // Select the item at the same index, or the new last item
                    // if the original last item was the one removed.
                    let new_index = index_to_remove.min(tabs_vec.len() - 1);
                    let new_selected_id = tabs_vec.get(new_index).map(|t| t.id);
                    selected_tab.set(new_selected_id);
                }
            }
        });
    }
}
