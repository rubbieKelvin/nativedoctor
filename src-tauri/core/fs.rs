use std::path::{Path, PathBuf};

use anyhow::{Context, Ok, bail};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncReadExt;

use crate::{
    EXTENSION_FOR_REQUEST,
    schema::roots::{ProjectRootSchema, RequestRootSchema},
};

#[derive(Clone, PartialEq)]
pub struct FileObject<T: Clone + PartialEq + Deserialize<'static> + Serialize> {
    pub id: uuid::Uuid,
    pub path: PathBuf,
    pub object: T,
}

impl<T: Clone + PartialEq + Deserialize<'static> + Serialize> FileObject<T> {
    pub fn new(path: PathBuf, object: T) -> FileObject<T> {
        return FileObject {
            id: uuid::Uuid::new_v4(),
            path,
            object,
        };
    }

    pub fn copy_from(&mut self, other: FileObject<T>) {
        self.path = other.path;
        self.object = other.object;
    }

    pub async fn save(&self) -> anyhow::Result<()> {
        let path = self.path.clone();

        // Make sure path is absolue
        if !path.is_absolute() {
            tracing::error!("{:?} Project path must be absolute", &path);
            bail!("Project path must be absolute".to_string());
        }

        let content = serde_yaml::to_string(&self.object).context("Error serializing object")?;
        tokio::fs::write(&self.path, content).await?;
        return Ok(());
    }
}

impl ProjectRootSchema {
    pub async fn load(path: &Path) -> anyhow::Result<FileObject<ProjectRootSchema>> {
        tracing::info!("Opening file");

        if !path.is_absolute() {
            anyhow::bail!("Path to load must be abosolute");
        }

        let file = tokio::fs::File::open(&path)
            .await
            .context("Failed to open file")?;

        let mut reader = tokio::io::BufReader::new(file);
        let mut content = String::new();

        reader.read_to_string(&mut content).await?;
        tracing::info!("{}", &content);
        let object = serde_yaml::from_str::<ProjectRootSchema>(&content)?;

        return Ok(FileObject::new(path.to_path_buf(), object));
    }
}

impl FileObject<ProjectRootSchema> {
    pub fn get_requests_dir(&self) -> PathBuf {
        let dir = self.path.parent().context("Cannot get project file parent").unwrap();
        return dir.join(match &self.object.requests_dir {
            Some(dir) => dir,
            None => "requests",
        });
    }

    pub async fn get_requests(&self) -> anyhow::Result<Vec<FileObject<RequestRootSchema>>> {
        let dir = self.get_requests_dir();
        let mut reader = tokio::fs::read_dir(dir).await?;
        let mut result = vec![];

        while let Some(entry) = reader.next_entry().await? {
            let path = entry.path();
            let file = tokio::fs::File::open(&path)
                .await
                .context("Failed to open request")?;

            let mut file_reader = tokio::io::BufReader::new(file);
            let mut content = String::new();

            file_reader.read_to_string(&mut content).await?;
            let object = serde_yaml::from_str::<RequestRootSchema>(&content)
                .context("Parsing request root error")?;

            result.push(FileObject::new(path, object));
        }

        return Ok(result);
    }
}

impl FileObject<RequestRootSchema> {
    pub fn get_name(&self) -> Option<String> {
        let filename = self.path.file_stem();
        return match filename {
            Some(filename) => filename.to_str().map(|s| s.to_string()),
            None => None,
        };
    }

    pub fn set_name(&mut self, name: &str, folder: &PathBuf) {
        let name = name.trim();
        if name == "" {
            self.path = PathBuf::new();
            return;
        }
        self.path = folder.join(format!("{name}.{EXTENSION_FOR_REQUEST}"));
    }
}
