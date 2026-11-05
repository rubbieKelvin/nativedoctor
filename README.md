# NativeDoctor

[![Made in Nigeria](https://img.shields.io/badge/made%20in-nigeria-008751.svg?style=flat-square)](https://github.com/acekyd/made-in-nigeria)
[![License](https://img.shields.io/github/license/rubbieKelvin/nativedoctor?style=flat-square)](LICENSE)

Postman for the command line. Collections are plain folders of YAML, so they live in Git.

```bash
cargo build --release -p nd-cli   # binary: target/release/nativedoctor
```

HTTP works today. WebSocket and gRPC are planned.

## Quick start

```bash
nativedoctor init my-api && cd my-api
nativedoctor run ping.request.yaml

# One-off request, with no file
nativedoctor send POST :3000/users name=John age:=29 X-Debug:1 page==2

# One-time changes to a saved request
nativedoctor run users/create.request.yaml name=Jane -e prod

# Save a one-off request (add --offline to save without sending)
nativedoctor send POST :3000/users name=John --save users/create.request.yaml
```

| Command | Does |
|---|---|
| `init [DIR]` | Create a collection |
| `run <FILE or FOLDER> [ITEM...]` | Run a request, or a folder of them in path order, stopping at the first failure |
| `send [METHOD] URL [ITEM...]` | Send a one-off request |
| `list [PATH]` | List requests |
| `check [PATH]` | Validate files and variables without using the network |

## Collections

```
my-api/
├── nativedoctor.yaml
├── environments/
│   └── dev.yaml
├── users/
│   └── get-user.request.yaml
└── .nativedoctor/          # never committed: secrets and captured values
```

Requests are found by looking for `*.request.yaml`. Commands find the collection by walking up from
the request file or current folder. A request file outside a collection still runs.

```yaml
# nativedoctor.yaml
name: My API
default_env: dev
exclude: ["drafts/**"]
vars:
  API_VERSION: v1
defaults:
  headers:
    x-client: nativedoctor
```

## Request files

```yaml
name: Log in
http:
  method: POST              # default: GET, or POST with a body
  url: ${BASE_URL}/login
  query: { verbose: true, tag: [a, b] }
  headers:
    x-trace: ${!uuidv4}
    accept: null            # null removes a default header
  body:
    json: { user: "${USERNAME}", password: "${PASSWORD}" }
  timeout: 10s              # default 30s, 0 disables it
  follow_redirects: true    # default false
  max_redirects: 5          # default 30
  verify_tls: true
capture:
  API_TOKEN: $.access_token # JSONPath, or header:Name
assert:
  status: [200, 201]
  headers: { content-type: application/json }   # value must contain this
  body: { $.user.active: true }
```

The body is one of `json`, `text`, `form`, `file` or `multipart`:

```yaml
body:
  multipart:
    title: My CV
    cv: { file: ./cv.pdf, type: application/pdf }
```

Files named in a request must be inside its collection. Files named on the command line can be
anywhere.

## Request items

| Item | Meaning |
|---|---|
| `Header:value` | Set a header |
| `Header:` | Remove a header |
| `Header;` | Send an empty header |
| `key==value` | Query parameter |
| `key=value` | JSON string field (or a form field with `--form`) |
| `key:=29` | Raw JSON value |
| `user[name]=x`, `tags[]=y`, `list[0]=z` | Nested JSON |
| `key=@file`, `key:=@file.json`, `Header:@file`, `key==@file` | Value from a file |
| `cv@file;type=application/pdf` | File upload, with `--form` or `--multipart` |

Use `\` to escape a separator in a name (`foo\==bar`), and `--` before items that start with `-`.

`--body` sends a raw body: text, `@file`, or `-` for stdin. Stdin is only read when you ask for it.

## Variables and secrets

`${NAME}` is a variable and `$${NAME}` is a literal. `${!name}` is generated fresh each time:
`uuidv4`, `nanoid`, `now`, `yesterday`, `tomorrow`, `random_date_past`, `random_date_future`,
`random_int`, `random_bool`, `color`, `random_name`, `random_username`, `random_email`,
`random_phone`, `random_words`, `random_paragraph`.

```yaml
# environments/dev.yaml: committed, so it only holds secret names
vars:
  BASE_URL: https://dev.example.com
  USER_ID:                  # declared, value comes from elsewhere
secrets:
  - API_TOKEN
```

Highest priority first:

1. `--var NAME=value`
2. Process environment, for declared names only
3. Captured values, in `.nativedoctor/state/<env>.yaml`
4. `.nativedoctor/secrets/<env>.env`, for declared secrets only
5. Environment `vars`
6. Collection `vars`

Undeclared process variables can't be read, so a request in a cloned collection can't send your
`AWS_SECRET_ACCESS_KEY` anywhere. Values set by `--var` or the process environment aren't replaced
by captures.

Secret values show as `***` in headers, request bodies, errors, `--offline`, `--json` and terminal
output. A captured value is only treated as secret if its name is listed under `secrets`.

**Raw response bodies aren't redacted.** When output is piped, the body passes through byte for byte
so that `| jq` and `> file.png` work.

## Output

A terminal gets formatted, coloured headers and body. A pipe gets only the raw body.

| Flag | Does |
|---|---|
| `--print WHAT` | `H`, `B` request head and body; `h`, `b` response head and body; `m` timing |
| `-v` | Full exchange, same as `--print=HBhbm` |
| `--offline` | Build and print the request without sending it |
| `--stream`, `-S` | Print the body as it arrives (automatic for `text/event-stream`) |
| `--json` | One JSON result. With `--stream`, one JSON event per line |
| `--pretty all\|none` | Override the terminal or pipe default |

## Exit codes

Any HTTP response exits 0 unless you pass `--fail` or a check fails.

| Code | Meaning |
|---|---|
| 0 | Success |
| 1 | Error: connection, invalid file, missing variable, bad usage, failed capture |
| 2 | Timeout |
| 3, 4, 5 | 3xx, 4xx or 5xx response, with `--fail` |
| 6 | Too many redirects |
| 7 | Failed assertion |

## Development

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

`nd-core` handles files, variables and request preparation with no networking. `nd-engine` sends
requests and emits a stream of events. `nd-cli` is the binary.
