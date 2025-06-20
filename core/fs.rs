use std::path::{Path, PathBuf};

use anyhow::Context;
use serde::Deserialize;
use tokio::io::AsyncReadExt;

use crate::schema::roots::{ProjectRootSchema, RequestRootSchema};

#[derive(Clone, PartialEq)]
pub struct FileObject<T: Clone + PartialEq + Deserialize<'static>> {
    pub id: uuid::Uuid,
    pub path: PathBuf,
    pub object: T,
}

impl<T: Clone + PartialEq + Deserialize<'static>> FileObject<T> {
    pub fn new(path: PathBuf, object: T) -> FileObject<T> {
        return FileObject {
            id: uuid::Uuid::new_v4(),
            path,
            object,
        };
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
    fn get_requests_dir(&self) -> PathBuf {
        return self.path.join(match &self.object.requests_dir {
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
    pub fn get_name(&self) -> String {
        let filename = self.path.file_name().unwrap();
        return filename.to_str().unwrap().to_string();
    }
}