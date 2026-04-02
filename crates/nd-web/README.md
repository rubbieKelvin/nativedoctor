# nd-web

Local **web UI** crate for **nativedoctor**, built with **Dioxus 0.7**. It is intended to list and run top-level request files from configured directories.

## Status

Integration is **incomplete**: some server paths may still be stubbed or contain `todo!()`. Prefer the **`nativedoctor` CLI** for reliable runs until the web stack is finished.

## Running (from workspace root)

The **`nativedoctor`** binary (from `nd-cli`) includes a `web` subcommand:

```bash
cargo run -p nd-cli -- web --dir . --bind 127.0.0.1:8080
```

See the root [**README.md**](../../README.md) for options and security notes (bind address, outbound HTTP).

## Development

This crate follows a typical Dioxus fullstack layout (`src/`, `views/`, server functions). For Dioxus-specific tooling (`dx serve`, Tailwind, etc.), refer to the [Dioxus book](https://dioxuslabs.com/learn/0.7/getting_started) and the upstream project templates.
