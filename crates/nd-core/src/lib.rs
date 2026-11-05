//! Collections, request files, variables and request preparation. No networking.

pub mod check;
pub mod collection;
mod dynamic;
pub mod error;
pub mod evaluate;
pub mod items;
pub mod manifest;
pub mod pairs;
pub mod prepare;
pub mod request;
pub mod state;
pub mod template;
pub mod vars;

pub use collection::Collection;
pub use error::{Error, Result};
pub use pairs::Pairs;
pub use prepare::{prepare, PrepareContext, PreparedBody, PreparedPart, PreparedRequest};
pub use request::{Assertions, Body, HttpSpec, Part, RequestFile};
pub use vars::{Redactor, Variables};
