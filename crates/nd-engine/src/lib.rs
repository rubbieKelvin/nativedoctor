//! HTTP execution, Rhai runtime, and run timeline for **nativedoctor**. Builds on `nd-core`'s
//! OpenAPI-backed model; this is the crate the CLI, desktop app, and (later) backend/agents all
//! call into, so results never differ between them ("one engine everywhere").

pub mod error;
pub mod execute;
pub mod rhai;
pub mod stream;

pub use error::{Error, Result};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use nd_core::model::RequestRef;
use nd_core::Workspace;

use execute::types::ExecutionResult;
use stream::Session;

/// Resolve `request` against `workspace` and send it, emitting timeline events on `session`.
/// This is the entry point CLI and desktop share.
pub async fn run_request(
    workspace: &Workspace,
    request: &RequestRef,
    session: Arc<Mutex<Session>>,
    var_overrides: Option<&HashMap<String, String>>,
    stream_body: bool,
) -> Result<ExecutionResult> {
    let resolved = workspace.resolve(request).map_err(Error::Core)?;
    let name = workspace
        .doc(&request.doc)
        .and_then(|doc| doc.operation(&request.path, request.method).ok())
        .and_then(|op| op.summary.clone());

    execute::execute_request(&resolved, session, var_overrides, name, stream_body).await
}
