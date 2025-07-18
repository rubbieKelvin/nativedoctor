use nativedoctor_core::EXTENSION_FOR_PROJECT;
use rfd::AsyncFileDialog;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::session::Session;

impl Session {
    /// gets the path where the project was or should be stored
    async fn get_project_path(&self) -> Option<PathBuf> {
        // we have an actuall path (file has been saved before)
        if self.path.is_some() {
            return self.path.clone();
        }

        // file has not been saved before
        // open file dialog
        let picker = AsyncFileDialog::new().set_title("Pick folder to save project");
        return picker.pick_folder().await.map(|handler| {
            let path = handler.path();
            let root = path.join(format!("project.{EXTENSION_FOR_PROJECT}"));
            return root;
        });
    }

    pub async fn save_to_fs(&self) -> Result<(), String> {
        // TODO: Update return type
        // IDEA: we can keep track of what changed so we dont have to rewite the whole project
        // 01: We need to save the project meta data
        // 02: We need to save all requests into a sub /request folder
        let project_root = self.cast_to_project_root_schema();
        let path = self.get_project_path().await;

        if path.is_none() {
            return Err("Could not get file path".to_string());
        }

        let path = path.unwrap();
        tracing::debug!("Path: {:?}", path);

        // TODO: handle these errors better
        let content = serde_yaml::to_string(&project_root).map_err(|e| e.to_string())?;
        let mut file = fs::File::create(path).map_err(|e| e.to_string())?;
        file.write_all(content.as_bytes()).map_err(|e| e.to_string())?;

        return Ok(());
    }

    pub fn load_from_fs() -> Self {
        tracing::warn!("Not implemented yet");
        return Session {
            path: Some(PathBuf::new()),
            name: "NotImplementedYet".to_string(),
            description: String::new(),
            version: "0.0.1".to_string(),
            ..Default::default()
        };
    }
}
