use dioxus::{
    hooks::{use_context, use_context_provider},
    signals::{Signal, Writable},
};

use crate::{actions::ProjectActions, logic::project_reducer, states::ProjectState};

pub struct ProjectStateManager;

impl ProjectStateManager {
    pub fn provide() -> Signal<ProjectState> {
        return use_context_provider(|| Signal::new(ProjectState::new()));
    }

    pub fn inject() -> (
        Signal<ProjectState>,
        impl FnMut(ProjectActions) -> Result<(), String> + Clone,
    ) {
        let mut state = use_context::<Signal<ProjectState>>();
        let dispatch = move |action: ProjectActions| project_reducer(&mut *state.write(), action);

        return (state, dispatch);
    }
}
