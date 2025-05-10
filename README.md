# Rustle

A powerful and developer-friendly tool for defining, executing, and testing HTTP API requests in Rust. Rustle allows you to define API tests and workflows in a structured, human-readable YAML format, making it easy to integrate API testing directly into your development workflow and repository.

## Features

- **YAML-Based Configuration**: Define your API tests in a clear, structured YAML format
- **Environment-Aware**: Configure variables for different environments (dev, staging, prod)
- **Request Chaining**: Define dependencies between requests
- **Rich Request Support**:
  - All HTTP methods (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS, CONNECT, TRACE)
  - Multiple body types (JSON, GraphQL, XML, Text, Form-urlencoded, Multipart)
  - Headers and query parameters
  - Request configuration (timeouts, retries, delays)
- **Post-Request Scripting**: Execute scripts after requests for validation and data extraction
- **File Imports**: Modular configuration through file imports
- **Project Metadata**: Define project information and authors

## Installation

```bash
cargo install rustle
```

## Quick Start

- Create a new Rustle project:

```bash
rustle init my-api-tests
cd my-api-tests
```

- Define your API tests in YAML format:

```yaml
# example.yaml
env:
  base_url:
    default: "https://api.example.com"
    dev: "https://dev.api.example.com"
    prod: "https://prod.api.example.com"

requests:
  GetStatus:
    method: GET
    url: "{{base_url}}/status"
    headers:
      Accept: application/json
    script:
      post_request: |
        assert response.status == 200;
        log("Status check completed.");

  CreateItem:
    method: POST
    url: "{{base_url}}/items"
    config:
      depends_on: [LoginUser]
      timeout: 10s
      retries: 2
    headers:
      Content-Type: application/json
    body:
      type: json
      content:
        name: "{{itemName}}"
        price: "{{itemPrice}}"

calls:
  smoke_test:
    - GetStatus
    - CreateItem
```

- Run your tests:

```bash
# Run in default environment
rustle run

# Run in specific environment
rustle run --env dev

# Run specific test sequence
rustle run --sequence smoke_test
```

## Comprehensive Examples

### 1. Authentication Flow with Token Management

```yaml
env:
  base_url:
    default: "https://api.example.com"
  credentials:
    dev:
      username: "test_user"
      password: "test_pass"
    prod:
      username: "{{PROD_USERNAME}}"
      password: "{{PROD_PASSWORD}}"

requests:
  Login:
    method: POST
    url: "{{base_url}}/auth/login"
    headers:
      Content-Type: application/json
    body:
      type: json
      content:
        username: "{{credentials.username}}"
        password: "{{credentials.password}}"
    script:
      post_request: |
        # Extract and store tokens
        access_token = response.body.json.access_token;
        refresh_token = response.body.json.refresh_token;
        
        # Store tokens in environment
        env.set("access_token", access_token);
        env.set("refresh_token", refresh_token);
        
        # Validate response
        assert response.status == 200;
        assert typeof access_token == "string";
        assert typeof refresh_token == "string";

  RefreshToken:
    method: POST
    url: "{{base_url}}/auth/refresh"
    headers:
      Authorization: "Bearer {{refresh_token}}"
    script:
      post_request: |
        # Update access token
        new_access_token = response.body.json.access_token;
        env.set("access_token", new_access_token);
        
        assert response.status == 200;
        assert new_access_token != env.get("access_token");

  GetProtectedResource:
    method: GET
    url: "{{base_url}}/protected/resource"
    config:
      depends_on: [Login]
    headers:
      Authorization: "Bearer {{access_token}}"
    script:
      post_request: |
        if (response.status == 401) {
          # Token expired, refresh and retry
          env.set("needs_refresh", true);
        } else {
          assert response.status == 200;
          assert response.body.json.data != null;
        }
```

### 2. File Upload with Progress Tracking

```yaml
requests:
  UploadFile:
    method: POST
    url: "{{base_url}}/upload"
    config:
      timeout: 300s  # 5 minutes for large files
    headers:
      Authorization: "Bearer {{access_token}}"
    body:
      type: multipart
      parts:
        - kind: field
          name: description
          value: "Large file upload test"
        - kind: file
          name: file
          path: "./large_file.zip"
          mime_type: application/zip
    script:
      post_request: |
        assert response.status == 200;
        upload_id = response.body.json.upload_id;
        env.set("upload_id", upload_id);
        
        # Check upload status
        if (response.body.json.status == "processing") {
          log("File is being processed. Upload ID: " + upload_id);
        }

  CheckUploadStatus:
    method: GET
    url: "{{base_url}}/upload/{{upload_id}}/status"
    config:
      depends_on: [UploadFile]
      retries: 5
      delay: 5s  # Wait 5 seconds between retries
    script:
      post_request: |
        assert response.status == 200;
        status = response.body.json.status;
        
        if (status == "completed") {
          log("Upload completed successfully");
        } else if (status == "failed") {
          error = response.body.json.error;
          log("Upload failed: " + error);
          assert false, "Upload failed: " + error;
        } else {
          # Still processing, will retry
          assert status == "processing";
        }
```

