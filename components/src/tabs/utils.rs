use std::{slice::Iter, vec::IntoIter};

use dioxus::prelude::*;
use uuid::Uuid;

// defined these marker trait here so i dont have to do T: ..
// i can just do T: TabGenerics
pub trait TabGenerics: PartialEq + Clone + TabPayload {}
impl<T> TabGenerics for T where T: PartialEq + Clone + TabPayload {}

// Tab payload must implement this trait
pub trait TabPayload {
    type Identifier: Clone + PartialEq;
    /// we need a title for the tab button.
    fn render_title(&self, selected: bool) -> Element;

    /// we need a way to uniquely identify a page so we dont open one page twice per tab
    fn unique_identifier(&self) -> Self::Identifier;
}

// data that the tab can work with. this would wrap a payload that holds the actual data
#[derive(Clone, PartialEq)]
pub struct TabItemData<T: TabGenerics> {
    pub id: Uuid,
    pub payload: T,
    pub closable: bool,
}

impl<T: TabGenerics> TabItemData<T> {
    pub fn new(payload: T) -> Self {
        TabItemData {
            id: Uuid::new_v4(),
            payload,
            closable: true,
        }
    }

    #[allow(unused)]
    pub fn set_closable(mut self, closable: bool) -> Self {
        self.closable = closable;
        return self;
    }
}

#[derive(PartialEq, Clone)]
pub struct TabSet<T: TabGenerics>(Option<uuid::Uuid>, Vec<TabItemData<T>>);

impl<T: TabGenerics> TabSet<T> {
    pub fn new(vector: Vec<TabItemData<T>>) -> Self {
        return TabSet(
            // set the selected id to the first item in the tabset
            vector.first().map(|t| t.id.clone()),
            vector,
        );
    }

    pub fn iter(&self) -> Iter<'_, TabItemData<T>> {
        return self.1.iter();
    }

    #[allow(unused)]
    pub fn add_tab(&mut self, item: TabItemData<T>) {
        if self.get_similar(&item).is_some() {
            tracing::info!("Already has a tab with similar content. Skipping");
            return;
        }

        self.1.push(item);
    }

    pub fn get_similar(&self, item: &TabItemData<T>) -> Option<&TabItemData<T>> {
        let uid = item.payload.unique_identifier();
        return self
            .iter()
            .find(|item| item.payload.unique_identifier() == uid);
    }

    pub fn remove_via_index(&mut self, index: usize) {
        self.1.remove(index);
    }

    pub fn len(&self) -> usize {
        return self.1.len();
    }

    pub fn is_empty(&self) -> bool {
        return self.1.is_empty();
    }

    pub fn get(&self, index: usize) -> Option<&TabItemData<T>> {
        return self.1.get(index);
    }

    pub fn first(&self) -> Option<&TabItemData<T>> {
        return self.1.first();
    }

    pub fn get_selected(&self) -> Option<&TabItemData<T>> {
        return self
            .0
            .map(|id| self.1.iter().filter(|t| t.id == id).last())?;
    }

    pub fn get_selected_mut(&mut self) -> Option<&mut TabItemData<T>> {
        return self
            .0
            .map(|id| self.1.iter_mut().filter(|t| t.id == id).last())?;
    }

    #[allow(unused)]
    pub fn clear(&mut self) {
        self.1.retain(|t| !t.closable);
    }

    pub fn select(&mut self, id: Option<uuid::Uuid>) {
        self.0 = id;
    }
}

impl<T: TabGenerics> IntoIterator for TabSet<T> {
    type Item = TabItemData<T>;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.1.into_iter()
    }
}

/// Context passed to each tab pill and its children.
#[derive(Clone, PartialEq)]
pub struct TabState<T: TabGenerics + 'static> {
    pub tab: TabItemData<T>,
    pub tabs: Signal<TabSet<T>>,
}

impl<T: TabGenerics + 'static> TabState<T> {
    /// Removes the current tab from the list and handles re-selection.
    pub fn remove(&self) {
        let tab_id_to_remove = self.tab.id;
        let mut tabs = self.tabs;

        let currently_selected_id = tabs().get_selected().map(|t| t.id.clone());

        tabs.with_mut(|tabs_set| {
            let Some(index_to_remove) = tabs_set.iter().position(|t| t.id == tab_id_to_remove)
            else {
                tracing::warn!("Tried to remove a tab that does not exist.");
                return;
            };

            // Remove the tab.
            tabs_set.remove_via_index(index_to_remove);
            tracing::info!(
                "Removed tab at index {}. New count: {}",
                index_to_remove,
                tabs_set.len()
            );

            // If the removed tab was the selected one, we need to select a new one.
            if currently_selected_id == Some(tab_id_to_remove) {
                if tabs_set.is_empty() {
                    // No tabs left, so clear selection.
                    tabs_set.select(None);
                } else {
                    // Select the item at the same index, or the new last item
                    // if the original last item was the one removed.
                    let new_index = index_to_remove.min(tabs_set.len() - 1);
                    let new_selected_id = tabs_set.get(new_index).map(|t| t.id);
                    tabs_set.select(new_selected_id);
                    
                }
            }
        });
    }
}
