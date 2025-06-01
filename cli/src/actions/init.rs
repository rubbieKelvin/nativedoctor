use std::env::current_dir;

pub fn init(name: &str) {
    let cwd = current_dir().unwrap();
    rustle_core::init::init_project(cwd, name).unwrap();
}
