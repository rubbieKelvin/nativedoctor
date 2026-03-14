pub mod http;
pub mod project;
pub mod sequence;

/// Content of a resource file (HTTP or Sequence). Returned as JSON from read_resource_file.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(tag = "type")]
pub enum ResourceFileContent {
    #[serde(rename = "http")]
    Http(http::HttpResourceFile),
    #[serde(rename = "sequence")]
    Sequence(sequence::SequenceResourceFile),
}
