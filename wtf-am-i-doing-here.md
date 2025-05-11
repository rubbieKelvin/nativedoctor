# WTF am I Doing: my personal sanity Check

(I'm writing this so I don't get lost in the abyss of all this Rust code as I build this project and add more features). So basically, I'm building a program that executes HTTP requests based on configuration provided from a YAML file.

The core idea is to define API tests and workflows in a structured, human-readable YAML format. Here's a simple example:

```yaml
env:
  base_url:
    default: "http://0.0.0.0:8000"

requests:
  Ping:
    method: GET
    url: "{{base_url}}/ping" # Uses the 'base_url' environment variable

calls:
  # Define an execution sequence named 'test_001'
  test_001:
    - Ping
```

This looks pretty easy to understand, right? I hope so! In this example, we're defining a GET request named `Ping` that targets `http://0.0.0.0:8000/ping` (once the `base_url` variable is resolved). The request isn't made until it's explicitly specified in a `calls` sequence.

The main advantage here is integrating API tests directly into your development workflow and repository. Unlike external tools, your API test configurations live alongside your code. This is particularly powerful for CI/CD pipelines, allowing you to automatically test your backend APIs with specific data and sequences whenever a pull request is created or code is deployed.

## Imports (`imports`)

The `imports` section allows you to include definitions from other YAML files.

```yaml
imports:
  - "common/auth_requests.yaml"
  - "shared/default_headers.yaml"
```

When a file is loaded, the program should recursively load any files listed in its `imports` section. Definitions from imported files (requests, env vars, calls) are merged into the main context. If there are conflicts (e.g., two files define the same request name), the behavior needs to be defined (e.g., last one wins, or raise an error).

_(I haven't implemented this yet in the Rust code.)_

## Environment Variables (`env`)

The `env` section is used to define environment-specific variables. This allows you to use the same request definitions across different environments (development, staging, production) without modifying the request itself.

```yaml
env:
  base_url:
    default: "[https://api.example.com](https://api.example.com)" # Default value
    dev: "[https://dev.api.example.com](https://dev.api.example.com)" # Override for 'dev' environment
    staging: "[https://staging.api.example.com](https://staging.api.example.com)" # Override for 'staging'
  api_key:
    default: "fallback_key"
    prod: "super_secret_prod_key"
  user_id:
    default: "12345"
```

At runtime, when you execute a test, you can specify the target environment (e.g., `dev`, `prod`). The program will resolve the value for each environment variable by checking if an override exists for the specified environment; if it does, that value is used, otherwise, the `default` value is used.

For simplicity and consistency in the Rust implementation, i've decided that **all environment variable values (both `default` and overrides) must be strings** in the YAML file (I didnt want to go through the stress 😪).

```yaml
env:
  timeout_seconds:
    default: "30"
  debug_mode:
    default: "true"
```

The `build_env` function in the `Runner` struct is responsible for taking the parsed `env` structure and the specified runtime environment name, resolving the correct string value for each variable, and returning a `HashMap<String, String>` representing the active environment variables for a given request execution.

## Request Definitions (`requests`)

The `requests` section contains the core of your API tests – the definitions of individual HTTP requests. Each key in this map is a unique name for a request, and its value is a map detailing the request configuration (method, URL, headers, body, etc.).

```yaml
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
        // Script to extract token and set env var
        env.set("authToken", response.body.json.data.token);
        console.assert(response.status == 200);

  GetProfile:
    method: GET
    url: "{{base_url}}/users/{{user_id}}"
    config:
      depends_on: [LoginUser] # This request needs LoginUser to run first
      timeout: 10s # I dont think i've implemented this yet (in the rust code. it's avaliable in the schema) but no p
    headers:
      Authorization: "Bearer {{authToken}}" # Uses env var set by LoginUser
```

Key elements within a request definition include:

- `method`: The HTTP method (GET, POST, etc.).
- `url`: The target URL, supporting `{{variable}}` interpolation.
- `config`: Optional settings like `depends_on` (a list of request names that must complete first), `delay`, `timeout`, and `retries`.
- `headers`: A map of header key-value pairs, supporting interpolation in values.
- `query`: A map of query parameter key-value pairs, supporting interpolation. Values can be complex YAML structures (lists, objects) which the engine needs to serialize correctly for the URL.
- `body`: Defines the request body, with different structures based on the `type` (json, graphql, xml, text, form-urlencoded, multipart). Body content also supports interpolation.
- `script`: Contains script code to run at different phases, currently focusing on `post_request`.

The `build_request` function in the `Runner` struct is responsible for taking a parsed `Request` schema and the resolved environment variables (`HashMap<String, String>`) and constructing a `reqwest::Request` object, handling all the interpolation and body formatting.

## Call Sequences (`calls`)

The `calls` section defines named sequences of requests to be executed. This allows you to group requests into logical workflows or test scenarios.

```yaml
calls:
  full_login_flow: # Name of the sequence
    - LoginUser
    - GetProfile
    - UpdateProfile # Assuming UpdateProfile is another request
  smoke_test: # Another sequence
    - Ping
    - GetStatus # Assuming GetStatus is another request
```

The `calls` key maps a sequence name (string) to a list of request names (strings). When a sequence is executed, the requests in its list are processed in order.

The `call` function in the `Runner` struct handles the execution logic. It takes a request name and recursively calls itself for any requests listed in the `depends_on` configuration of the current request. This recursive nature requires special handling in Rust's `async fn`, which we address using the `#[async_recursion]` attribute from the `async_recursion` crate.

The overall execution flow would involve:

1.  Loading and parsing the main YAML file (and its imports).
2.  Building the initial environment variables based on the `env` section and the specified runtime environment.
3.  Selecting a sequence from the `calls` map to execute.
4.  Iterating through the request names in the selected sequence.
5.  For each request name, calling the `call` function.
6.  The `call` function checks `depends_on` and recursively calls dependencies first.
7.  Once dependencies are met, `call` uses `build_request` to create the `reqwest::Request`.
8.  The `reqwest::Request` is executed using the `reqwest::Client`.
9.  The response is received.
10. The `post_request` script (if any) is executed, with access to the response and the environment.
11. The result (success/failure, captured data) is handled.

## Post Execution Scripting (`script: post_request`)

The `post_request` script block contains code that has access to the request's `response` and the current `env`. this code would be run after the request is completed.

```yaml
script:
  post_request: |
    const token = response.body.json.data.token;
    env.set("authToken", token); // Update environment for subsequent requests

    assert response.status == 200; // Validate status code
    assert typeof token === 'string'; // Validate data type
    assert response.time < 500; // Performance assertion (assuming 'time' is available)

    if (response.headers["X-RateLimit-Remaining"] === "0") {
    log("Rate limit hit!"); // Log information
    }
```

I jusr discovered `deno_runtime` ([crates.io/crates/deno_runtime])... a slimmed-down version of the Deno CLI's runtime. This looks like an excellent candidate for executing these post-request scripts.
_( The YAML schema for scripts is defined, but the integration logic is not yet implemented.)_

## Adding logs to this code.

[learnxbyexample](https://learnxbyexample.com/rust/logging/)
[tracing](https://github.com/tokio-rs/tracing)

## TODO
- [ ] fix call stack generator
- [ ] add deno runtime for scripting