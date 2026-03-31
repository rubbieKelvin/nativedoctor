//! Server functions (`#[server]`) backing the web UI.

pub mod list;
pub mod run_request;
pub mod run_sequence;
pub mod types;
pub use types::CatalogEntry;

pub use list::list_catalog;
pub use run_request::run_request_file;
pub use run_sequence::run_sequence_file;
