use std::env::current_dir;

pub fn init(name: &str) {
    let cwd = current_dir().unwrap();
    rustle_api::init::init_project(cwd, name).unwrap();
}
