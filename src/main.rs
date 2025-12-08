// Bring Types and Functions from external Crates into Scope
use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use sqlx::{Row, sqlite::SqlitePool};

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

#[derive(Debug, Deserialize)]
// Doesn't require id since it's auto-generated
struct NewJob {
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

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

#[derive(Serialize)]
struct BulkJobResponse {
    jobs: Vec<Job>,
    inserted: usize,
    skipped: usize,
}

// Init the SQLite Database and return a connection pool
async fn init_db() -> SqlitePool {
    // Connect to a local SQLite file named jobs.db
    // If the file does not exist, SQLite will create it
    let pool = SqlitePool::connect("sqlite:jobs.db")
        .await
        .expect("Failed to connect to SQLite");

    // Create the jobs table if it does not already exist
    sqlx::query(
        r#"
		CREATE TABLE IF NOT EXISTS jobs (
			id INTEGER PRIMARY KEY,
			title TEXT NOT NULL,
			company TEXT NOT NULL,
			location TEXT NOT NULL,
			url TEXT NOT NULL UNIQUE,
			description TEXT NOT NULL
		);
		"#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create jobs table");

    pool
}

// Get for health Handler
async fn health_handler() -> Json<HealthResponse> {
    // Create an instance wrap in Json
    Json(HealthResponse {
        status: "ok".to_string(),
    })
}

// Handler for GET /jobs
async fn list_jobs_handler(state: State<AppState>, params: Query<JobQuery>) -> Json<Vec<Job>> {
    // Extract inner values from the Axum extractors
    let pool = &state.pool;
    let params = params.0;

    // Fetch all jobs from the database
    let rows = sqlx::query(
        r#"
		SELECT id, title, company, location, url, description
		FROM jobs;
		"#,
    )
    .fetch_all(pool)
    .await
    .expect("Failed to fetch jobs from database");

    // Map database rows into our Job struct
    let mut all_jobs: Vec<Job> = Vec::new();

    for row in rows {
        let job = Job {
            id: row.get("id"),
            title: row.get("title"),
            company: row.get("company"),
            location: row.get("location"),
            url: row.get("url"),
            description: row.get("description"),
        };
        all_jobs.push(job);
    }

    // If there is no keyword, just return all jobs
    let Some(keyword) = params.keyword.as_deref() else {
        return Json(all_jobs);
    };

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

// Insert new job into the DB and return the Generated 'id'
async fn insert_job(pool: &SqlitePool, new_job: &NewJob) -> Result<i64, sqlx::Error> {
    // Execute an INSERT statement with the values from `new_job`
    let result = sqlx::query(
        r#"
		INSERT INTO jobs (title, company, location, url, description)
		VALUES (?1, ?2, ?3, ?4, ?5);
		"#,
    )
    .bind(&new_job.title)
    .bind(&new_job.company)
    .bind(&new_job.location)
    .bind(&new_job.url)
    .bind(&new_job.description)
    .execute(pool)
    .await?;

    let id = result.last_insert_rowid();
    Ok(id)
}

async fn create_job_handler(
    state: State<AppState>,
    payload: Json<NewJob>,
) -> Result<(StatusCode, Json<Job>), StatusCode> {
    let pool = &state.pool;
    let new_job = payload.0;
    let id = insert_job(pool, &new_job)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let job = Job {
        id: id as i32,
        title: new_job.title,
        company: new_job.company,
        location: new_job.location,
        url: new_job.url,
        description: new_job.description,
    };

    Ok((StatusCode::CREATED, Json(job)))
}

async fn bulk_create_jobs_handler(
    state: State<AppState>,
    payload: Json<Vec<NewJob>>,
) -> Result<(StatusCode, Json<BulkJobResponse>), StatusCode> {
    let pool = &state.pool;
    let new_jobs = payload.0;

    if new_jobs.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    if new_jobs.len() > 500 {
        return Err(StatusCode::PAYLOAD_TOO_LARGE);
    }

    let mut transaction = pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut created_jobs: Vec<Job> = Vec::new();
    let mut inserted_count = 0;
    let mut skipped_count = 0;

    for new_job in new_jobs {
        let result = sqlx::query(
            r#"
		INSERT OR IGNORE INTO jobs (title, company, location, url, description)
		VALUES (?1, ?2, ?3, ?4, ?5);
		"#,
        )
        .bind(&new_job.title)
        .bind(&new_job.company)
        .bind(&new_job.location)
        .bind(&new_job.url)
        .bind(&new_job.description)
        .execute(transaction.as_mut())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if result.rows_affected() > 0 {
            let id = result.last_insert_rowid();
            let job = Job {
                id: id as i32,
                title: new_job.title,
                company: new_job.company,
                location: new_job.location,
                url: new_job.url,
                description: new_job.description,
            };
            created_jobs.push(job);
            inserted_count += 1;
        } else {
            skipped_count += 1;
        }
    }

    transaction
        .commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((
        StatusCode::CREATED,
        Json(BulkJobResponse {
            jobs: created_jobs,
            inserted: inserted_count,
            skipped: skipped_count,
        }),
    ))
}

// Function for asynic program using Tokio
#[tokio::main]
async fn main() {
    // Init the Database and get a Connection Pool
    let pool = init_db().await;

    let app_state = AppState { pool: pool.clone() };

    // Build a router that can handle GET /health
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/jobs", get(list_jobs_handler).post(create_job_handler))
        .route("/jobs/bulk", post(bulk_create_jobs_handler))
        .with_state(app_state);

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
