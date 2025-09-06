# HTTPBin Test Suite for Native Doctor

This directory contains a comprehensive test suite for [httpbin.org](https://httpbin.org/) endpoints using Native Doctor.

## Overview

HTTPBin is a simple HTTP Request & Response Service that provides various endpoints for testing HTTP clients. This test suite covers all major endpoint categories available on httpbin.org.

## Project Structure

```text
httpbin/
├── main.ndp.yaml              # Main project configuration
├── README.md                  # This file
│
├── Basic HTTP Methods/
├── get.nd.yaml               # GET request test
├── post.nd.yaml              # POST with JSON body
├── put.nd.yaml               # PUT request test
├── patch.nd.yaml             # PATCH request test
├── delete.nd.yaml            # DELETE request test
│
├── Status Code Testing/
├── status-200.nd.yaml        # HTTP 200 OK
├── status-404.nd.yaml        # HTTP 404 Not Found
├── status-500.nd.yaml        # HTTP 500 Internal Server Error
├── status-random.nd.yaml     # Random status selection
│
├── Authentication/
├── basic-auth.nd.yaml        # HTTP Basic Auth
├── bearer-token.nd.yaml      # Bearer token auth
├── digest-auth.nd.yaml       # HTTP Digest Auth
│
├── Headers & Cookies/
├── headers.nd.yaml           # Header inspection
├── user-agent.nd.yaml        # User-Agent testing
├── cookies-set.nd.yaml       # Cookie setting
├── cookies-get.nd.yaml       # Cookie retrieval
│
├── Redirects & Delays/
├── redirect.nd.yaml          # HTTP redirects
├── delay.nd.yaml             # Response delays
├── cache.nd.yaml             # Cache control
│
├── Data Formats/
├── json.nd.yaml              # JSON response
├── xml.nd.yaml               # XML response
├── html.nd.yaml              # HTML response
├── base64.nd.yaml            # Base64 decoding
│
├── Form Data/
├── form-data.nd.yaml         # Form-urlencoded data
├── multipart-form.nd.yaml    # Multipart form data
│
└── Utilities/
    ├── ip.nd.yaml            # IP address detection
    ├── uuid.nd.yaml          # UUID generation
    ├── gzip.nd.yaml          # GZIP compression
    └── deflate.nd.yaml       # Deflate compression
```

## Usage

### Run Individual Tests

```bash
# Test basic GET request
nativedoctor run httpbin/get.nd.yaml

# Test POST with JSON
nativedoctor run httpbin/post.nd.yaml

# Test authentication
nativedoctor run httpbin/basic-auth.nd.yaml
```

### Run Test Sequences

```bash
# Run basic HTTP methods
nativedoctor run httpbin/main.ndp.yaml

# Or specify a sequence
nativedoctor run httpbin/main.ndp.yaml --sequence basic-methods
nativedoctor run httpbin/main.ndp.yaml --sequence authentication
nativedoctor run httpbin/main.ndp.yaml --sequence all-tests
```

## Test Categories

### 1. Basic HTTP Methods

Tests fundamental HTTP verbs (GET, POST, PUT, PATCH, DELETE) with various payloads and headers.

### 2. Status Code Testing

Validates handling of different HTTP status codes including success, client errors, and server errors.

### 3. Authentication

Tests various authentication mechanisms:

- HTTP Basic Authentication
- Bearer Token Authentication  
- HTTP Digest Authentication

### 4. Headers & Cookies

Validates header handling and cookie management:

- Custom header inspection
- User-Agent parsing
- Cookie setting and retrieval

### 5. Redirects & Delays

Tests client behavior with:

- HTTP redirects (multiple hops)
- Delayed responses
- Cache control headers

### 6. Data Formats

Tests different response formats:

- JSON parsing
- XML handling
- HTML content
- Base64 decoding

### 7. Form Data

Tests form submission methods:

- URL-encoded form data
- Multipart form data

### 8. Utilities

Tests utility endpoints:

- IP address detection
- UUID generation
- Compression support (GZIP, Deflate)

## About HTTPBin

[HTTPBin](https://httpbin.org/) is a simple HTTP Request & Response Service that provides endpoints for testing HTTP libraries. It's particularly useful for:

- Testing HTTP clients
- Debugging API requests
- Learning HTTP protocols
- Validating request/response handling

All endpoints return JSON responses containing information about the request that was made, making it easy to verify that your HTTP client is working correctly.
