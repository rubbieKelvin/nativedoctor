use dioxus::{
    hooks::{use_context, use_context_provider},
    signals::Signal,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub name: String,
    pub path: String,
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

    pub fn open(&mut self, path: String) {
        self.current = Some(Project {
            name: path.split("/").last().unwrap().to_string(),
            path,
        });
    }
}
