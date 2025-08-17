use nd_core::{direction::Direction, iterself::CircularIterSelf};
use strum::IntoEnumIterator;

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
    RotateResponseTab(Direction),
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

impl CircularIterSelf for RequestTab {
    fn all() -> Vec<Self>
    where
        Self: Sized + PartialEq + Clone,
    {
        return RequestTab::iter().collect::<Vec<Self>>();
    }
}

#[derive(strum::Display, Default, Clone, PartialEq, Debug, strum::EnumIter)]
pub enum ResponseTab {
    Headers,
    #[default]
    Body,
    Log,
}

impl CircularIterSelf for ResponseTab {
    fn all() -> Vec<Self>
    where
        Self: Sized + PartialEq + Clone,
    {
        return ResponseTab::iter().collect::<Vec<Self>>();
    }
}

#[derive(strum::Display)]
pub enum ApplicationEvent {
    Input(ratatui::crossterm::event::Event),
    HttpRequestCallCompleted(reqwest::Result<reqwest::blocking::Response>),
}
