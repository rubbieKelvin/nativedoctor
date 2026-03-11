# Native Doctor — Agent Guide

This document describes the **Native Doctor** project so that AI agents and contributors can understand its purpose, core concepts, and current state. The product is built **incrementally**; not all features below exist yet.

---

## What Is Native Doctor?

Native Doctor is an **API client and resource runner** similar to [Postman](https://www.postman.com/) or [HTTPie](https://httpie.io/), with one central difference:

- **Each collection is a real folder** on the local filesystem.
- **Each resource** (the thing you run or group) **is represented by a file or folder** inside that collection—e.g. a JSON file for a call, or a subfolder for grouping.

So collections and resources are **file-based and version-control friendly**: you can manage them with Git, edit them in any editor, and share them by sharing a directory.

---

## Tech Stack

| Layer        | Technology |
|-------------|------------|
| Desktop app | **Tauri 2** (Rust backend, WebView frontend) |
| Frontend    | **Vue 3**, **TypeScript**, **Vite** |
| UI components | **shadcn-vue** ([shadcn-vue.com](https://www.shadcn-vue.com)), **Tailwind CSS v4** |
| Backend     | **Rust**: `reqwest` (HTTP), `serde` / `serde_json` |

Relevant paths:

- Frontend: `src/` (e.g. `App.vue`, `main.ts`)
- UI components: `src/components/ui/` (shadcn-vue; use `@/components/ui/<name>`)
- Utils: `src/lib/utils.ts` (includes `cn()` for class names)
- Styles: `src/index.css` (Tailwind + theme CSS variables)
- Backend: `src-tauri/src/lib.rs` (Tauri commands and HTTP logic)
- Config: `src-tauri/tauri.conf.json`, `package.json`, `components.json` (shadcn-vue)

### UI components (shadcn-vue)

- **Add a component**: `pnpm dlx shadcn-vue@latest add <component>` (e.g. `add button`, `add card`, `add input`). Components are installed under `src/components/ui/`.
- **Import**: use the `@` alias, e.g. `import { Button } from "@/components/ui/button"`.
- **Class names**: use the `cn()` helper from `@/lib/utils` to merge Tailwind classes (e.g. with conditional or variant styles).
- **Theming**: edit CSS variables in `src/index.css` (`:root` and `.dark`); base color and style are in `components.json`.

---

## Core Concepts

These are the three main conceptual pillars of the app. They are implemented **little by little**; the list below is the target design.

### 1. Collections

- A **collection** is a **group of resources**.
- On disk: **one collection = one folder**; each **resource** is a file or subfolder inside it (e.g. one JSON file per call-type resource, subfolders for grouping).
- A **resource** is the unit of “something in a collection.” It has a **type**. Supported (or planned) resource types:
  - **HTTP** — classic REST/HTTP call (method, URL, query, headers, body).
  - **GraphQL** — GraphQL request.
  - **gRPC** — gRPC call.
  - **WebSocket** — WebSocket connection/messages.
  - **Folder** — grouping only; contains other resources (maps to a subfolder on disk).
- Call-type resources (HTTP, GraphQL, gRPC, WebSocket) can be **executed**; they can use **variables** (e.g. from an environment) resolved at execution time.
- **Pre-** and **post-execution** scripts (written in **Rhai**) can run before or after a call-type resource is run; **logs** from execution are kept.

So in short: **Collection = folder of resources** (files + optional subfolders), with multiple **resource types** (HTTP, GraphQL, gRPC, WebSocket, Folder).

### 2. Environments

- An **environment** is a **set of variables** (e.g. `base_url`, `api_key`, `user_id`) that can be **used when executing any resource** (e.g. in URL, headers, or body).
- Environment variables can come from **multiple sources**. For now we support:
  - **UI**: Variables set or edited in the app (e.g. in a dedicated environment panel).
  - **File**: Variables loaded from a file, e.g. a **`.env`** file (path or reference stored in the app).
- Other sources (e.g. shell env, secrets manager) may be added later. The design should allow merging or overriding across sources (e.g. UI overrides file).
- Environments are the main way to switch context (e.g. dev vs staging vs prod) without changing resource definitions.

### 3. Sequences

- A **sequence** is an **ordered flow of resource calls** (similar to test cases or workflows). It references call-type resources (HTTP, GraphQL, gRPC, WebSocket).
- Resources in a sequence can be run:
  - **In sequence** (one after another),
  - **In parallel**, or
  - **Asynchronously** (fire-and-forget or with controlled concurrency).
- Sequences can have **runtime variables** that:
  - **Derive from** the current environment, and
  - Can be **persisted back into** the environment when needed (e.g. save a token from one call for later calls).

**Sequences are the most important feature of the app**: they are the main way users orchestrate multi-step API flows and reuse data between resource calls.

---

## Current Implementation State

- **HTTP execution**: A single HTTP call can be sent from the UI; the backend exposes `send_http_request` (in `src-tauri/src/lib.rs`) and returns status, headers, body, and timing.
- **UI**: Left rail (History / Collections), sidebar list, main area with request builder (Params, Headers, Body, Auth tabs) and response pane. History is in-memory only; **Collections** tab exists but does not yet implement the folder/resource model.
- **Not yet implemented**: File-based collections and resources (all resource types), environments, variables, pre/post Rhai scripts, logs, and sequences (sequential/parallel/async runs and env persistence).

When adding features, prefer **small, incremental steps** that align with the concepts above.

---

## Conventions for Agents

- **Collections** = filesystem folders; **resources** = files (e.g. JSON per call-type) or subfolders (folder-type). Design file formats and APIs so resource type (HTTP, GraphQL, gRPC, WebSocket, Folder) is clear and extensible.
- **Environments** = variable sets used at execution time; avoid hardcoding env-specific values in resource definitions when variables can be used.
- **Sequences** are first-class: design so that a sequence can reference call-type resources (e.g. by path or id), define order/parallelism, and read/write environment variables.
- Pre/post scripts are **Rhai**; keep the scripting surface small and focused on the call (request/response) and env.
- Prefer extending the existing Tauri commands and Vue UI rather than replacing them; keep current HTTP request/response shapes where they still fit.
- **UI**: Use **shadcn-vue** components for new UI (buttons, inputs, dialogs, etc.). Add via `pnpm dlx shadcn-vue@latest add <component>`. Use the `@` path alias and `cn()` from `@/lib/utils` for class merging.

---

## Summary

| Concept      | Short description |
|-------------|--------------------|
| **Collection** | Folder of **resources**. Resources have types: **HTTP**, **GraphQL**, **gRPC**, **WebSocket**, **Folder** (grouping). Call-type resources can use variables and pre/post Rhai scripts; execution logs are tracked. |
| **Environment** | Set of variables used when executing resources (and optionally updated by sequences). Sources: **UI** and **file** (e.g. `.env`) for now; more sources may be added later. |
| **Sequence** | Ordered/parallel/async run of **resource calls** with env-derived runtime variables and optional persistence back to the environment; **core feature** of the app. |

Building is **incremental**: implement collections (folder + resources, starting with HTTP), then environments (variables), then sequences (orchestration + env read/write), and script/log features as needed.
