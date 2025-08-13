#[derive(Debug, Clone, strum::Display)]
pub enum ActiveInput {
    RequestUrl,
    RequestTitle,
}

pub enum XDirection {
    Left,
    Right,
}

pub enum Command {
    Quit,
    StartEditing(ActiveInput),
    StopEditing,
    RotateHttpMethod,
    RotateRequestTab(XDirection),
    ToggleReqeustOutputPane,
}
