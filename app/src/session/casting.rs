use nativedoctor_core::schema::roots::RequestRootSchema;
use std::{collections::HashMap, path::PathBuf};
use uuid::Uuid;

use nativedoctor_core::schema::{
    calls::CallSchema,
    project::ProjectDefinationSchema,
    roots::{EnvironmentRootSchema, ProjectRootSchema},
};

use crate::session::{EnvironmentDefination, RequestDefination, Session, VariableValue};

impl Session {
    pub fn to_fs_schema(
        &self,
    ) -> (
        ProjectRootSchema,
        Vec<EnvironmentRootSchema>,
        Vec<RequestRootSchema>,
    ) {
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
            calls: CallSchema {
                main: vec![],
                overrides: HashMap::new(),
            },
        };

        let environment_root_schemas: Vec<EnvironmentRootSchema> = self
            .custom_environments
            .iter()
            .map(|e| e.to_environment_schema())
            .collect::<Vec<EnvironmentRootSchema>>();

        let request_root_schemas: Vec<RequestRootSchema> = self
            .requests
            .iter()
            .map(|r| r.to_request_schema())
            .collect::<Vec<RequestRootSchema>>();

        return (
            project_root_schema,
            environment_root_schemas,
            request_root_schemas,
        );
    }

    pub fn cast_environment_schema_to_session_type(
        path: PathBuf,
        root: EnvironmentRootSchema,
    ) -> EnvironmentDefination {
        return EnvironmentDefination {
            id: Uuid::new_v4(),
            ref_id: root.ref_id,
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
        };
    }

    pub fn cast_request_schema_to_session_type(
        path: PathBuf,
        root: RequestRootSchema,
    ) -> RequestDefination {
        let config = root.config.unwrap_or_default();

        return RequestDefination {
            id: Uuid::new_v4(),
            ref_id: root.ref_id,
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
        };
    }

    pub fn cast_calls_schema_to_session_type(root: CallSchema) -> HashMap<String, Vec<String>> {
        let mut calls = root.overrides.clone();
        calls.insert(root.main.join("main"), root.main);
        return calls;
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
            calls: Session::cast_calls_schema_to_session_type(root.calls),
            current_env,
            custom_environments: environments,
        };
    }
}
