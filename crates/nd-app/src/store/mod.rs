pub mod models;

use anyhow::Context;
use sqlx::SqlitePool;
use std::path::{Path, PathBuf};

use models::{NativedoctorProject, RecentProject};

pub struct Store {
    pool: SqlitePool,
}

impl Store {
    pub async fn init() -> anyhow::Result<Self> {
        let db_path = Self::db_path();
        let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

        let pool = SqlitePool::connect(&db_url)
            .await
            .context("failed to open database")?;

        sqlx::migrate!()
            .run(&pool)
            .await
            .context("failed to run migrations")?;

        Ok(Self { pool })
    }

    fn db_path() -> PathBuf {
        let base = dirs::home_dir().expect("home directory not found");
        let dir = base.join(".nativedoctor");
        std::fs::create_dir_all(&dir).ok();
        dir.join("nd.db")
    }

    // ---------------------------------------------------------------------------
    // Recent projects
    // ---------------------------------------------------------------------------

    pub async fn add_recent(&self, name: &str, path: &str) -> anyhow::Result<()> {
        sqlx::query(
            "INSERT INTO recent_projects (name, path) VALUES (?, ?)
             ON CONFLICT(path) DO UPDATE SET last_opened_at = datetime('now')",
        )
        .bind(name)
        .bind(path)
        .execute(&self.pool)
        .await
        .context("failed to add recent project")?;

        Ok(())
    }

    pub async fn list_recents(&self) -> anyhow::Result<Vec<RecentProject>> {
        sqlx::query_as::<_, RecentProject>(
            "SELECT id, name, path, last_opened_at FROM recent_projects ORDER BY last_opened_at DESC",
        )
        .fetch_all(&self.pool)
        .await
        .context("failed to list recent projects")
    }

    pub async fn remove_recent(&self, path: &str) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM recent_projects WHERE path = ?")
            .bind(path)
            .execute(&self.pool)
            .await
            .context("failed to remove recent project")?;

        Ok(())
    }

    // ---------------------------------------------------------------------------
    // Project creation
    // ---------------------------------------------------------------------------

    pub async fn create_project(
        &self,
        name: &str,
        location: &Path,
    ) -> anyhow::Result<NativedoctorProject> {
        let project_dir = location.join(name);
        std::fs::create_dir_all(&project_dir)
            .with_context(|| format!("failed to create project directory: {}", project_dir.display()))?;

        let project = NativedoctorProject {
            name: name.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            requests: Vec::new(),
        };

        let yaml_path = project_dir.join(".nativedoctor");
        let yaml = serde_yaml::to_string(&project).context("failed to serialize project")?;
        std::fs::write(&yaml_path, yaml).context("failed to write .nativedoctor")?;

        self.add_recent(name, &project_dir.to_string_lossy()).await?;

        Ok(project)
    }

    pub fn load_project(path: &Path) -> anyhow::Result<NativedoctorProject> {
        let yaml_path = if path.is_dir() {
            path.join(".nativedoctor")
        } else {
            path.to_path_buf()
        };

        let yaml = std::fs::read_to_string(&yaml_path)
            .with_context(|| format!("failed to read project file: {}", yaml_path.display()))?;

        serde_yaml::from_str(&yaml).context("failed to parse .nativedoctor")
    }

    pub fn save_project(path: &Path, project: &NativedoctorProject) -> anyhow::Result<()> {
        let yaml_path = if path.is_dir() {
            path.join(".nativedoctor")
        } else {
            path.to_path_buf()
        };

        let yaml = serde_yaml::to_string(project).context("failed to serialize project")?;
        std::fs::write(&yaml_path, yaml).context("failed to write .nativedoctor")?;

        Ok(())
    }
}
