use crate::handlers::jobs::AppState;
use crate::ingest::adzuna;
use crate::models::{BulkJobResponse, IngestQuery, Job};
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use sqlx::Row;

pub async fn ingest_adzuna_handler(
    state: State<AppState>,
    params: Query<IngestQuery>,
) -> Result<(StatusCode, Json<BulkJobResponse>), StatusCode> {
    let params = params.0;
    let pool = &state.pool;

    // Fetch multiple pages (10 pages × 50 results = 500 jobs max)
    let mut all_new_jobs = Vec::new();

    for page in 1..=10 {
        println!("Fetching page {} for keyword: {}", page, params.keyword);

        let jobs = adzuna::fetch_jobs_from_adzuna(
            &params.keyword,
            &params.location,
            page,
            params.remote_only.unwrap_or(false),
            params.max_days_old,
        )
        .await
        .map_err(|e| {
            eprintln!("Error fetching page {}: {:?}", page, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        println!("Got {} jobs from page {}", jobs.len(), page);

        // If we get fewer than 50 jobs, we've reached the end
        let job_count = jobs.len();
        all_new_jobs.extend(jobs);

        if job_count < 50 {
            println!("Reached end of results at page {}", page);
            break;
        }
    }

    println!("Total jobs fetched: {}", all_new_jobs.len());

    // Start transaction to insert all jobs
    let mut transaction = pool
        .begin()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut created_jobs: Vec<Job> = Vec::new();
    let mut inserted_count = 0;
    let mut skipped_count = 0;

    // Insert each job
    for new_job in all_new_jobs {
        // PostgreSQL uses ON CONFLICT instead of INSERT OR IGNORE
        let result = sqlx::query(
            r#"
            INSERT INTO jobs (title, company, location, url, description)
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT (url) DO NOTHING
            RETURNING id;
            "#,
        )
        .bind(&new_job.title)
        .bind(&new_job.company)
        .bind(&new_job.location)
        .bind(&new_job.url)
        .bind(&new_job.description)
        .fetch_optional(transaction.as_mut())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // If RETURNING gives us an id, it was inserted
        if let Some(row) = result {
            let id: i64 = row.get("id");
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
            // Conflict occurred, job was skipped
            skipped_count += 1;
        }
    }

    transaction
        .commit()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("Inserted: {}, Skipped: {}", inserted_count, skipped_count);

    Ok((
        StatusCode::CREATED,
        Json(BulkJobResponse {
            jobs: created_jobs,
            inserted: inserted_count,
            skipped: skipped_count,
        }),
    ))
}