### 3. GraphQL API Testing

```yaml
requests:
  SearchUsers:
    method: POST
    url: "{{base_url}}/graphql"
    headers:
      Content-Type: application/json
      Authorization: "Bearer {{access_token}}"
    body:
      type: graphql
      query: |
        query SearchUsers($query: String!, $limit: Int!) {
          users(search: $query, limit: $limit) {
            id
            name
            email
            role
            createdAt
          }
        }
      variables:
        query: "{{search_term}}"
        limit: 10
    script:
      post_request: |
        assert response.status == 200;
        users = response.body.data.users;
        
        # Validate response structure
        assert Array.isArray(users);
        assert users.length <= 10;
        
        # Store first user for subsequent requests
        if (users.length > 0) {
          env.set("first_user_id", users[0].id);
        }

  UpdateUserRole:
    method: POST
    url: "{{base_url}}/graphql"
    config:
      depends_on: [SearchUsers]
    headers:
      Content-Type: application/json
      Authorization: "Bearer {{access_token}}"
    body:
      type: graphql
      query: |
        mutation UpdateUserRole($userId: ID!, $role: UserRole!) {
          updateUserRole(userId: $userId, role: $role) {
            id
            role
            updatedAt
          }
        }
      variables:
        userId: "{{first_user_id}}"
        role: "ADMIN"
    script:
      post_request: |
        assert response.status == 200;
        updated_user = response.body.data.updateUserRole;
        
        assert updated_user.id == env.get("first_user_id");
        assert updated_user.role == "ADMIN";
        assert updated_user.updatedAt != null;
```

### 4. Complex Workflow with Conditional Logic

```yaml
env:
  base_url:
    default: "https://api.example.com"
  test_data:
    default:
      items:
        - name: "Item 1"
          price: 100
        - name: "Item 2"
          price: 200

requests:
  CreateOrder:
    method: POST
    url: "{{base_url}}/orders"
    headers:
      Content-Type: application/json
    body:
      type: json
      content:
        items: "{{test_data.items}}"
    script:
      post_request: |
        assert response.status == 201;
        order_id = response.body.json.order_id;
        env.set("order_id", order_id);
        
        # Store order status
        env.set("order_status", response.body.json.status);

  ProcessPayment:
    method: POST
    url: "{{base_url}}/orders/{{order_id}}/payment"
    config:
      depends_on: [CreateOrder]
    body:
      type: json
      content:
        amount: "{{test_data.items[0].price}}"
        currency: "USD"
    script:
      post_request: |
        assert response.status == 200;
        payment_id = response.body.json.payment_id;
        env.set("payment_id", payment_id);
        
        # Check payment status
        if (response.body.json.status == "pending") {
          env.set("needs_payment_confirmation", true);
        }

  ConfirmPayment:
    method: POST
    url: "{{base_url}}/payments/{{payment_id}}/confirm"
    config:
      depends_on: [ProcessPayment]
    script:
      post_request: |
        if (env.get("needs_payment_confirmation")) {
          assert response.status == 200;
          assert response.body.json.status == "confirmed";
        } else {
          log("Payment confirmation not needed");
        }

  CheckOrderStatus:
    method: GET
    url: "{{base_url}}/orders/{{order_id}}"
    config:
      depends_on: [ConfirmPayment]
    script:
      post_request: |
        assert response.status == 200;
        order = response.body.json;
        
        # Validate order state
        assert order.id == env.get("order_id");
        assert order.status == "completed";
        assert order.payment_id == env.get("payment_id");
```

## YAML Configuration Format

The YAML format is defined by a JSON Schema (Draft-07) and supports:

- Environment variables with defaults and environment-specific overrides
- Request definitions with method, URL, headers, body, and configuration
- Post-request scripting
- File imports for modular configuration
- Project metadata

### Key Sections

#### Environment Variables

```yaml
env:
  base_url:
    default: "https://api.example.com"
    dev: "https://dev.api.example.com"
    prod: "https://prod.api.example.com"
  api_key:
    default: "test_key"
    prod: "prod_secret_key"
```

#### Request Definitions

```yaml
requests:
  GetUser:
    method: GET
    url: "{{base_url}}/users/{{user_id}}"
    headers:
      Authorization: "Bearer {{auth_token}}"
    config:
      depends_on: [LoginUser]
      timeout: 10s
      retries: 2
```

#### Request Body Types

```yaml
# JSON Body
body:
  type: json
  content:
    name: "{{itemName}}"
    price: "{{itemPrice}}"

# GraphQL Body
body:
  type: graphql
  query: |
    query GetUser($id: ID!) {
      user(id: $id) {
        id
        name
      }
    }
  variables:
    id: "{{user_id}}"

# Multipart Body
body:
  type: multipart
  parts:
    - kind: field
      name: description
      value: "Item description"
    - kind: file
      name: image
      path: "./images/item.png"
      mime_type: image/png
```

See the [YAML Schema](librustle/src/rustle.draft-07.schema.yaml) for complete details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 
