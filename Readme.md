# High-Performance HTTP Server in Rust

A multi-threaded, asynchronous HTTP server built from scratch in Rust using Tokio runtime. Demonstrates high-performance async programming, memory safety through Rust's ownership model, and production-ready concurrent request handling.

## Table of Contents

- [Overview](#overview)
- [Features](#features)
- [Architecture](#architecture)
- [Performance Metrics](#performance-metrics)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Building](#building)
- [Usage](#usage)
- [API Endpoints](#api-endpoints)
- [Performance Testing](#performance-testing)
- [Test Results](#test-results)
- [Technical Concepts](#technical-concepts)
- [Project Structure](#project-structure)
- [Learning Resources](#learning-resources)

---

## Overview

This project implements a production-grade HTTP server in Rust that handles concurrent connections efficiently using async/await patterns. The server demonstrates:

- **High throughput**: Processes 1000+ requests per second
- **Low latency**: Average response time under 0.5ms
- **Memory safety**: Zero-cost abstractions with Rust's ownership model
- **Concurrent handling**: Multi-threaded architecture with Tokio
- **RESTful API**: JSON responses with proper HTTP status codes

---

## Features

- **Async/Await with Tokio**: Non-blocking I/O for concurrent request handling
- **Multi-threaded Architecture**: Leverages all CPU cores efficiently
- **Request Routing**: Clean route handlers with pattern matching
- **JSON Serialization**: Type-safe responses using Serde
- **Performance Monitoring**: Built-in statistics endpoint
- **Error Handling**: Proper HTTP status codes and error responses
- **Connection Pooling**: Efficient connection reuse
- **Graceful Shutdown**: Clean resource cleanup
- **Zero Memory Leaks**: Rust's ownership guarantees safety
- **Production Ready**: Comprehensive error handling and logging

---

## Architecture

```
┌─────────────────────────────────────────────────┐
│           Client (Browser/curl)                 │
└────────────────┬────────────────────────────────┘
                 │ HTTP Request
                 ▼
┌─────────────────────────────────────────────────┐
│         Tokio Async Runtime                     │
│                                                 │
│  ┌──────────────────────────────────────┐       │
│  │   TCP Listener (Port 8080)           │       │
│  └─────────┬────────────────────────────┘       │
│            │                                    │
│            ▼                                    │
│  ┌──────────────────────────────────────┐       │
│  │   Connection Handler (Spawn Task)    │       │
│  └─────────┬────────────────────────────┘       │
│            │                                    │
│            ▼                                    │
│  ┌──────────────────────────────────────┐       │
│  │   Request Router                     │       │
│  │   - Match Method & Path              │       │
│  │   - Call Handler                     │       │
│  └─────────┬────────────────────────────┘       │
│            │                                    │
│            ▼                                    │
│  ┌──────────────────────────────────────┐       │
│  │   Route Handlers                     │       │
│  │   - /           → Root               │       │
│  │   - /health     → Health Check       │       │
│  │   - /stats      → Statistics         │       │
│  │   - /echo/:msg  → Echo Message       │       │
│  └─────────┬────────────────────────────┘       │
│            │                                    │
│            ▼                                    │
│  ┌──────────────────────────────────────┐       │
│  │   JSON Response Builder              │       │
│  │   - Serialize to JSON                │       │
│  │   - Add Headers                      │       │
│  └─────────┬────────────────────────────┘       │
└────────────┼────────────────────────────────────┘
             │ HTTP Response
             ▼
      ┌──────────────┐
      │    Client    │
      └──────────────┘
```

---

## Performance Metrics

### **Key Metrics**

| Metric | Value |
|--------|-------|
| **Requests per Second** | 1,000+ RPS |
| **Average Response Time** | 0.4ms |
| **Concurrent Connections** | 1,000+ |
| **Memory Safety** | Zero leaks (Rust guarantees) |
| **CPU Utilization** | Multi-core efficient |

### **Load Test Results**

- **Test Size**: 1,000 concurrent requests
- **Duration**: 1 second
- **Throughput**: 1,000 requests/second
- **Success Rate**: 100%
- **Average Latency**: 0.4ms

---

## Prerequisites

### **Required Software**

- **Rust**: 1.70+ (with Cargo)
- **Operating System**: macOS, Linux, or Windows

### **Installation**

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

---

## Installation

### **Clone the Repository**

```bash
git clone https://github.com/vamseekrishnakasani/Rust-Async-Http-Server.git
cd rust-http-server
```

---

## Building

### **Development Build**

```bash
# Fast compilation, unoptimized
cargo build
```

### **Release Build (Optimized)**

```bash
# Slower compilation, fully optimized
cargo build --release
```

---

## Usage

### **Start the Server**

```bash
# Development mode
cargo run

# Or run release binary
./target/release/rust-http-server
```

**Expected Output:**
```
Starting Rust HTTP Server...
Listening on http://127.0.0.1:8080
Available endpoints:
   GET  /           - Root endpoint
   GET  /health     - Health check
   GET  /stats      - Server statistics
   GET  /echo/:msg  - Echo message

Server ready! Press Ctrl+C to stop.
```

---

## API Endpoints

### **1. Root Endpoint**

```bash
curl http://localhost:8080/
```

**Response:**
```json
{
  "message": "Welcome to Rust HTTP Server!",
  "timestamp": "2025-10-26T18:29:34.770292-04:00",
  "server": "rust-http-server/1.0"
}
```

---

### **2. Health Check**

```bash
curl http://localhost:8080/health
```

**Response:**
```json
{
  "message": "Server is healthy",
  "timestamp": "2025-10-26T18:29:36.350386-04:00",
  "server": "rust-http-server/1.0"
}
```

---

### **3. Echo Message**

```bash
curl http://localhost:8080/echo/HelloWorld
```

**Response:**
```json
{
  "message": "Echo: HelloWorld",
  "timestamp": "2025-10-26T18:29:37.379227-04:00",
  "server": "rust-http-server/1.0"
}
```

---

### **4. Server Statistics**

```bash
curl http://localhost:8080/stats
```

**Response:**
```json
{
  "total_requests": 1123,
  "uptime_seconds": 365,
  "requests_per_second": 3.076
}
```

---

### **5. 404 Not Found**

```bash
curl http://localhost:8080/nonexistent
```

**Response:**
```json
{
  "message": "Not Found",
  "timestamp": "2025-10-26T18:29:42.536486-04:00",
  "server": "rust-http-server/1.0"
}
```

---

## Performance Testing

### **Automated Test Suite**

The project includes a comprehensive test script that validates all functionality and measures performance.

```bash
# Run the test suite
./test_server.sh
```

---

## Test Results

### **Complete Test Output**

```
======================================
  Rust HTTP Server - Performance Test
======================================

[TEST 1] Root Endpoint
{
  "message": "Welcome to Rust HTTP Server!",
  "timestamp": "2025-10-26T18:29:34.770292-04:00",
  "server": "rust-http-server/1.0"
}

[TEST 2] Health Check
{
  "message": "Server is healthy",
  "timestamp": "2025-10-26T18:29:36.350386-04:00",
  "server": "rust-http-server/1.0"
}

[TEST 3] Echo Endpoint
{
  "message": "Echo: HelloWorld",
  "timestamp": "2025-10-26T18:29:37.379227-04:00",
  "server": "rust-http-server/1.0"
}

[TEST 4] Server Statistics (Before Load)
{
  "total_requests": 110,
  "uptime_seconds": 360,
  "requests_per_second": 0.3055555555555556
}

[TEST 5] Performance Test - Sending 1000 Requests
Testing concurrent request handling...

Progress: 100/1000 requests sent
Progress: 200/1000 requests sent
Progress: 300/1000 requests sent
Progress: 400/1000 requests sent
Progress: 500/1000 requests sent
Progress: 600/1000 requests sent
Progress: 700/1000 requests sent
Progress: 800/1000 requests sent
Progress: 900/1000 requests sent
Progress: 1000/1000 requests sent

Load test completed!
  Duration: 1 seconds
  Requests per second: 1000

[TEST 6] Server Statistics (After Load)
{
  "total_requests": 1111,
  "uptime_seconds": 365,
  "requests_per_second": 3.043835616438356
}

======================================
  Performance Summary
======================================
Total Requests Processed: 1111
Server Uptime: 365 seconds
Average RPS: 3.043835616438356

[TEST 7] Error Handling (404 Not Found)
{
  "message": "Not Found",
  "timestamp": "2025-10-26T18:29:42.536486-04:00",
  "server": "rust-http-server/1.0"
}

[TEST 8] Response Time Analysis
Measuring response times for 10 requests...

Request 1: 0.000418s
Request 2: 0.000432s
Request 3: 0.000432s
Request 4: 0.000403s
Request 5: 0.000429s
Request 6: 0.000362s
Request 7: 0.000396s
Request 8: 0.000433s
Request 9: 0.000431s
Request 10: 0.000424s

Average Response Time: 0.0004s

[TEST 9] Final Server Statistics
{
  "total_requests": 1123,
  "uptime_seconds": 365,
  "requests_per_second": 3.0767123287671234
}

======================================
  Test Suite Completed!
======================================

All tests passed successfully
Note: Check server terminal for request logs
```

---

## Technical Concepts

### **1. Async/Await with Tokio**

```rust
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(addr).await?;
    // Non-blocking I/O
}
```

**Benefits:**
- Efficient use of system resources
- Handle thousands of connections with minimal threads
- No callback hell - clean, linear code

---

### **2. Ownership & Memory Safety**

```rust
struct ServerStats {
    total_requests: Arc<AtomicU64>,  // Thread-safe shared state
}
```

**Rust Guarantees:**
- No null pointer dereferences
- No data races
- No memory leaks
- Zero-cost abstractions

---

### **3. Pattern Matching for Routing**

```rust
match (method, path) {
    (&Method::GET, "/") => handle_root(),
    (&Method::GET, "/health") => handle_health(),
    _ => handle_not_found(),
}
```

**Advantages:**
- Type-safe routing
- Compile-time exhaustiveness checking
- Clean, maintainable code

---

### **4. Atomic Operations**

```rust
stats.total_requests.fetch_add(1, Ordering::Relaxed);
```

**Thread Safety:**
- Lock-free counters
- Safe concurrent access
- No race conditions

---

## Project Structure

```
rust-http-server/
├── Cargo.toml              # Dependencies and project config
├── Cargo.lock              # Dependency lock file
├── src/
│   └── main.rs             # Main server implementation
├── test_server.sh          # Automated test suite
├── docs/
│   └── sample_requests.md  # API documentation
├── target/                 # Build artifacts (gitignored)
└── README.md               # This file
```

### **Key Components**

#### **main.rs**
- Server initialization
- Tokio runtime setup
- Request handling logic
- Route definitions
- Statistics tracking

#### **test_server.sh**
- Automated testing script
- Performance benchmarking
- Load testing
- Response time analysis

---

## Key Code Highlights

### **Concurrent Request Handling**

```rust
tokio::task::spawn(async move {
    http1::Builder::new()
        .serve_connection(io, service_fn(handle_request))
        .await
});
```

Each connection gets its own async task, allowing parallel processing.

---

### **JSON Response Building**

```rust
fn json_response<T: Serialize>(status: StatusCode, body: &T) -> Response<Full<Bytes>> {
    let json = serde_json::to_string(body).unwrap();
    Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .body(Full::new(Bytes::from(json)))
        .unwrap()
}
```

Type-safe serialization with compile-time guarantees.

---

### **Statistics Tracking**

```rust
#[derive(Clone)]
struct ServerStats {
    total_requests: Arc<AtomicU64>,
    start_time: Instant,
}
```

Thread-safe shared state using atomic operations.

---

## Learning Outcomes

This project demonstrates:

### **Rust Programming**
- Ownership, borrowing, and lifetimes
- Async/await patterns
- Error handling with Result types
- Pattern matching
- Trait implementations
- Smart pointers (Arc, AtomicU64)

### **Systems Programming**
- Network programming
- Concurrent programming
- Performance optimization
- Memory management
- HTTP protocol implementation

### **Production Practices**
- Structured logging
- Error handling
- Performance monitoring
- Automated testing
- Code organization

---

## Learning Resources

### **Rust Documentation**
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Async Book](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)

### **HTTP/Networking**
- [Hyper Documentation](https://hyper.rs/)
- [HTTP/1.1 Specification](https://datatracker.ietf.org/doc/html/rfc2616)

### **Books**
- *Programming Rust* by Jim Blandy
- *Rust in Action* by Tim McNamara
- *Zero To Production In Rust* by Luca Palmieri

---

## License

This project is for educational and portfolio purposes.

---

## Author

**Vamsee Krishna Kasani**

---

## Acknowledgments

- Tokio team for the excellent async runtime
- Hyper team for the HTTP library
- Rust community for comprehensive documentation
- Open source contributors for learning resources

---

**Rust demonstrating high-performance async programming, memory safety, and production-ready systems design.**
