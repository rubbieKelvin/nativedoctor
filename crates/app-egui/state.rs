use serde::{Deserialize, Serialize};

/// Holds serializzable application states, flags, ..etc
/// we can choose to save this state to disk and retrieve it later on
#[derive(Default, Serialize, Deserialize)]
pub struct ApplicationState {
    pub page: PageType,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PageType {
    #[default]
    RequestPage,
}
