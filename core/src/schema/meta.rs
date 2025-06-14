use std::path::PathBuf;

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct MetaSchema {
    pub filepath: Option<PathBuf>, // the file the parent struct belongs to
    pub main_file: Option<PathBuf> // the original file that was called
}

impl MetaSchema {
    pub fn new() -> MetaSchema {
        return MetaSchema {
            filepath: None,
            main_file: None
        };
    }

    pub fn set_filepath(mut self, filepath: Option<PathBuf>) -> Self {
        self.filepath = filepath;
        return self;
    }

    pub fn set_main_file(mut self, main_file: Option<PathBuf>) -> Self {
        self.main_file = main_file;
        return self;
    }
}
