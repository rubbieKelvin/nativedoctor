module static;

/// Supplements generated definitions for **imported request files** (`.json` / `.yaml` / `.yml`).
///
/// Each imported request module exposes **`invoke`** — not `call`, because `call` is a Rhai reserved
/// keyword in qualified paths such as `module::call(...)`.
///
/// Pass a map to override `${VAR}` placeholders for that HTTP request; overrides take precedence
/// over runtime variables from `env()` / `set()` / the process environment.
///
/// Example:
/// ```rhai
/// import "api/get_user.json" as get_user;
/// let r = get_user::invoke(#{ user_id: "42" });
/// assert(r.status == 200);
/// ```
fn invoke(kwargs: Map) -> Map;

/// Same as `invoke(#{})` — run the request with no per-call variable overrides.
fn invoke() -> Map;

/// Stream the request, unlike invoke this one fetces bit by bit
fn stream(kwargs: Map) -> Map;

/// Same as `stream(#{})` — run the request with no per-call variable overrides.
fn stream() -> Map;
