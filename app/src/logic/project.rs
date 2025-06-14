use std::path::Path;

use nd_core::schema::root::RootSchema;

use crate::{actions::ProjectActions, states::ProjectState};

fn load_file(state: &mut ProjectState, path: String) -> Result<(), String> {
    let path_ref = Path::new(&path);
    let root_schema = RootSchema::load(path_ref, None).map_err(|e| e.to_string())?;

    state.path = Some(path);
    state.content = Some(root_schema);
    return Ok(());
}

pub fn project_reducer(state: &mut ProjectState, action: ProjectActions) -> Result<(), String> {
    match action {
        ProjectActions::LoadFile(path) => load_file(state, path),
    }
}
