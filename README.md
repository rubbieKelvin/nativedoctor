# Native Doctor

[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/yourusername/nativedoctor)
[![Development Status](https://img.shields.io/badge/status-early%20development-orange.svg)](https://github.com/yourusername/nativedoctor)

> A powerful and developer-friendly CLI tool for defining, executing, and testing HTTP API requests in Rust.

> **⚠️ Early Development Notice**: Native Doctor is currently in active development. While the core functionality is working, some features may be incomplete or subject to breaking changes. I welcome feedback and contributions as I work toward a stable release!

Native Doctor allows you to define API tests and workflows in structured, human-readable YAML files, making it easy to integrate API testing directly into your development workflow and CI/CD pipelines.

## Development Status

**Current Version**: Early Development (v0.1.0)

### What's Working
- ✅ Basic HTTP requests (GET, POST, PUT, DELETE, PATCH)
- ✅ Multiple body types (JSON, XML, form data, multipart)
- ✅ Request dependencies and chaining
- ✅ YAML-based configuration
- ✅ Project organization with sequences

### What's In Progress
- 🚧 Project sequence execution (partially implemented)
- 🚧 Response output handling and formatting
- 🚧 Advanced authentication flows
- 🚧 Environment variable support
- 🚧 Comprehensive error handling

### Planned Features
- 📋 Response validation and assertions
- 📋 Template variables and dynamic content
- 📋 Plugin system for extensibility
- 📋 Interactive CLI mode
- 📋 Performance testing capabilities

I'm actively working on completing these features and welcome contributions from the community!

## Features

- **Simple YAML Configuration** - Define HTTP requests in clean, readable YAML format
- **Request Dependencies** - Chain requests with automatic dependency resolution  
- **Multiple Body Types** - Support for JSON, XML, GraphQL, form data, multipart, and more
- **Authentication Support** - Built-in support for Basic, Bearer, and Digest authentication
- **High Performance** - Built in Rust for speed and reliability
- **Project Organization** - Group requests into sequences and workflows
- **Comprehensive Testing** - Perfect for API testing and validation
- **Rich Documentation** - Self-documenting request files with built-in descriptions

## Quick Start

### Installation

> **Note**: As this project is in early development, installation is currently from source only. Pre-built binaries and package manager distributions will be available in future releases.

```bash
# Clone the repository
git clone https://github.com/yourusername/nativedoctor.git
cd nativedoctor

# Build from source
cargo build --release

# Install locally
cargo install --path .
```

**Requirements:**
- Rust 1.70 or higher
- Cargo (comes with Rust)

### Basic Usage

#### 1. Create a Simple Request

```bash
# Create a single request file
nativedoctor new --request ping
```

This creates `ping.nd.yaml`:

```yaml
name: ping
method: GET
url: https://httpbin.org/get
```

#### 2. Create a Project

```bash
# Create a project with multiple requests
nativedoctor new my-api-tests
```

This creates a project structure with `main.ndp.yaml` and sample requests.

#### 3. Run Requests

```bash
# Execute a single request
nativedoctor run ping.nd.yaml

# Execute a project sequence
nativedoctor run my-api-tests/main.ndp.yaml
```

## Documentation

### Request File Format (`.nd.yaml`)

A request file defines a single HTTP request:

```yaml
name: "User Login"
method: POST
url: "https://api.example.com/auth/login"
doc: "Authenticate user and return JWT token"

headers:
  Content-Type: "application/json"
  User-Agent: "Native-Doctor/1.0"

body:
  type: json
  content: |
    {
      "username": "testuser",
      "password": "securepass123"
    }

config:
  timeout: 5000
  retries: 3
  tags: ["auth", "critical"]
  output: "./login-response.json"
```

### Project File Format (`.ndp.yaml`)

A project file defines sequences of requests:

```yaml
name: "E-commerce API Tests"
version: "1.0.0"
description: "Complete test suite for e-commerce API"
default_sequence: "user-flow"

sequence:
  user-flow:
    - ./auth/login.nd.yaml
    - ./products/list-products.nd.yaml
    - ./cart/add-to-cart.nd.yaml
    - ./orders/checkout.nd.yaml
  
  admin-flow:
    - ./auth/admin-login.nd.yaml
    - ./admin/create-product.nd.yaml
    - ./admin/manage-orders.nd.yaml
```

### Supported HTTP Methods

- `GET` - Retrieve data
- `POST` - Create resources
- `PUT` - Update resources
- `PATCH` - Partial updates
- `DELETE` - Remove resources
- `HEAD` - Headers only
- `OPTIONS` - Preflight requests

### Body Types

#### JSON
```yaml
body:
  type: json
  content: |
    {
      "key": "value",
      "nested": {
        "array": [1, 2, 3]
      }
    }
```

#### GraphQL

```yaml
body:
  type: graphql
  query: |
    query GetUser($id: ID!) {
      user(id: $id) {
        name
        email
      }
    }
  variables:
    id: "123"
```

#### Form Data

```yaml
body:
  type: form-urlencoded
  content:
    username: "testuser"
    password: "secret123"
    remember_me: "true"
```

#### Multipart Form

```yaml
body:
  type: multipart
  parts:
    - kind: field
      name: "description"
      value: "File upload test"
    - kind: file
      name: "avatar"
      path: "./assets/avatar.jpg"
      mime_type: "image/jpeg"
```

### Request Dependencies

Chain requests by specifying dependencies:

```yaml
name: "Protected Resource Access"
method: GET
url: "https://api.example.com/protected"

config:
  require:
    - "./auth/login.nd.yaml"  # Run login first
  
headers:
  Authorization: "Bearer ${login.token}"  # Use token from login
```

### Authentication

#### Basic Authentication
```yaml
headers:
  Authorization: "Basic dXNlcjpwYXNz"  # base64(user:pass)
```

#### Bearer Token

```yaml
headers:
  Authorization: "Bearer eyJhbGciOiJIUzI1NiIs..."
```

#### Custom Headers

```yaml
headers:
  X-API-Key: "your-api-key"
  X-Custom-Auth: "custom-value"
```

## Use Cases

### API Development & Testing

- **Unit Testing**: Test individual API endpoints
- **Integration Testing**: Test request flows and dependencies
- **Regression Testing**: Ensure API changes don't break existing functionality

### CI/CD Integration

- **Automated Testing**: Run API tests in your build pipeline
- **Environment Validation**: Verify deployments across environments
- **Health Checks**: Monitor API availability and performance

### Documentation & Examples

- **Living Documentation**: Keep API examples up-to-date with code
- **Onboarding**: Help new developers understand API usage
- **Client Examples**: Provide working examples for API consumers

## Project Structure

```text
your-project/
├── main.ndp.yaml              # Project configuration
├── auth/
│   ├── login.nd.yaml         # Authentication requests
│   └── refresh-token.nd.yaml
├── users/
│   ├── create-user.nd.yaml   # User management
│   ├── get-user.nd.yaml
│   └── update-user.nd.yaml
├── products/
│   └── list-products.nd.yaml # Product operations
└── orders/
    ├── create-order.nd.yaml  # Order workflow
    └── get-order.nd.yaml
```

## Examples

Check out the `httpbin/` directory for a comprehensive example project that demonstrates:

- All HTTP methods (GET, POST, PUT, DELETE, PATCH)
- Different body types (JSON, XML, form data, multipart)
- Authentication patterns (Basic, Bearer, Digest)
- Status code handling
- Request chaining and dependencies
- Error handling and retries

## Configuration Options

### Request Configuration
```yaml
config:
  require: ["./dependency.nd.yaml"]  # Request dependencies
  delay: 1000                       # Delay before execution (ms)
  timeout: 5000                     # Request timeout (ms)
  retries: 3                        # Number of retry attempts
  tags: ["smoke", "critical"]       # Request categorization
  output: "./response.json"         # Save response to file
```

### Global Settings
- Environment variables for dynamic values
- Custom headers and authentication
- Timeout and retry policies
- Output formatting and logging

## Testing

Run the test suite:

```bash
# Run all tests
cargo test

# Run specific test modules
cargo test slugify
cargo test gen_call_request_sequence

# Run with output
cargo test -- --nocapture
```

## Contributing

I welcome contributions and feedback! Since this is an early-stage project, your input is especially valuable in shaping the direction and priorities. Here's how to get started:

1. **Fork the Repository**
   ```bash
   git clone https://github.com/yourusername/nativedoctor.git
   cd nativedoctor
   ```

2. **Create a Feature Branch**
   ```bash
   git checkout -b feature/amazing-feature
   ```

3. **Make Your Changes**
   - Add tests for new functionality
   - Update documentation as needed
   - Follow Rust coding conventions

4. **Test Your Changes**
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```

5. **Submit a Pull Request**
   - Describe your changes clearly
   - Link any related issues
   - Ensure CI passes

### Development Setup

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone and setup
git clone https://github.com/yourusername/nativedoctor.git
cd nativedoctor

# Install dependencies and build
cargo build

# Run tests
cargo test

# Install locally for testing
cargo install --path .
```

## Getting Help

Since Native Doctor is in early development, you might encounter issues or have questions. Here's how to get help:

### Reporting Issues
- **Bugs**: Please report bugs on our [GitHub Issues](https://github.com/yourusername/nativedoctor/issues) page
- **Feature Requests**: I'd love to hear your ideas! Open a feature request issue
- **Questions**: Use GitHub Discussions or create an issue with the "question" label

### Known Limitations
- Project sequence execution is not fully implemented
- Some error messages may not be user-friendly yet
- Documentation may be incomplete for advanced features
- Breaking changes may occur between versions

### Getting Support
- Check existing issues to see if your problem has been reported
- Include your OS, Rust version, and Native Doctor version when reporting bugs
- Provide minimal reproducible examples when possible

## Troubleshooting

### Common Issues

**Request file not found**
```bash
Error: No such file: "./request.nd.yaml"
```

- Ensure the file path is correct
- Use absolute paths if needed
- Check file permissions

**Schema validation errors**

```bash
Error: Incorrect type. Expected "object(RequestBody)"
```

- Validate your YAML syntax
- Check the schema documentation
- Use proper indentation (spaces, not tabs)

**Dependency cycles**

```bash
Error: Cyclic dependency detected involving 'request.nd.yaml'
```

- Review your request dependencies
- Remove circular references
- Use linear dependency chains

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.md) file for details.

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) for performance and safety
- Uses [reqwest](https://github.com/seanmonstar/reqwest) for HTTP client functionality
- Powered by [clap](https://github.com/clap-rs/clap) for CLI interface
- YAML parsing with [serde_yaml](https://github.com/dtolnay/serde-yaml)

---

**Happy API Testing!**

If you find Native Doctor useful, please consider giving it a star on GitHub!