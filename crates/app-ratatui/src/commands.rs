#[derive(Debug, Clone, strum::Display)]
pub enum ActiveInput {
    Url,
}

pub enum XDirection {
    Left,
    Right
}

pub enum Command {
    Quit,
    StartEditing(ActiveInput),
    StopEditing,
    RotateHttpMethod,
    RotateRequestTab(XDirection)
}
