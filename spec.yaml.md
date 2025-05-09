# API Test Specification (YAML Format)

This document describes a YAML-based format for defining HTTP requests, environment-aware variables, request chaining, scripting, and testing logic. This format aims for clarity and structure using standard YAML conventions.

## File Format

The specification uses the YAML 1.2 format. Files typically use the `.yaml` or `.yml` extension.

## Top-Level Structure

A test file is a YAML map with the following potential top-level keys:

* `imports`: (Optional) A list of paths to other YAML files to include definitions from.
* `env`: (Optional) Defines environment variables.
* `requests`: (Optional) Defines named HTTP requests and their configurations.
* `calls`: (Optional) A list specifying which requests to execute when the file is processed.

```yaml
# Example top-level structure
imports:
  - common_env.yaml
  - auth_workflow.yaml

env:
  # Environment variables defined here

requests:
  # Request definitions here

calls:
  # List of request names to call here
```

## Imports (`imports`)

The `imports` key, if present, must be a YAML list where each item is a string representing the path to another test YAML file. The processing engine is responsible for loading and merging definitions from these files.

```yaml
imports:
  - "./shared/common_headers.yaml"
  - "../environments/prod.yaml"
  - "/global/configs/base_requests.yaml"
```

## Environment Variables (`env`)

The `env` key, if present, must be a YAML map defining environment variables. **For consistency, every variable definition must be a map containing a `default` key.**

* **Syntax:** Each variable name must map to a YAML map. This map **must** contain a `default` key specifying the variable's default value. Optional environment-specific overrides can be added as other keys in this map, where the key name is the environment name.
    ```yaml
    env:
      variable_name: # Variable name
        default: "default_value" # REQUIRED Default value
        environment_name_1: "value_for_env_1" # Optional override for environment 1
        environment_name_2: "value_for_env_2" # Optional override for environment 2
      another_var: # Another variable
        default: "another_default_value" # REQUIRED Default value
    ```
* **Values:** Values (for `default` and environment overrides) can be strings, numbers, booleans, lists, or maps. Variable interpolation (`{{variable_name}}`) is supported within string values, to be processed by the engine. Simple filters or functions (`{{variable | default: "fallback"}}`, `{{uuid()}}`) may also be supported.
* **Interpolation:** Use `{{variable_name}}` to interpolate variables in strings throughout the test file (URLs, headers, body, scripts, etc.).

```yaml
# Updated Environment Variables Example
env:
  username:
    default: "default_user"
    dev: "dev_user"
  base_url:
    default: "https://api.example.com" # Default is required
    dev: "https://dev.api.example.com"
    prod: "https://prod.api.example.com"
  api_key:
    default: null # Use null if there's no generic default, but prod requires a value
    prod: "prod_secret_key"
  timeout_seconds:
    default: 10 # Default is required even if simple value
  debug_mode:
    default: false # Default is required
```

## Request Definitions (`requests`)

The `requests` key, if present, must be a YAML map where keys are the names of the requests (strings) and values are maps defining the details of each request.

Each request definition map supports the following keys:

* `method`: (Required) A string representing the HTTP method (e.g., `GET`, `POST`, `PUT`).
* `url`: (Required) A string representing the target URL. Variable interpolation is supported.
* `config`: (Optional) A map for request-specific configurations.
* `headers`: (Optional) A map for request headers.
* `query`: (Optional) A map for URL query parameters.
* `body`: (Optional) A map defining the request body, structured by type.
* `script`: (Optional) A map containing script logic to be executed after the request.

```yaml
requests:
  GetStatus:
    method: GET
    url: "{{base_url}}/status"
    headers:
      Accept: application/json
    script:
      post_request: |
        log("Status check completed.");
        assert response.status == 200;

  CreateItem:
    method: POST
    url: "{{base_url}}/items"
    config:
      depends_on: [LoginUser]
      timeout: 10s
      retries: 2
    # ... other sections
```

### Request Configuration (`config`)

The `config` key within a request definition is an optional map supporting:

