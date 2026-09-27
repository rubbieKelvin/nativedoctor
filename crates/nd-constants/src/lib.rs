//! Shared literals for **nativedoctor** crates: HTTP client identity, tracing targets, and CLI
//! filters. Use this crate so runtime (`nd-core`, `nd-engine`), CLI (`nd-cli`), and desktop
//! (`nd-desktop`) stay aligned.

/// Package version of this crate (matches workspace via `version.workspace` in `Cargo.toml`).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// `User-Agent` header value for outbound HTTP (`nativedoctor/<VERSION>`).
pub const USER_AGENT: &str = concat!("nativedoctor/", env!("CARGO_PKG_VERSION"));

/// The OpenAPI specification extension nativedoctor stores concrete "what to send" values in
/// (path/query/header/body/timeout/etc.) on an operation, since OpenAPI itself only describes API
/// shape, not one tool's scratch state. Written to disk as `x-nativedoctor-request` — `oas3`
/// strips/adds the `x-` prefix itself, so this constant deliberately omits it.
pub const ND_EXTENSION_REQUEST: &str = "nativedoctor-request";

/// Workspace manifest file name (tracks the sidebar tree, environments, and OpenAPI doc list).
pub const WORKSPACE_MANIFEST_FILE: &str = "nativedoctor.yaml";

/// [`tracing`] target for Rhai post-script log lines.
pub const TRACING_TARGET_RHAI: &str = "nativedoctor::rhai";

/// Lowercase HTTP header name used for computed `User-Agent`.
pub const HTTP_HEADER_USER_AGENT: &str = "user-agent";

/// Lowercase HTTP header name used for computed `Accept`.
pub const HTTP_HEADER_ACCEPT: &str = "accept";

/// Lowercase HTTP header name for `Content-Type`.
pub const HTTP_HEADER_CONTENT_TYPE: &str = "content-type";

/// Default `RUST_LOG`-style filter when the CLI passes `--verbose` and the env var is unset.
pub const CLI_TRACING_FILTER_VERBOSE: &str = "nd_core=debug,nd_engine=debug,warn";

/// Default tracing filter when not verbose and `RUST_LOG` is unset.
pub const CLI_TRACING_FILTER_QUIET: &str = "warn";
