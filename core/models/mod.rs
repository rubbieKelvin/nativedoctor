use std::path::PathBuf;

use serde::Serialize;

pub mod env_root;
pub mod project_root;
pub mod request_root;

#[derive(Clone, PartialEq, Serialize)]
pub struct FileObject<T: Clone + PartialEq + Serialize> {
    object: T,
    path: Option<PathBuf>,
}
