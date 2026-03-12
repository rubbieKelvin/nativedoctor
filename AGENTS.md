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
| Backend     | **Rust**: `reqwest` (HTTP), `serde` / `serde_json`, **rusqlite** (local app data) |

Relevant paths:

- Frontend: `src/` (e.g. `App.vue`, `main.ts`)
- UI components: `src/components/ui/` (shadcn-vue; use `@/components/ui/<name>`)
- Utils: `src/lib/utils.ts` (includes `cn()` for class names)
- Styles: `src/index.css` (Tailwind + theme CSS variables)
- Backend: `src-tauri/src/lib.rs` (Tauri commands and HTTP logic)
- Config: `src-tauri/tauri.conf.json`, `package.json`, `components.json` (shadcn-vue)
- Schemas: `schema/request.schema.json` (`.request.json` call-type resources), `schema/sequence.schema.json` (`.sequence.json` sequences)
- Project config: `nativedoctor.json` at the root of a project folder (see “Opening a project”). Schema: `schema/nativedoctor.schema.json`.
- Local app DB: SQLite file (e.g. `nativedoctor.db`) in the Tauri app data directory; used for recent projects, settings, and other app-specific data (see “Local app data” below).

### Local app data (rusqlite)

App-specific data (e.g. **recent projects**, settings, request history) is stored in a **local SQLite database** using **[rusqlite](https://docs.rs/rusqlite)**.

- **Crate**: `rusqlite` with the `bundled` feature (SQLite compiled in; no system install required).
- **DB file**: Stored in the Tauri app data directory (e.g. `nativedoctor.db`). Resolve the path via Tauri’s path APIs.
- **Blocking**: rusqlite is synchronous. Run DB access inside `tauri::async_runtime::spawn_blocking` (or a dedicated thread) so the UI thread is not blocked. Tauri commands can remain async and delegate the actual `Connection` work to the blocking task.
- **Tables**: Create tables on first run (e.g. `CREATE TABLE IF NOT EXISTS recent_projects (...)`). Add Tauri commands such as `get_recent_projects` and `add_recent_project` for the frontend to call.

### Component structure

- **Resource pads** (request/response UIs per resource type) live in `src/components/resourcepads/<type>/` (e.g. `http/`). Each pad is a **composite**: one main component (e.g. `HttpResourcePad.vue`) composes **smaller components in the same folder** (e.g. UrlMethodBar, RequestTabs, ParamsPanel, HeadersPanel, BodyPanel, AuthPanel, ResponsePane). Base UI building blocks come from **shadcn-vue** in `@/components/ui/`; add any missing components via the registry: `pnpm dlx shadcn-vue@latest add <component>`.

### UI components (shadcn-vue)

- **Add a component**: `pnpm dlx shadcn-vue@latest add <component>` (e.g. `add button`, `add card`, `add input`). Components are installed under `src/components/ui/`.
- **Import**: use the `@` alias, e.g. `import { Button } from "@/components/ui/button"`.
- **Class names**: use the `cn()` helper from `@/lib/utils` to merge Tailwind classes (e.g. with conditional or variant styles).
- **Theming**: edit CSS variables in `src/index.css` (`:root` and `.dark`); base color and style are in `components.json`.

---

## Opening a project

A project can be opened in two ways:

1. **Command line**: Pass a folder when running the app, e.g. `nativedoctor .`. The app checks for a nativedoctor.json file in that folder and opens with that as the current project. if no nativedoctor.json, create it.
2. **In-app**: When the app is started without a folder argument, the user sees the **Recent Projects** screen (`src/components/workspace/RecentProjects/RecentProjects.vue`). It lists projects opened previously and provides a button (or similar) to **open a nativedoctor project** (e.g. pick a nativedoctor.json file via a dialog). Choosing a the file opens that project.
3. **Created Inapp**: We can have a button in the app and even in the recent projects page that opens new page `src/components/workspace/CreateProject/CreateProject.vue`, where you input the project name, description and a folder path. you can click a button next to the folder field that opens a folder dialog an asks you where you want to save the project. after the folder has been selected, show the folder path in the folder input field. then the create button in the page should now create the project and open it.

**What is a Native Doctor project?** Any **folder** that contains a file named **`nativedoctor.json`** is treated as a Native Doctor project. Folders without this file are not considered projects (or can be handled as “simple request folder” / legacy behavior as you define it).

**The `nativedoctor.json` file** lives at the **root of the project folder** and holds:

- Project **description** and **metadata**
- **Extra details** (as needed for the product)
- **Environment sources** (e.g. references to `.env` files or other env config)
- **All files used in the project** (e.g. which request/sequence files belong to the project)

The structure of `nativedoctor.json` is defined by the JSON schema at `schema/nativedoctor.schema.json` (update that file as the product evolves).

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
- **Resource pads**: Build pads from subcomponents in the same folder; use **shadcn-vue** components as the base UI (from `@/components/ui/`). Add registry components when needed; do not introduce one-off primitives that duplicate shadcn patterns.
- **Rust**: Prefer explicit return types on functions where possible, and use an explicit **`return`** statement in the body rather than expression-based return. For example, prefer:
  ```rust
  pub fn project_has_nativedoctor(path: String) -> bool {
      return std::path::Path::new(&path)
          .join("nativedoctor.json")
          .exists();
  }
  ```
  instead of omitting `return` and relying on the final expression as the return value.

---

## Summary

| Concept      | Short description |
|-------------|--------------------|
| **Project** | Folder containing `nativedoctor.json`; opened via CLI or Recent Projects. Config holds description, metadata, env sources, and file list. |
| **Collection** | Folder of **resources**. Resources have types: **HTTP**, **GraphQL**, **gRPC**, **WebSocket**, **Folder** (grouping). Call-type resources can use variables and pre/post Rhai scripts; execution logs are tracked. |
| **Environment** | Set of variables used when executing resources (and optionally updated by sequences). Sources: **UI** and **file** (e.g. `.env`) for now; more sources may be added later. |
| **Sequence** | Ordered/parallel/async run of **resource calls** with env-derived runtime variables and optional persistence back to the environment; **core feature** of the app. |

Building is **incremental**: implement collections (folder + resources, starting with HTTP), then environments (variables), then sequences (orchestration + env read/write), and script/log features as needed.
