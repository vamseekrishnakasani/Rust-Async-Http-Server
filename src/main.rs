use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;

use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Incoming as IncomingBody, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;
use http_body_util::Full;
use hyper::body::Bytes;
use serde::Serialize;
use chrono::Local;

// Server statistics
#[derive(Clone)]
struct ServerStats {
    total_requests: Arc<AtomicU64>,
    start_time: Instant,
}

impl ServerStats {
    fn new() -> Self {
        Self {
            total_requests: Arc::new(AtomicU64::new(0)),
            start_time: Instant::now(),
        }
    }

    fn increment_requests(&self) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
    }

    fn get_total_requests(&self) -> u64 {
        self.total_requests.load(Ordering::Relaxed)
    }

    fn get_uptime_seconds(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }
}

// Response structures
#[derive(Serialize)]
struct JsonResponse {
    message: String,
    timestamp: String,
    server: String,
}

#[derive(Serialize)]
struct StatsResponse {
    total_requests: u64,
    uptime_seconds: u64,
    requests_per_second: f64,
}

// Main request handler
async fn handle_request(
    req: Request<IncomingBody>,
    stats: ServerStats,
) -> Result<Response<Full<Bytes>>, Infallible> {
    stats.increment_requests();

    let path = req.uri().path();
    let method = req.method();

    println!(
        "[{}] {} {} - Request #{}",
        Local::now().format("%Y-%m-%d %H:%M:%S"),
        method,
        path,
        stats.get_total_requests()
    );

    let response = match (method, path) {
        (&hyper::Method::GET, "/") => handle_root(),
        (&hyper::Method::GET, "/health") => handle_health(),
        (&hyper::Method::GET, "/stats") => handle_stats(stats),
        (&hyper::Method::GET, path) if path.starts_with("/echo/") => {
            let message = &path[6..];
            handle_echo(message)
        }
        _ => handle_not_found(),
    };

    Ok(response)
}

// Route handlers
fn handle_root() -> Response<Full<Bytes>> {
    let response = JsonResponse {
        message: "Welcome to Rust HTTP Server!".to_string(),
        timestamp: Local::now().to_rfc3339(),
        server: "rust-http-server/1.0".to_string(),
    };
    json_response(StatusCode::OK, &response)
}

fn handle_health() -> Response<Full<Bytes>> {
    let response = JsonResponse {
        message: "Server is healthy".to_string(),
        timestamp: Local::now().to_rfc3339(),
        server: "rust-http-server/1.0".to_string(),
    };
    json_response(StatusCode::OK, &response)
}

fn handle_stats(stats: ServerStats) -> Response<Full<Bytes>> {
    let uptime = stats.get_uptime_seconds();
    let total_requests = stats.get_total_requests();
    let rps = if uptime > 0 {
        total_requests as f64 / uptime as f64
    } else {
        0.0
    };

    let response = StatsResponse {
        total_requests,
        uptime_seconds: uptime,
        requests_per_second: rps,
    };
    json_response(StatusCode::OK, &response)
}

fn handle_echo(message: &str) -> Response<Full<Bytes>> {
    let response = JsonResponse {
        message: format!("Echo: {}", message),
        timestamp: Local::now().to_rfc3339(),
        server: "rust-http-server/1.0".to_string(),
    };
    json_response(StatusCode::OK, &response)
}

fn handle_not_found() -> Response<Full<Bytes>> {
    let response = JsonResponse {
        message: "Not Found".to_string(),
        timestamp: Local::now().to_rfc3339(),
        server: "rust-http-server/1.0".to_string(),
    };
    json_response(StatusCode::NOT_FOUND, &response)
}

// Helper function
fn json_response<T: Serialize>(status: StatusCode, body: &T) -> Response<Full<Bytes>> {
    let json = serde_json::to_string(body).unwrap();
    Response::builder()
        .status(status)
        .header("Content-Type", "application/json")
        .header("Server", "rust-http-server/1.0")
        .body(Full::new(Bytes::from(json)))
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let stats = ServerStats::new();

    println!("🚀 Starting Rust HTTP Server...");
    println!("📡 Listening on http://{}", addr);
    println!("📊 Available endpoints:");
    println!("   GET  /           - Root endpoint");
    println!("   GET  /health     - Health check");
    println!("   GET  /stats      - Server statistics");
    println!("   GET  /echo/:msg  - Echo message");
    println!("\n✨ Server ready! Press Ctrl+C to stop.\n");

    let listener = TcpListener::bind(addr).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let stats_clone = stats.clone();

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(move |req| {
                        let stats = stats_clone.clone();
                        handle_request(req, stats)
                    }),
                )
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}