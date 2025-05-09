# .api File Format Documentation (Enhanced DX)

The `.api` file format is a structured, human-readable syntax for defining HTTP requests, environment-aware variables, chaining, and testing logic. This version emphasizes Developer Experience (DX) through improved clarity, readability, and powerful scripting capabilities.

## File Structure

A typical `.api` file consists of:

- **`@env` blocks:** For declaring environment-aware variables.
- **One or more `@request` blocks:** For defining individual HTTP requests with detailed configurations.
- **Optional `@call <RequestName>` statements:** To execute a named request (details of `@call` are minimal, serving as a placeholder for invocation).

## Environment Variables

Environment variables allow for dynamic request configurations tailored to different environments (e.g., development, staging, production).

- **Declaration:** Variables are declared within an `@env ... @end` block.
- **Syntax:**
  - Default value: `<name> = <value>`
  - Environment-specific value: `<name>.<environment> = <value>` (e.g., `base_url.dev = https://dev.api.example.com`)
- **Interpolation:** Use `{{variable_name}}` to interpolate variables within request blocks. Optionally, simple filters or functions like `{{variable | default: "fallback"}}` or `{{uuid()}}` could be supported by the processing engine.

**Example:**

```api
@env
  username = "default_user"
  username.dev = "dev_user"
  base_url = https://api.example.com
  base_url.dev = https://dev.api.example.com
  base_url.prod = https://prod.api.example.com
  api_key.prod = "prod_secret_key"
@end
```

If a request runs in the `dev` environment, `{{base_url}}` would resolve to `https://dev.api.example.com` and `{{username}}` to `dev_user`. In `prod`, `{{api_key}}` would use `prod_secret_key`.

## Request Block Syntax

The `@request` block is the core of the `.api` file, defining an HTTP request and its associated logic.

**Structure:**

```api
@request <RequestName>
  METHOD <HTTP_METHOD>      // e.g., POST, GET, PUT, DELETE
  URL <TargetURL>           // e.g., {{base_url}}/items/{{item_id}}

  // Optional: Request-specific configurations
  @config {
    depends-on: [<OtherRequestName1>, <OtherRequestName2>] // Array of request names
    delay: <duration>            // e.g., 500ms, 1s (delay before execution)
    timeout: <duration>          // e.g., 30s (request timeout)
    retries: <number>            // e.g., 3 (number of retries on failure)
  }

  // Optional: Headers section
  @headers {
    Content-Type: application/json
    Authorization: Bearer {{authToken}}
    X-Custom-Header: {{some_variable}}
    // X-Request-ID: {{uuid()}} // Example of a dynamic function call
  }

  // Optional: Query Parameters section
  @query {
    page: {{page_number | default: 1}}
    limit: 25
    search_term: "{{search_query}}"
    tags: ["active", "featured"] // Processor defines how arrays are stringified
  }

  // --- Body Section (Choose one style based on content type) ---

  // Option A: Typed Body (e.g., json, xml, text, form-urlencoded)
  @body:json
  {
    "name": "{{itemName}}",
    "details": {
      "price": {{itemPrice}},
      "available": true
    }
  }

  // Option B: GraphQL Body (with optional variables section)
  @body:graphql
  query GetItem($id: ID!) {
    item(id: $id) {
      id
      name
      description
    }
  }
  @variables:json // Optional: variables for the GraphQL query
  {
    "id": "{{current_item_id}}"
  }

  // Option C: Multipart Form Data Body
  @body:multipart {
    field "description" = "Item submission for {{itemName}}"
    field "itemId" = "{{item_id}}"
    file "image" = "/path/to/image.png" type "image/png" // Optional explicit MIME type
    file "attachment" = "./data/report.pdf"
  }

  // --- Post-Request Processing: Scripting for Extractions & Assertions ---
  @script:post-request {
    // Variable Extraction:
    // Access to response: response.status, response.headers, response.body (parsed), response.time
    new_item_id = response.body.json.data.id
    auth_token = response.body.json.token

    // Setting variables for use in subsequent requests (scoped to environment or globally)
    env.set("currentItemId", new_item_id)
    env.set("sessionToken", auth_token)

    // Assertions:
    assert response.status == 201
    assert response.headers["Content-Type"].startsWith("application/json")
    assert response.body.json.message == "Item created successfully"
    assert typeof response.body.json.data.id == "string"
    assert response.time < 1000ms // Performance check: response time under 1000ms

    // Conditional logic or logging:
    if (response.body.json.data.needs_review) {
      log "Item {{new_item_id}} requires manual review."
    }
  }
@end
```

### Key Sections and Directives within `@request`:

