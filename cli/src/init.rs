use std::env::current_dir;

pub fn init(name: &str) {
    // TODO: gracefully handle errors
    let cwd = current_dir().unwrap();
    librustle::init::init_project(cwd, name).unwrap();
}
