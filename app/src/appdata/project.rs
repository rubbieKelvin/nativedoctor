use dioxus::{
    hooks::{use_context, use_context_provider},
    signals::Signal,
};

use nd_core::executor::runner::{Runner, ScriptEngine};

#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub name: String,
    pub path: String,
    // pub runner: Runner,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProjectManager {
    pub current: Option<Project>,
    // pub recent_projects: Vec<Project>,
}

impl ProjectManager {
    pub fn provide() {
        use_context_provider::<Signal<ProjectManager>>(|| {
            Signal::new(ProjectManager { current: None })
        });
    }

    pub fn inject() -> Signal<ProjectManager> {
        return use_context::<Signal<ProjectManager>>();
    }

    pub fn open(&mut self, path: String) -> Result<(), String> {
        let project = Runner::new(&path, None, ScriptEngine::None, true);

        self.current = Some(Project {
            name: path.split("/").last().unwrap().to_string(),
            path,
            // runner: project,
        });

        Ok(())
    }
}
