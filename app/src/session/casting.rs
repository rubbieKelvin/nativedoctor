use nativedoctor_core::schema::roots::RequestRootSchema;
use std::{collections::HashMap, path::PathBuf};
use uuid::Uuid;

use nativedoctor_core::schema::{
    project::ProjectDefinationSchema,
    roots::{EnvironmentRootSchema, ProjectRootSchema},
};

use crate::session::{EnvironmentDefination, RequestDefination, Session, VariableValue};

impl Session {
    pub fn to_fs_schema(&self) -> ProjectRootSchema {
        let project_root_schema = ProjectRootSchema {
            project: ProjectDefinationSchema {
                name: self.name.clone(),
                description: self.description.clone(),
                version: if self.version.is_empty() {
                    None
                } else {
                    Some(self.version.clone())
                },
            },
            calls: self.calls.clone(),
        };

        return project_root_schema;
    }

    pub fn cast_environment_schema_to_session_type(
        path: PathBuf,
        root: EnvironmentRootSchema,
    ) -> EnvironmentDefination {
        let filename = if path.try_exists().unwrap_or_default() {
            path.file_stem()
                .map(|s| s.to_str())
                .flatten()
                .map(|s| s.to_string())
                .unwrap_or_else(|| nanoid::nanoid!())
        } else {
            nanoid::nanoid!()
        };

        return EnvironmentDefination {
            id: Uuid::new_v4(),
            ref_id: filename,
            name: root.name,
            path: Some(path),
            variables: root
                .variables
                .iter()
                .map(|e| {
                    (
                        e.name.clone(),
                        VariableValue {
                            value: e.value.as_str().unwrap_or("").to_string(),
                            initial: e.value.as_str().unwrap_or("").to_string(),
                            sensitive: e.secret.clone(),
                            description: e.description.clone(),
                        },
                    )
                })
                .collect::<HashMap<String, VariableValue>>(),
            ..Default::default()
        };
    }

    pub fn cast_request_schema_to_session_type(
        path: PathBuf,
        root: RequestRootSchema,
    ) -> RequestDefination {
        let config = root.config.unwrap_or_default();
        let filename = if path.try_exists().unwrap_or_default() {
            path.file_stem()
                .map(|s| s.to_str())
                .flatten()
                .map(|s| s.to_string())
                .unwrap_or_else(|| nanoid::nanoid!())
        } else {
            nanoid::nanoid!()
        };

        return RequestDefination {
            id: Uuid::new_v4(),
            ref_id: filename,
            name: root.name,
            method: root.method,
            url: root.url,
            doc: root.doc,
            headers: root.headers.unwrap_or_default(),
            dependencies: config.require,
            timeout: config.timeout.unwrap_or(30000),
            retries: config.retries,
            query: root.query.unwrap_or_default(),
            body: root.body,
            class: config.class.unwrap_or("".to_string()),
            path: Some(path),
            ..Default::default()
        };
    }

    pub fn from_fs_schema(
        path: PathBuf,
        current_env: Option<String>,
        root: ProjectRootSchema,
        requests: Vec<RequestDefination>,
        environments: Vec<EnvironmentDefination>,
    ) -> Self {
        return Self {
            path: Some(path),
            name: root.project.name,
            description: root.project.description,
            version: root.project.version.unwrap_or("0.0.1".to_string()),
            requests,
            calls: root.calls.clone(),
            current_env,
            environments: environments,
            ..Default::default()
        };
    }
}
