use std::path::PathBuf;

use serde::Deserialize;

#[derive(Clone, Deserialize, Debug, PartialEq)]
pub struct MetaSchema {
    pub filepath: Option<PathBuf>, // the file the parent struct belongs to
    pub working_directory: Option<PathBuf>,
}

impl MetaSchema {
    pub fn new() -> MetaSchema {
        return MetaSchema {
            filepath: None,
            working_directory: None,
        };
    }

    pub fn add_filepath(mut self, filepath: Option<PathBuf>) -> Self {
        self.filepath = filepath;
        return self;
    }

    pub fn add_wd(mut self, working_directory: Option<PathBuf>) -> Self {
        self.working_directory = working_directory;
        return self;
    }
}
