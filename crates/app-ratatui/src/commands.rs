#[derive(Debug, Clone)]
pub enum ActiveInput {
    Url,
}

pub enum Command {
    Quit,
    StartEditing(ActiveInput),
    StopEditing,
}
