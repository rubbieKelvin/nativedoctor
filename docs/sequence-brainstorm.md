# Sequence design brainstorm

Notes for later: how sequences could work, improvements, and what to nail down. See also **AGENTS.md** for the high-level concept.

---

## 1. What is a sequence, and where does it live?

**Options:**

- **Sequence as a resource**  
  A sequence is just another resource type (e.g. `sequence.json` in a collection). Same folder = same collection; easy to discover and version. You’d need a clear way to distinguish “call-type” vs “sequence” in the resource type enum (e.g. `Sequence` next to HTTP, GraphQL, …).

- **Sequence as a separate top-level thing**  
  Sequences live in their own store or folder (e.g. `sequences/` next to collections). Cleaner separation but two places to look for “what can I run.”

- **Sequence as a file inside a collection**  
  e.g. `my-collection/sequence.login-and-fetch.json`. The sequence references resources by path or id within that collection (or even across collections if you allow it). Keeps flow and resources together.

**Improvement:** Decide whether a sequence is “a resource in a collection” or “a first-class entity that points at resources.” That choice drives file layout and UI (e.g. “Run sequence” from collection view vs global “Sequences” panel).

---

## 2. How is the flow defined? (structure of a sequence)

The provided screenshot suggests a **node-based flow** (DAG) rather than a simple linear list. This is more powerful for complex orchestrations.

- **Nodes as Resource Instances**  
  Each node in the flow represents an instance of a resource (HTTP, GraphQL, etc.) being called. A single resource could technically be used in multiple nodes within the same sequence.
  - **Node Identity**: Each node has a unique ID (e.g., `node_2`) and a reference to a resource in a collection.
  - **Visual Representation**: Nodes show the resource name, HTTP method, and parent collection.

- **Edges as Execution Dependencies**  
  Arrows (edges) between nodes define the execution order. 
  - `Node A -> Node B` means "Run Node B after Node A finishes."
  - Multiple outgoing arrows from one node can trigger parallel execution of downstream nodes.
  - Multiple incoming arrows to one node can act as a "join," waiting for all precursors to finish (or the first one, depending on configuration).

- **Value Binding (Data Flow)**  
  This is the "glue" shown in the screenshot. It allows passing data from one node's response to another node's request without manually managing global environment variables.
  - **Source**: A path in the response (e.g., `node_2.response.body.id`).
  - **Target**: A field in the next request (e.g., `node_3.request.params.user_id`).
  - **UI**: A dedicated "Value Binding" tool to visually link fields between nodes.

- **Blocks (Grouping)**  
  The screenshot shows a "Block" tool. Blocks can be used to:
  - **Visually organize** related nodes.
  - **Control execution scope**: e.g., "Run all nodes in this block in parallel," or "If any node in this block fails, abort the block."

**Improvement:** Adopt a DAG-based internal representation (nodes and edges) instead of a simple array of steps. This maps directly to the visual "Flow" editor.

---

## 3. Runtime variables and persistence

- **Node-Scoped Variables (Outputs)**  
  Every node execution produces outputs (status, headers, body, timing). These are automatically available to downstream nodes via the node ID (e.g., `{{node_2.body.token}}`).

- **Value Binding vs. Environment Persistence**  
  - **Value Binding**: Short-lived, direct data flow between nodes in a specific sequence run. This is the primary way to link calls.
  - **Environment Persistence**: Long-lived, explicitly saving a value back to the global Environment (e.g., "Save `node_1.body.token` as `auth_token` in 'Dev Env'").

- **Scope**  
  - **Run Scope**: Variables set during a sequence run (including all node outputs) are visible only to that run.
  - **Environment Scope**: Persisted across runs and different resources.

- **Overrides**  
  Sequence could accept **overrides** (e.g., "run with `base_url = …`") that apply only for that run and don’t change the stored environment. Good for quick tests.

**Improvement:** Document how "Value Binding" (the UI tool) generates these run-scoped references automatically, while "Persist to Env" remains a deliberate post-execution action or a specific node type.

---

## 4. Failure and control flow

