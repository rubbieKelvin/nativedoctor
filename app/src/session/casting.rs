use std::collections::HashMap;

use nativedoctor_core::schema::{
    calls::CallSchema, env::EnvironmentVariableSchema, project::ProjectDefinationSchema,
    roots::ProjectRootSchema,
};

use crate::session::Session;

impl Session {
    fn cast_to_project_defination_schema(&self) -> ProjectDefinationSchema {
        return ProjectDefinationSchema {
            name: self.name.clone(),
            description: self.description.clone(),
            version: if self.version.is_empty() {
                None
            } else {
                Some(self.version.clone())
            },
        };
    }

    fn cast_to_environment_schema(&self) -> HashMap<String, EnvironmentVariableSchema> {
        // TODO: Complete this
        return HashMap::new();
    }

    fn cast_to_call_schema(&self) -> CallSchema {
        return CallSchema {
            main: vec![],
            overrides: HashMap::new(),
        };
    }

    pub fn cast_to_project_root_schema(&self) -> ProjectRootSchema {
        let project = self.cast_to_project_defination_schema();
        let env = self.cast_to_environment_schema();
        let calls = self.cast_to_call_schema();

        return ProjectRootSchema {
            project,
            calls,
            env,
        };
    }
}
