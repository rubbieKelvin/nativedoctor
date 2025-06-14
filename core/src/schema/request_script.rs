use serde::Deserialize;

/// Represents the script section of a request.
#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct RequestScriptConfigSchema {
    pub post_request: Option<ScriptSchema>,
    pub pre_request: Option<ScriptSchema>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "snake_case", tag = "language")]
pub enum ScriptSchema {
    // TODO: Add support for javascript. for now, rhai seems the easiest to integrate
    // #[serde(rename = "lua")]
    // Lua { content: String },
    // #[serde(rename = "javascript")]
    // Javascript { content: String },
    #[serde(rename = "rhai")]
    Rhai { content: String },
}
