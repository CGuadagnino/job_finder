// Bring Types and Functions from external Crates into Scope
use axum::{Json, Router, routing::get};
use serde::Serialize;

// This struct respresents the JSON response
#[derive(Serialize)]
struct HealthResponse {
    status: String,
}

struct Job {
    id: i32,
    title: String,
    company: String,
    location: String,
    url: String,
    description: String,
}

// Get for health Handler
async fn health_handler() -> Json<HealthResponse> {
    // Create an instance wrap in Json
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

// Handler for GET /jobs
async fn list_jobs_handler() -> Json<Vec<Job>> {
    let jobs: Vec<Job> = vec![
        Job {
            id: 1,
            title: "Rust Backend Engineer".to_string(),
            company: "ExampleCorp".to_string(),
            location: "Remote".to_string(),
            url: "https://example.com/jobs/1".to_string(),
            description: "Work on backend services using Rust and modern web technologies."
                .to_string(),
        },
        Job {
            id: 2,
            title: "Full Stack Engineer (React + Rust)".to_string(),
            company: "DevTools Inc.".to_string(),
            location: "Austin, TX".to_string(),
            url: "https://example.com/jobs/2".to_string(),
            description: "Build full stack features with a React frontend and Rust backend."
                .to_string(),
        },
    ];

    // Wrap the vector in Json so Axum will serialize it to a JSON response body
    Json(jobs)
}

// Function for asynic program using Tokio
#[tokio::main]
async fn main() {
    // Build a router that can handle GET /health
    let app = Router::new().route("/health", get(health_handler));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to Bind Address");

    println!(
        "Server Listening on http://{}",
        listener.local_addr().unwrap()
    );

    // Start serving the application
    axum::serve(listener, app).await.expect("Server Error")
}
