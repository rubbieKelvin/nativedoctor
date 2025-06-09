use uuid::Uuid;

#[derive(PartialEq, Clone, Debug)]
pub enum KeyValuePairData {
    String(String, String), // (key, value)
    File(String, String),   // (key, file_path)
}

#[derive(PartialEq, Clone, Debug)]
pub struct KeyValuePairObject {
    pub id: String,
    pub enabled: bool,
    pub data: KeyValuePairData,
}

impl KeyValuePairObject {
    // Helper to create a new default string pair
    pub fn new_string() -> Self {
        KeyValuePairObject {
            id: Uuid::new_v4().to_string(),
            enabled: true,
            data: KeyValuePairData::String(String::new(), String::new()),
        }
    }

    // Helper to create a new default file pair
    pub fn new_file() -> Self {
        KeyValuePairObject {
            id: Uuid::new_v4().to_string(),
            enabled: true,
            data: KeyValuePairData::File(String::new(), String::new()),
        }
    }

    pub fn key(&self) -> &str {
        match &self.data {
            KeyValuePairData::String(key, _) => key,
            KeyValuePairData::File(key, _) => key,
        }
    }

    pub fn value(&self) -> &str {
        match &self.data {
            KeyValuePairData::String(_, value) => value,
            KeyValuePairData::File(_, value) => value,
        }
    }
}