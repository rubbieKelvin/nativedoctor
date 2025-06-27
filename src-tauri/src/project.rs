use std::path::PathBuf;

use crate::core::{
    create_project_template,
    fs::FileObject,
    schema::roots::{ProjectRootSchema, RequestRootSchema},
};

#[tauri::command]
pub async fn create_project() -> Result<
    (
        FileObject<ProjectRootSchema>,
        Vec<FileObject<RequestRootSchema>>,
    ),
    String,
> {
    println!("Loading project");
    let (project, requests) = create_project_template("Untitled");
    Ok((
        FileObject::new(PathBuf::new(), project),
        requests
            .iter()
            .map(|r| FileObject::new(PathBuf::new(), r.clone()))
            .collect(),
    ))
}
