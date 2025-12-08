mod db;
mod handlers;
mod ingest;
mod models;

use axum::{
    Router,
    routing::{get, post},
};
use handlers::jobs::AppState;
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Initialize database
    let pool = db::init_db().await;
    let app_state = AppState { pool };

    // Build router
    let app = Router::new()
        .route("/health", get(handlers::health::health_handler))
        .route(
            "/jobs",
            get(handlers::jobs::list_jobs_handler).post(handlers::jobs::create_job_handler),
        )
        .route("/jobs/bulk", post(handlers::jobs::bulk_create_jobs_handler))
        .route(
            "/ingest/adzuna",
            get(handlers::ingest::ingest_adzuna_handler),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(app_state);

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind address");

    println!(
        "Server listening on http://{}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.expect("Server error")
}
