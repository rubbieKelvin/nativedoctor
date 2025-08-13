use ratatui::{style::Stylize, text::Span};

#[derive(Debug, Clone, strum::Display)]
pub enum ActiveInput {
    RequestUrl,
    RequestTitle,
}

pub enum Direction {
    Left,
    Right,
}

impl Direction {
    /// Applies the direction to a `usize` value, wrapping it around a given limit.
    ///
    /// This method moves the value `i` one step in the specified direction. The
    /// operation is circular, meaning that moving left from `0` results in `limit - 1`,
    /// and moving right from `limit - 1` results in `0`. This is achieved using
    /// the `rem_euclid` method to handle wrapping correctly for both directions.
    pub fn apply_usize(&self, i: &mut usize, limit: usize) {
        *i = match self {
            Direction::Left => {
                if *i == 0 {
                    limit-1
                } else {
                    *i - 1
                }
            }
            Direction::Right => {
                if *i == limit-1 {
                    0
                } else {
                    *i + 1
                }
            }
        }
        .rem_euclid(limit);
    }
}

pub enum Command {
    Quit,
    StartEditing(ActiveInput),
    StopEditing,
    RotateHttpMethod(Direction),
    RotateRequestTab(Direction),
    ToggleReqeustOutputPane,
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