* `depends_on`: (Optional) A YAML list of strings, where each string is the name of another request that must complete successfully before this request runs.
* `delay`: (Optional) A string specifying a duration to wait before executing the request (e.g., `"500ms"`, `"1s"`).
* `timeout`: (Optional) A string specifying the request timeout duration (e.g., `"30s"`).
* `retries`: (Optional) A number specifying how many times to retry the request on failure.

```yaml
config:
  depends_on: [LoginUser, SetupData]
  delay: 2s
  timeout: 15s
  retries: 3
```

### Headers (`headers`)

The `headers` key within a request definition is an optional map where keys are header names (strings) and values are header values (strings). Variable interpolation is supported in values.

```yaml
headers:
  Content-Type: application/json
  Authorization: "Bearer {{authToken}}"
  X-Correlation-ID: "{{uuid()}}"
```

### Query Parameters (`query`)

The `query` key within a request definition is an optional map where keys are query parameter names (strings) and values are the parameter values. Values can be strings, numbers, booleans, or lists. Variable interpolation is supported in string values. The processing engine handles the serialization of values (e.g., how lists are represented in the URL).

```yaml
query:
  page: "{{page_number | default: 1}}"
  limit: 25
  is_active: true
  tags: ["new", "promo"]
```

### Request Body (`body`)

The `body` key within a request definition is an optional map that defines the request body. It must contain a `type` key and content specific to that type.

* `type`: (Required) A string specifying the body format. Supported types: `json`, `graphql`, `xml`, `text`, `form-urlencoded`, `multipart`.
* `content`: (Required for `json`, `xml`, `text`, `form-urlencoded`) The body content.
    * For `json`: A YAML map or list representing the JSON structure.
    * For `xml`, `text`, `form-urlencoded`: A string, typically using YAML's literal block scalar (`|`) for multi-line content.
* `query`: (Required for `graphql`) A string using `|` for the multi-line GraphQL query.
* `variables`: (Optional for `graphql`) A YAML map or list for GraphQL query variables, expected to be serializable as JSON.
* `parts`: (Required for `multipart`) A YAML list defining the multipart parts.

#### Body Types Details:

* **`type: json`**
    ```yaml
    body:
      type: json
      content:
        name: "{{itemName}}"
        details:
          price: "{{itemPrice}}"
          available: true
    ```
* **`type: graphql`**
    ```yaml
    body:
      type: graphql
      query: |
        query GetItem($id: ID!) {
          item(id: $id) {
            id
            name
            description
          }
        }
      variables: # Optional variables for the query
        id: "{{current_item_id}}"
    ```
* **`type: xml`**
    ```yaml
    body:
      type: xml
      content: |
        <item>
          <name>{{itemName}}</name>
          <price>{{itemPrice}}</price>
        </item>
    ```
* **`type: text`**
    ```yaml
    body:
      type: text
      content: |
        This is plain text content.
        Value: {{some_variable}}
    ```
* **`type: form-urlencoded`**
    ```yaml
    body:
      type: form-urlencoded
      content: "key1=value1&key2={{variable}}" # Standard URL-encoded string
    ```
* **`type: multipart`**
    The `parts` list contains maps, each defining a part:
    * `kind`: (Required) `"field"` or `"file"`.
    * `name`: (Required) String, the name of the part.
    * `value`: (Required for `kind: field`) String, the value of the form field.
    * `path`: (Required for `kind: file`) String, the path to the file.
    * `mime_type`: (Optional for `kind: file`) String, the explicit MIME type of the file.

    ```yaml
    body:
      type: multipart
      parts:
        - kind: field
          name: description
          value: "Item submission for {{itemName}}"
        - kind: field
          name: itemId
          value: "{{item_id}}"
        - kind: file
          name: image
          path: "/path/to/image.png"
          mime_type: image/png # Optional
        - kind: file
          name: attachment
          path: "./data/report.pdf"
    ```

### Post-Request Scripting (`script`)

The `script` key within a request definition is an optional map. It can contain keys representing different scripting phases (e.g., `post_request` for logic after the request completes). The value associated with a scripting phase key is a string containing the script code, typically using YAML's literal block scalar (`|`) for multi-line scripts.

The scripting language and the available objects (like `response`, `env`, `log`, `assert`) are defined by the processing engine, not by this YAML specification. The YAML format simply holds the script code as a string.

