use models::direction::Direction;

#[derive(Debug, Clone, strum::Display)]
pub enum ActiveInput {
    RequestUrl,
    RequestTitle,
}

pub enum Command {
    Quit,
    StartTextEditing(ActiveInput),
    AbortTextEditing,
    FinishTextEditing,
    RotateHttpMethod,
    RotateRequestTab(Direction),
    ToggleRequestOutputPane,
    SendRequest,
}

#[derive(Debug, Default, Clone)]
pub enum InputState {
    Editing {
        which: ActiveInput,
    },
    #[default]
    Normal,
}

#[derive(strum::Display, Default, Clone, PartialEq, Debug, strum::EnumIter)]
pub enum RequestTab {
    Params,
    Header,
    Auth,
    #[default]
    Body,
    Doc,
    Script,
}

#[derive(strum::Display, Default, Clone, PartialEq, Debug, strum::EnumIter)]
pub enum ResponseTab {
    Headers,
    #[default]
    Body,
    Log,
}

pub enum ApplicationEvent {
    Input(ratatui::crossterm::event::Event),
    HttpRequestCallCompleted(reqwest::Result<reqwest::blocking::Response>),
}
