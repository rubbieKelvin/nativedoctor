pub mod models;

pub struct Store {
    recents: Vec<models::RecentProject>,
}

impl Store {
    // loads a nd.json file in ~/.nativedoctor/
    pub fn init() {}
}