- **On step failure**  
  Options: **abort sequence**, **retry N times**, **continue** (mark failed, run rest), or **branch** (e.g. “on 4xx do step X, on 2xx do step Y”). Start simple: abort or continue; add retry/branch later.

- **Timeouts**  
  Per-step timeout and optional global sequence timeout so a hung call doesn’t block forever.

- **Idempotency / reruns**  
  If the user re-runs a sequence, do you start from a clean env (or last saved env)? “Run from step 3” is a nice improvement so you don’t re-run login every time.

**Improvement:** Specify default behavior (e.g. “on first failure: abort and show logs”) and one or two optional policies (e.g. “continue on failure”) so the engine has clear semantics.

---

## 5. Scripts and sequences

- **Step-level scripts**  
  You already have pre/post (Rhai) on **resources**. For sequences, the same script runs whether the resource is called standalone or from a sequence. So “post-request: set `token` from response” already feeds into “persist to env” or “run-scoped for next step” if the engine passes that through.

- **Sequence-level scripts**  
  Optional: **before sequence** / **after sequence** Rhai (e.g. “seed test data” or “cleanup”). Less critical for v1; can add once step-level flow is solid.

- **Conditional steps**  
  “Run step 3 only if `status === 200`” could be a small Rhai expression or a simple rule (e.g. “status in [200, 201]”). Reuse Rhai so you don’t invent a second language.

**Improvement:** Keep sequence logic as “orchestration + env read/write”; keep Rhai for “compute from response / decide.” Then sequences stay declarative and scripts stay in one place.

---

## 6. UX and Authoring

- **Flow Canvas (Primary Editor)**  
  A visual workspace where users drag resources from the sidebar onto the canvas to create nodes.
  - **Drag-and-Drop**: Drag an HTTP resource from the collection tree into the canvas to create a node.
  - **Connecting**: Click and drag from one node to another to create an execution dependency.
  - **Contextual Editing**: Selecting a node opens a **"Panel View"** (as seen in the screenshot) at the bottom or side to edit the specific request details (Params, Headers, Body) for that node.

- **Value Binding UI**  
  An interactive mode where users can select a field in the current request (e.g., a header value) and "point" to a field in a previous node's response. This creates an internal reference like `{{node_2.body.id}}`.

- **Execution & Debugging**  
  - **Visual Progress**: Nodes change color/state during execution (e.g., gray -> blue (running) -> green (success) / red (fail)).
  - **Live Console**: A "Console" tab (seen in the screenshot) for real-time logs.
  - **Manual Trigger**: A "Run" button to execute the entire flow or a "Run from here" option on specific nodes.

- **Reuse**  
  - **Sub-sequences**: a step could be “run sequence X” (sequence as a node). Enables “login sequence” + “API sequence” composed together.  
  - **Templates**: “New sequence from template: login → fetch list → fetch detail” to bootstrap common flows.

- **File format**  
  If a sequence is a resource or a file in a collection, one possible shape (you can simplify):

  - `name`, `description`
  - `steps`: array of `{ resource: path-or-id, mode: "sequential"|"parallel"|"fire-and-forget", condition?: RhaiExpr, setEnv?: { "varName": "$.path.or.rhai" } }`
  - Optional `parallelGroups`: list of step indices that run together (if you don’t want mode per step).
  - Optional `onFailure`: `"abort"` | `"continue"`

  That gives you a concrete format to implement and refine.

---

## 7. Summary: what to nail down next

| Area | Decide / improve |
|------|-------------------|
| **Identity** | Sequence as resource type vs separate entity; where it lives on disk. |
| **Structure** | Steps = list of (resource, mode, optional condition, optional setEnv); add parallel groups if needed. |
| **Variables** | Run-scoped step outputs vs “persist to environment” (when and how). |
| **Failure** | Default: abort; optional: continue or retry. |
| **Scripts** | Reuse resource pre/post Rhai; optional sequence-level before/after later. |
| **Format** | One JSON schema (or small set of fields) for a sequence file so you can implement the engine and UI against it. |
