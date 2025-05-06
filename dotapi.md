# .api File Format Documentation

The `.api` file format is a structured, human-readable syntax for defining HTTP requests, environment-aware variables, chaining, and testing logic. It is inspired by `.http` files but aims for greater clarity and extensibility.

---

## File Structure

A typical `.api` file may contain:

* `@env` blocks for variable declarations
* One or more `@request` blocks

---

## Environment Variables

```api
@env
username = "admin"
base_url = "https://fallback.url"
base_url.dev  = "https://dev.url"
base_url.prod = "https://prod.url"

@end
```

* Declared with `@env ... @end`
* Supports per-environment overrides via `& { env = value }`
* Use `{{variable}}` to interpolate variables inside request blocks

---

## Request Block Syntax

### Basic Request

```api
@request GetUser
GET {{base_url}}/users/{{user_id}}
@end
```

### Full Example

if body exists, it must be the last thing to declare

```api
@request LoginUser
POST {{base_url}}/auth/login
@header Content-Type: application/json
@extract token from $.token
@body
{
  "username": "{{username}}",
  "password": "secret"
}
@end
```

---

## Supported Directives

| Directive        | Purpose                                              |
| ---------------- | ---------------------------------------------------- |
| `@request <id>`  | Begins a request block                               |
| `@end`           | Ends the request block or env block                  |
| `<METHOD> <URL>` | HTTP verb and target URL                             |
| `@header`        | Adds a request header                                |
| `@query`         | Adds a query string parameter                        |
| `@body`          | Raw body (e.g., JSON)                                |
| `@graphql`       | GraphQL query body                                   |
| `@multipart`     | Begins multipart section                             |
| `@field`         | Adds a field in a multipart request                  |
| `@file`          | Adds a file in a multipart request                   |
| `@expect`        | Expected response MIME type (for validation/testing) |
| `@extract`       | Extract a value from the response using JSONPath     |
| `@assert`        | Assert status code, headers, or body content         |
| `@depends-on`    | Sets dependency on another request                   |
| `@delay`         | Introduce a delay before the request is made (ms)    |

---

## Multi-request Example

```api
@env
base_url = "https://api.example.com"
@end

@request Login
POST {{base_url}}/login
@extract token from $.token
@body { "username": "admin", "password": "secret" }
@end

@request FetchData
GET {{base_url}}/data
@header Authorization: Bearer {{token}}
@depends-on Login
@end
```

---

## Assertions and Testing

```api
@assert status = 200
@assert header.Content-Type = application/json
@assert body.json.user.id exists
```

Useful for scripting request validations in CI pipelines or development flows.

---

## Syntax Rules

* All variables use `{{variable_name}}`
* All blocks start with `@keyword` and end with `@end`
* Comments can be added using `#` at the start of a line
* Directives are line-oriented and do not nest (except JSON/GraphQL bodies)
