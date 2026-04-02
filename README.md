# nativedoctor

**nativedoctor** is a file-driven HTTP client: you describe requests in **JSON** or **YAML**, run **Rhai** scripts that can import other scripts and request files, and execute everything from the command line or embed **`nd-core`** in your own Rust code. It fits API exploration, smoke tests, and light automation without ad-hoc shell `curl` scripts.

- **Request files** (`.json`, `.yaml`, `.yml`): one HTTP call per file (method, URL, query, headers, body).
- **Rhai scripts** (`.rhai`): sandboxed scripting with `env` / `set`, `assert`, `log`, optional `persist`, and **`import`** of other `.rhai` modules and request files.
- **Template expansion**: `${VAR}` from the runtime map (process env, `--env` files, Rhai `set`, optional persistence); **dynamic** `${!name}` helpers (see below).
- **Imported requests**: `import "api.json" as api` then **`api::invoke(#{ user_id: "42" })`** to run that HTTP request with per-call variable overrides (overrides win over the runtime map).
- **OpenAPI 3.0.x**: generate starter request files from a spec (`generate`).
- **Rhai definition files**: emit **`.d.rhai`** stubs for editors / language servers (`definitions`).
- **Web UI** (optional): local **Axum** server + **Vue 3** / **TypeScript** / **Tailwind CSS v4** SPA, embedded in the binary with **`rust-embed`** (`web`).

The CLI binary is **`nativedoctor`**. Core logic lives in **`nd-core`**; **`nd-generate`** implements OpenAPI import; **`nd-constants`** holds shared literals; **`nd-web`** serves the browser UI and JSON API.

---

## Requirements

- **Rust** (2021 edition), stable toolchain, to build from source.
- **pnpm** on `PATH` when building **`nd-web`**: the crate’s `build.rs` runs `pnpm install` and `pnpm build` in `crates/nd-web/frontend` so assets can be embedded. To skip that (e.g. CI with a prebuilt `frontend/dist/`), set **`ND_WEB_SKIP_FRONTEND_BUILD=1`** (see [`nd-web` crate](crates/nd-web)).
- Network access for real HTTP calls (optional: **`--no-network-io`** expands and prints only).

---

## Install

### From source (workspace root)

```bash
cargo build --release -p nd-cli
```

The binary is at `target/release/nativedoctor`. Add that directory to your `PATH`, or run it by path.

Building **`nd-web`** as a dependency pulls in the frontend build via `build.rs` (requires **pnpm** unless you use **`ND_WEB_SKIP_FRONTEND_BUILD=1`** with an existing `crates/nd-web/frontend/dist/`).

### Prebuilt archives

