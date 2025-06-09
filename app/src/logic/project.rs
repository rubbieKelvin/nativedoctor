use crate::{actions::ProjectActions, states::ProjectState};

fn load_file(state: &mut ProjectState, path: String) -> Result<(), String> {
    state.load_file(path)?;
    return Ok(());
}

pub fn project_reducer(state: &mut ProjectState, action: ProjectActions) -> Result<(), String> {
    match action {
        ProjectActions::LoadFile(path) => load_file(state, path),
    }
}
