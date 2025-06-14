use std::path::Path;
use crate::{actions::ProjectActions, states::{ProjectContentLoadingState, ProjectState}};

fn load_file(state: &mut ProjectState, path: String) -> Result<(), String> {
    let path_ref = Path::new(&path);
    state.content = ProjectContentLoadingState::Loading(path_ref.to_path_buf());
    return Ok(());
}

pub fn project_reducer(state: &mut ProjectState, action: ProjectActions) -> Result<(), String> {
    match action {
        ProjectActions::LoadFile(path) => load_file(state, path),
    }
}