If this repository publishes **GitHub Releases**, attaching archives is automated (see [Release binaries (CI)](#release-binaries-ci)). Download the archive for your OS and place the `nativedoctor` binary on your `PATH`.

---

## Quick start

**Run a request file** (either form is equivalent):

```bash
nativedoctor run my-request.yaml
nativedoctor my-request.yaml
```

**Expand and print only** (no network I/O):

```bash
nativedoctor run my-request.yaml --no-network-io
```

**Run a Rhai script** (`.rhai`):

```bash
nativedoctor run my-flow.rhai
```

**Run several files** (each path is a request or a script, in order):

```bash
nativedoctor run ./a.yaml ./b.rhai ./c.yaml
```

**Keep one runtime across files** (Rhai `set` / `${VAR}` visible across runs):

```bash
nativedoctor run --retain-runtime ./login.yaml ./fetch.rhai
```

**Scaffold a request file**:

```bash
nativedoctor new examples/hello.yaml
nativedoctor new -u https://httpbin.org/get -n Demo examples/demo.yaml
```

**Generate requests from OpenAPI 3.0.x**:

```bash
nativedoctor generate -i openapi.json -o ./generated --format yaml
```

**Emit Rhai `.d.rhai` definitions** (IDE / LSP checkout [Rhai metadata](https://rhai.rs/book/engine/metadata/index.html)):

```bash
nativedoctor definitions --out-dir .rhai/definitions
# or a single merged file:
nativedoctor definitions --out-file nativedoctor.d.rhai
```

**Browse and run request files in a browser** (local HTTP UI; see [`web`](#web)):

```bash
nativedoctor web
nativedoctor web ./api ./scripts
```

---

## CLI reference

Global options apply where noted.

| Option | Description |
|--------|-------------|
| `-v`, `--verbose` | More detailed output; default tracing filter `nd_core=debug` unless `RUST_LOG` is set. |
| `--env <FILE>` | Merge variables from a dotenv-style file into the runtime ([dotenvy](https://docs.rs/dotenvy); repeatable; later files override earlier). |
| `--persistence-file <FILE>` | Optional persistence file for Rhai `persist()` (see `RuntimeEnv` in `nd-core`). |
| `--no-network-io` | **Request files:** expand and print the prepared request; no HTTP. **Rhai scripts:** still run; `invoke()` on imported requests uses dry-run behavior (no real HTTP) when this flag is set. **Web UI:** request “Send” becomes expand-only; script behavior follows `nd-core` options. |

### `run`

```text
nativedoctor run [OPTIONS] <FILE>...
```

| Option | Description |
|--------|-------------|
| `--retain-runtime` | Build the runtime once and reuse it for every file in this invocation (default: clear runtime between files). |
| `<FILE>...` | One or more paths: `.json`, `.yaml`, `.yml` (request), or `.rhai` (script). |

**Shorthand:** with no subcommand, a single positional `FILE` runs like `run` with one path.

### `definitions`

```text
nativedoctor definitions (--out-dir <DIR> | --out-file <FILE>)
```

Writes Rhai definition stubs (`.d.rhai`) for autocompletion / language servers: Rhai builtins plus nativedoctor globals (`env`, `set`, `assert`, `log`, `persist`), plus a small supplement for **`invoke`** on imported request modules. Use **`--out-dir`** for the usual multi-file layout or **`--out-file`** for one merged file.

### `web`

```text
nativedoctor web [OPTIONS] [DIR]...
```

Starts a local **Axum** server: JSON API under **`/api`** and a **Vue** SPA (static assets embedded at compile time). Each **`DIR`** is scanned **non-recursively** for top-level `*.json` / `*.yaml` / `*.yml` request files (validated before listing) and `*.rhai` scripts. Omit **`DIR`** to use the current directory.

| Option | Description |
|--------|-------------|
| `--bind <ADDR>` | Listen address (default **`127.0.0.1:8080`**). |
| `[DIR]...` | One or more workspace roots (default **`.`** when omitted). |

**Security:** treat as a **local development** tool. Anyone who can reach the bind address can trigger outbound HTTP to URLs in your files and run configured Rhai. Prefer loopback unless you understand the exposure.

### `generate`

```text
nativedoctor generate -i <SPEC> -o <DIR> [--format yaml|json]
```

Reads **OpenAPI 3.0.x** (JSON or YAML). **OpenAPI 3.1** and some `$ref` patterns may be rejected. Writes one request file per operation under `DIR`.

### `new`

```text
nativedoctor new [--url <URL>] [--name <NAME>] <PATH>
```

Writes a starter **request** document. Extension must be `.json`, `.yaml`, or `.yml`. Refuses to overwrite an existing file.

---

## Request files

A request file wraps an `HttpRequestSpec` under a top-level `request` key. Extensions: **`.json`**, **`.yaml`**, **`.yml`**.

Minimal YAML example:

```yaml
version: "0.0.0"
name: Example GET
request:
  method: GET
  url: https://httpbin.org/get
  query:
    foo: bar
  headers: {}
  body: null
  follow_redirects: true
  verify_tls: true
```

Useful fields (non-exhaustive):

| Area | Notes |
|------|--------|
| `method` | Any case; normalized when sending. |
| `url` | May contain `${VAR}` placeholders. |
| `query` / `headers` | String maps; values may use `${VAR}`. |
| `body` | Omitted or `null` for no body. JSON object/array → JSON body; string → text. Structured bodies support explicit `type` (e.g. `json`, `text`, `binary`, …). |
| `timeout_secs` | Optional; default from schema in `nd-core`. |
| `follow_redirects` | Default `true`. |
| `verify_tls` | Default `true`; set `false` only for local/dev. |

**JSON Schema:** `RequestFile::schema()` returns a JSON Schema document for tooling.

---

## Rhai scripts

Scripts run with a locked-down Rhai engine: **no arbitrary filesystem or network APIs** inside Rhai; HTTP happens only via **imported request files** and **`invoke`**.

### Built-ins

| Symbol | Role |
|--------|------|
| `env(key)` | Read a string from the runtime map; missing keys → `()`. |
| `set(key, value)` | Stringify `value` and store in the runtime map (visible to `${VAR}` and `env()`). |
| `assert(condition, message)` | Fail evaluation if `condition` is false. |
| `log(level, message)` | Log (tracing; optional `Logger` capture from the CLI). |
| `persist(key, value)` | If a persistence file is configured, update runtime and the persist file. |

### Imports

```rhai
import "helpers.rhai" as helpers;
import "get_user.json" as get_user;

let r = get_user::invoke(#{ user_id: "42" });
assert(r.status == 200, "bad status");
```

- Import paths are resolved **relative to the importing script’s directory** (or the main script’s directory for top-level imports).
- **`invoke()`** with no arguments uses only the runtime map for `${VAR}`.
- **`invoke(#{ name: value, ... })`** merges overrides for that request only; **overrides take precedence** over `env()` / `set()` / process env for `${VAR}` in that request.

Rhai reserves **`call`** as a keyword in qualified paths like `module::call(...)`, so imported request modules use **`invoke`**, not `call`.

---

## Environment and `${VAR}` templates

Before send, strings in URLs, query values, headers, and JSON/text bodies expand **`${IDENT}`** (letters, digits, underscore; see `nd-core`).

Dynamic placeholders use **`${!name}`** (fresh value per expansion). Examples include `uuidv4`, `nanoid`, `random_username`, `now`, etc. Unknown names error at expansion time.

By default the CLI seeds the runtime map from the **process environment**, then merges each **`--env`** file. The **`RuntimeEnv`** API also supports an isolated/empty starting map for embedded use (`nd-core`).

---

## OpenAPI generation

**Supported:** OpenAPI **3.0.x**.

**Not supported (today):** OpenAPI **3.1** (may be rejected), some `$ref` patterns.

Generated URLs may use **`${BASE_URL}`** when the spec has no `servers` entry. Path `{param}` segments become **`${param}`** template syntax.

---

## Using the library (`nd-core`)

Path or crates.io dependency on **`nd-core`**. Typical entry points:

- **Load / expand:** `RequestFile::from_file`, `HttpRequestSpec::expand` / `expand_with_overrides`, template helpers in `nd_core::utils::template`
- **Execute:** `RequestFile::execute` / `execute_with_overrides`, `ExecutionResult`
- **Rhai:** `run_rhai_script`, `RhaiScriptRunOptions`, definition export helpers in `nd_core::rhai`
- **Discovery:** `list_request_paths`, `list_rhai_paths`, `partition_valid_request_paths`

Install a **`tracing`** subscriber if you want structured logs from the core crate.

---

## Development

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all
cargo clippy --workspace -- -D warnings
```

**Frontend (`nd-web`):** package manager is **pnpm** (`crates/nd-web/frontend`). The **`nd-web`** crate runs `pnpm install` / `pnpm build` from **`build.rs`** before compiling so the SPA is embedded. For a fast iteration loop on the UI alone:

```bash
cd crates/nd-web/frontend && pnpm install && pnpm dev
```

(run the Rust server separately, with Vite proxying `/api` to it, per `vite.config.ts`).

---

## Release binaries (CI)

Publishing a **GitHub Release** (not draft-only) triggers `.github/workflows/release.yml`, which builds **`nativedoctor`** for Linux x86_64, Windows x86_64, macOS Apple Silicon, and macOS Intel, then uploads archives to that release. Builds use the release **tag** as the checkout ref so assets match the tagged sources.

Release jobs must have **Node + pnpm** (or pre-seed `frontend/dist` and set **`ND_WEB_SKIP_FRONTEND_BUILD=1`**) if the release build includes **`nd-web`**.

---

## Contributing

Issues and pull requests are welcome. When changing behavior, update this README, **PROJECT.md**, and any affected `///` / `//!` documentation in the crates you touch.
