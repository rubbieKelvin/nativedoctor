use std::{
    fs::File,
    io::{BufReader, BufWriter},
    path::Path,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RecentProject {
    pub path: String,
    pub name: String,
    pub active_environment: Option<String>,
}

impl RecentProject {
    pub fn new(path: String, name: String, active_environment: Option<String>) -> Self {
        Self {
            path,
            name,
            active_environment,
        }
    }
}

const RECENTS_PATH: &str = "recents.json";

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct RecentProjects {
    pub projects: Vec<RecentProject>,
}

impl RecentProjects {
    pub fn init() -> Self {
        let path = Path::new(RECENTS_PATH);

        if path.exists() {
            let projects = match Self::read() {
                Ok(projects) => projects,
                Err(e) => {
                    tracing::error!("Failed to read recents.json: {e}");
                    vec![]
                }
            };

            return Self { projects };
        }

        Self { projects: vec![] }
    }

    pub fn is_empty(&self) -> bool {
        self.projects.is_empty()
    }

    pub fn add(&mut self, project: RecentProject) -> Result<(), Box<dyn std::error::Error>> {
        // Remove any existing project with the same path to ensure uniqueness
        self.projects
            .retain(|existing| existing.path != project.path);

        // Add the project (this will be the most recent)
        self.projects.push(project);
        self.write()?;
        Ok(())
    }

    pub fn remove(&mut self, path: String) -> Result<(), Box<dyn std::error::Error>> {
        self.projects.retain(|project| project.path != path);
        self.write()?;
        Ok(())
    }

    fn write(&self) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create(RECENTS_PATH);

        if let Ok(file) = file {
            let writer = BufWriter::new(file);
            serde_json::to_writer_pretty(writer, &self.projects)?;
        }

        Ok(())
    }

    fn read() -> Result<Vec<RecentProject>, Box<dyn std::error::Error>> {
        let file = File::open(RECENTS_PATH);

        if let Ok(file) = file {
            let reader = BufReader::new(file);
            let projects: Vec<RecentProject> = serde_json::from_reader(reader)?;
            return Ok(projects);
        }

        return Ok(vec![]);
    }

    pub fn get(&self, path: String) -> Option<RecentProject> {
        return self
            .projects
            .iter()
            .find(|project| project.path == path)
            .cloned();
    }
}
