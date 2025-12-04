// Bring Types and Functions from external Crates into Scope
use axum::{Json, Router, extract::Query, routing::get};
use serde::{Deserialize, Serialize};

// This struct respresents the JSON response
#[derive(Serialize)]
struct HealthResponse {
    status: String,
}
#[derive(Serialize)]
struct Job {
    id: i32,
    title: String,
    company: String,
    location: String,
    url: String,
    description: String,
}

#[derive(Deserialize)]
struct JobQuery {
    // Optional keyword to filter jobs
    // Assume if none we return all jobs
    keyword: Option<String>,
}

// Get for health Handler
async fn health_handler() -> Json<HealthResponse> {
    // Create an instance wrap in Json
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

// Handler for GET /jobs
async fn list_jobs_handler(Query(params): Query<JobQuery>) -> Json<Vec<Job>> {
    let all_jobs: Vec<Job> = vec![
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

    // If there is no keyword, just return all jobs
    let Some(keyword) = params.keyword.as_deref() else {
        return Json(all_jobs);
    };

    // Lowercase the keyword for case-insensitive matching
    let keyword_lower = keyword.to_lowercase();

    // Filter jobs where title, company, location, or description contains the keyword
    let filtered: Vec<Job> = all_jobs
        .into_iter()
        .filter(|job| {
            job.title.to_lowercase().contains(&keyword_lower)
                || job.company.to_lowercase().contains(&keyword_lower)
                || job.location.to_lowercase().contains(&keyword_lower)
                || job.description.to_lowercase().contains(&keyword_lower)
        })
        .collect();

    Json(filtered)
}

// Function for asynic program using Tokio
#[tokio::main]
async fn main() {
    // Build a router that can handle GET /health
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/jobs", get(list_jobs_handler));

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
