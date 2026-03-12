# Native Doctor JSON schemas

This folder contains JSON Schema definitions for file-based resources and sequences.

## Schemas

- **request.schema.json** — Describes `.request.json` files: a single call-type resource (HTTP, GraphQL, gRPC, WebSocket). Use for one request per file in a collection.
- **sequence.schema.json** — Describes `.sequence.json` files: an ordered flow of resource calls with optional env persistence and step options.

## Using in editors

In a `.request.json` or `.sequence.json` file (e.g. inside a collection folder), set `$schema` so your editor can validate and provide completion:

```json
{
  "$schema": "../schema/request.schema.json",
  "type": "http",
  "method": "GET",
  "url": "https://api.example.com/users"
}
```

Use a relative path from the JSON file to this folder (e.g. `../schema/request.schema.json` from a file in a collection subfolder, or `./schema/request.schema.json` from the project root).
