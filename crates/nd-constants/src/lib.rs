//! Shared literals for **nativedoctor** crates: HTTP client identity, default document versions,
//! tracing targets, CLI filters, and generator placeholders. Use this crate so runtime (`nd-core`),
//! CLI (`nd-cli`), and tools (`ng-generate`) stay aligned.

pub mod urls;

/// Package version of this crate (matches workspace via `version.workspace` in `Cargo.toml`).
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// `User-Agent` header value for outbound HTTP (`nativedoctor/<VERSION>`).
pub const USER_AGENT: &str = concat!("nativedoctor/", env!("CARGO_PKG_VERSION"));

/// Default `version` field for request and sequence JSON/YAML documents when omitted.
pub const DOCUMENT_DEFAULT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Alias for request file schema default version.
pub const REQUEST_FILE_DEFAULT_VERSION: &str = DOCUMENT_DEFAULT_VERSION;

/// [`tracing`] target for Rhai post-script log lines.
pub const TRACING_TARGET_RHAI: &str = "nativedoctor::rhai";

/// Label for who initiated a Rhai log line (post-script); used in tracing and capture buffers.
pub const RHAI_LOG_INITIATOR: &str = "post_script";

/// Lowercase HTTP header name used for computed `User-Agent`.
pub const HTTP_HEADER_USER_AGENT: &str = "user-agent";

/// Lowercase HTTP header name used for computed `Accept`.
pub const HTTP_HEADER_ACCEPT: &str = "accept";

/// Lowercase HTTP header name for `Content-Type`.
pub const HTTP_HEADER_CONTENT_TYPE: &str = "content-type";

/// Default `RUST_LOG`-style filter when the CLI passes `--verbose` and the env var is unset.
pub const CLI_TRACING_FILTER_VERBOSE: &str = "nd_core=debug,warn";

/// Default tracing filter when not verbose and `RUST_LOG` is unset.
pub const CLI_TRACING_FILTER_QUIET: &str = "warn";

/// Placeholder base URL in generated OpenAPI request files when the spec has no `servers`.
pub const OPENAPI_GENERATE_BASE_URL_PLACEHOLDER: &str = "${BASE_URL}";