```yaml
script:
  post_request: |
    # Access response data: response.status, response.headers, response.body (parsed)
    new_item_id = response.body.json.data.id;
    auth_token = response.body.json.token;

    # Set environment variables
    env.set("currentItemId", new_item_id);
    env.set("sessionToken", auth_token);

    # Assertions
    assert response.status == 201;
    assert response.headers["Content-Type"].startsWith("application/json");
    assert response.body.json.message == "Item created successfully";
    assert typeof response.body.json.data.id == "string";
    assert response.time < 1000; # Assuming time is in ms

    # Logging or conditional logic (syntax dependent on the script engine)
    if (response.body.json.data.needs_review) {
      log("Item " + new_item_id + " requires manual review.");
    }
```
*(Note: The syntax within the script block is illustrative; the actual valid syntax depends entirely on the script engine implemented by the processor).*

## Calls (`calls`)

The `calls` key, if present, must be a YAML list where each item is a string corresponding to the name of a request defined in the `requests` section (or imported from another file). The processing engine executes these requests in the order listed, respecting any `depends_on` dependencies defined in the request configurations.

```yaml
calls:
  - LoginUser
  - GetUserData
  - UploadFile
```

## Example File (Full Multi-request Chaining in YAML)

```yaml
# Full Example API Test File in YAML

env:
  base_url:
    default: "https://api.yourapp.com/v1"
  user_email:
    dev: "tester@example.com"
  user_password:
    dev: "securepassword123"
  # Example of another env var
  default_page_size: 10

requests:
  LoginUser:
    method: POST
    url: "{{base_url}}/auth/login"
    headers:
      Content-Type: application/json
    body:
      type: json
      content:
        email: "{{user_email}}"
        password: "{{user_password}}"
    script:
      post_request: |
        # Extract token and user ID
        login_token = response.body.json.data.token;
        logged_in_user_id = response.body.json.data.userId;

        # Set env vars for subsequent requests
        env.set("authToken", login_token);
        env.set("currentUserId", logged_in_user_id);

        assert response.status == 200;
        assert typeof login_token == "string";

  GetUserData:
    method: GET
    url: "{{base_url}}/users/{{currentUserId}}" # Uses env var
    config:
      depends_on: [LoginUser] # Depends on LoginUser
    headers:
      Authorization: "Bearer {{authToken}}" # Uses env var
      Accept: application/json
    query:
      # Using env var with default filter example
      page_size: "{{default_page_size | default: 5}}"
      # Example list value
      fields: ["id", "name", "email"]
    script:
      post_request: |
        assert response.status == 200;
        assert response.body.json.data.id == env.get("currentUserId");
        assert response.body.json.data.email == env.get("user_email");
        log("Successfully fetched data for user: " + response.body.json.data.name);

  CreateNewResource:
    method: POST
    url: "{{base_url}}/resources"
    config:
      depends_on: [LoginUser] # Also depends on LoginUser
      timeout: 15s
    headers:
      Content-Type: application/json
      Authorization: "Bearer {{authToken}}"
    body:
      type: json
      content:
        name: "Resource {{uuid()}}" # Example function in interpolation
        status: "active"
    script:
      post_request: |
        assert response.status == 201;
        log("Resource created with ID: " + response.body.json.id);

  # Example of a GraphQL request
  SearchItems:
    method: POST
    url: "{{base_url}}/graphql"
    headers:
      Content-Type: application/json
      Authorization: "Bearer {{authToken}}"
    body:
      type: graphql
      query: |
        query Search($term: String!) {
          items(search: $term) {
            id
            name
          }
        }
      variables:
        term: "test item"
    script:
      post_request: |
        assert response.status == 200;
        assert response.body.data.items is List; # Example type assertion in script

# Define the sequence of requests to execute
calls:
  - LoginUser
  - GetUserData
  - CreateNewResource
  - SearchItems # This will run after LoginUser completes due to implicit sequence, even without depends_on if LoginUser is listed before it. depends_on is for explicit dependencies outside list order.
```

This YAML specification provides a structured and clear representation of all the concepts from your original `.api` format, using standard YAML features. The interpretation of variable interpolation and the execution of the script code remain responsibilities of the separate processing engine that consumes this YAML file.
