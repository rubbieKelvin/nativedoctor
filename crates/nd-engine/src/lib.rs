//! Sends requests and reports progress as a stream of events, which both the CLI and the desktop
//! app can consume.

mod cookies;
mod event;
mod http;
pub mod report;
pub mod session;

pub use cookies::{CookieInfo, CookieJar};
pub use event::{Error, Event};
pub use http::send;
pub use session::{Conclusion, FileScope, ResponseHead, Session};
