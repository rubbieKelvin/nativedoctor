use models::direction::Direction;
use ratatui::{style::Stylize, text::Span};

#[derive(Debug, Clone, strum::Display)]
pub enum ActiveInput {
    RequestUrl,
    RequestTitle,
}

pub enum Command {
    Quit,
    StartEditing(ActiveInput),
    StopEditing,
    RotateHttpMethod(Direction),
    RotateRequestTab(Direction),
    ToggleRequestOutputPane,
    SendRequest
}

#[derive(Debug, Default, Clone)]
pub enum InputState {
    Editing {
        which: ActiveInput,
    },
    #[default]
    Normal,
}

#[derive(Debug, Default, Clone, strum::Display, PartialEq, strum::EnumIter)]
pub enum RequestMethod {
    #[default]
    Get,
    Delete,
    Post,
    Patch,
    Put,
    Head,
    Option,
}

impl RequestMethod {
    pub fn span<'a>(&self) -> Span<'a> {
        let s = self.to_string().to_uppercase();
        match self {
            Self::Get => s.green(),
            Self::Delete => s.red(),
            Self::Post => s.blue(),
            Self::Patch => s.magenta(),
            Self::Put => s.yellow(),
            Self::Head => s.gray(),
            Self::Option => s.gray(),
        }
    }
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
    Input(ratatui::crossterm::event::Event)
}