- **`METHOD <HTTP_METHOD>`**: Defines the HTTP method (e.g., `GET`, `POST`).
- **`URL <TargetURL>`**: Specifies the request URL, allowing variable interpolation.
- **`@config { ... }`**:
  - `depends-on: [<RequestName>, ...]`: Specifies an array of other request names that must execute successfully before this request.
  - `delay: <duration>`: Introduces a delay (e.g., `500ms`, `2s`) before the request is made.
  - `timeout: <duration>`: Sets a timeout for the request.
  - `retries: <number>`: Specifies the number of times to retry the request upon failure.
- **`@headers { ... }`**: A block for defining request headers using `Key: Value` pairs.
- **`@query { ... }`**: A block for defining URL query parameters using `Key: Value` pairs. Array values might be supported depending on the processor.
- **`@body:<type>`**: Defines the request body. The `<type>` specifies the format:
  - `json`: The content is treated as a JSON object/array.
  - `graphql`: The content is a GraphQL query. Can be accompanied by `@variables:json { ... }` for query variables.
  - `xml`: The content is treated as XML.
  - `text`: The content is plain text.
  - `form-urlencoded`: The content is URL-encoded form data.
  - `multipart`: For `multipart/form-data` requests. Contains `field` and `file` sub-directives:
    - `field "<name>" = "<value>"`
    - `file "<name>" = "<path_to_file>" [type "<mime_type>"]`
- **`@script:post-request { ... }`**: A powerful block for scripting actions after the request completes.
  - **Variable Extraction:** Access response data (e.g., `response.body.json.data.id`, `response.headers["X-RateLimit-Limit"]`) and assign to variables.
  - **Environment Variable Setting:** Use `env.set("varName", value)` to make extracted data available to subsequent requests or the environment.
  - **Assertions:** Validate the response using `assert condition` (e.g., `assert response.status == 200`, `assert response.body.json.user.active == true`). Supports comparisons, type checks, and potentially more complex expressions depending on the script interpreter.
  - **Logging:** `log "message"` for debugging or informational output.
  - **Conditional Logic:** Basic `if` statements for more complex validation or flow control.
  - The scripting language is assumed to be a simple, expressive DSL or a subset of a common language (e.g., JavaScript-like).

## Multi-request Example (Chaining)

This example demonstrates logging in to get a token, then using that token to fetch user data.

```api
@env
  base_url = "https://api.yourapp.com/v1"
  user_email.dev = "tester@example.com"
  user_password.dev = "securepassword123"
@end

@request LoginUser
  METHOD POST
  URL {{base_url}}/auth/login

  @headers {
    Content-Type: application/json
  }

  @body:json
  {
    "email": "{{user_email}}",
    "password": "{{user_password}}"
  }

  @script:post-request {
    // Extract token and user ID from response
    // Assuming response: { "data": { "token": "xyz", "userId": "123" } }
    login_token = response.body.json.data.token
    logged_in_user_id = response.body.json.data.userId

    // Set them as environment variables for other requests to use
    env.set("authToken", login_token)
    env.set("currentUserId", logged_in_user_id)

    assert response.status == 200
    assert typeof login_token == "string"
  }
@end

@request GetUserData
  METHOD GET
  URL {{base_url}}/users/{{currentUserId}} // Uses 'currentUserId' set by LoginUser

  @config {
    depends-on: [LoginUser] // Ensures LoginUser runs first
  }

  @headers {
    Authorization: Bearer {{authToken}} // Uses 'authToken' set by LoginUser
    Accept: application/json
  }

  @script:post-request {
    assert response.status == 200
    assert response.body.json.data.id == {{currentUserId}}
    assert response.body.json.data.email == {{user_email}} // Assuming email is part of user data
    log "Successfully fetched data for user: " + response.body.json.data.name
  }
@end
```

In this flow:

1.  `LoginUser` executes, sends credentials, and on success:
    - Extracts `token` and `userId` from the response.
    - Sets `authToken` and `currentUserId` environment variables.
    - Performs assertions on its own response.
2.  `GetUserData` executes (due to `depends-on`):
    - Uses `{{currentUserId}}` in its URL and `{{authToken}}` in its headers.
    - Performs assertions on its response, validating the fetched user data.

## Assertions and Testing

Testing is primarily handled within the `@script:post-request` block using `assert` statements and other scripting capabilities. This provides a flexible and powerful way to validate various aspects of the API response.

**Examples of Assertions:**

```api
@script:post-request {
  // Status code
  assert response.status == 200
  assert response.status != 404

  // Headers
  assert response.headers["Content-Type"] == "application/json; charset=utf-8"
  assert response.headers["X-RateLimit-Remaining"] > 0

  // Body content (assuming JSON response)
  assert response.body.json.success == true
  assert response.body.json.data.items.length == 10
  assert response.body.json.user.name.startsWith("John")
  assert "admin" in response.body.json.user.roles // Check if 'admin' is in the roles array
  assert response.body.json.error == null

  // Response time
  assert response.time < 1500ms // Response took less than 1.5 seconds

  // Type checking
  assert typeof response.body.json.data.id == "number"
  assert Array.isArray(response.body.json.data.tags)
}
```
