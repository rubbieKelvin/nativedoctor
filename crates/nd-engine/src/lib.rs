//! Sends requests and reports progress as a stream of events, which both the CLI and the desktop
//! app can consume.

mod event;
mod http;

pub use event::{Error, Event};
pub use http::send;
