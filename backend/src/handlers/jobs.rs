use crate::db::insert_job;
use crate::models::{BulkJobResponse, Job, JobQuery, NewJob};
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};
use sqlx::{Row, postgres::PgPool};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool, // Changed from SqlitePool
}

pub async fn list_jobs_handler(state: State<AppState>, params: Query<JobQuery>) -> Json<Vec<Job>> {
    let pool = &state.pool;
    let params = params.0;

    let rows = sqlx::query(
        r#"
        SELECT id, title, company, location, url, description
        FROM jobs;
        "#,
    )
    .fetch_all(pool)
    .await
    .expect("Failed to fetch jobs from database");

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

    let Some(keyword) = params.keyword.as_deref() else {
        return Json(all_jobs);
    };

    let keyword_lower = keyword.to_lowercase();
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

pub async fn create_job_handler(
    state: State<AppState>,
    payload: Json<NewJob>,
) -> Result<(StatusCode, Json<Job>), StatusCode> {
    let pool = &state.pool;
    let new_job = payload.0;

    let id = insert_job(pool, &new_job)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let job = Job {
        id: id, // Changed: removed "as i32"
        title: new_job.title,
        company: new_job.company,
        location: new_job.location,
        url: new_job.url,
        description: new_job.description,
    };

    Ok((StatusCode::CREATED, Json(job)))
}

pub async fn bulk_create_jobs_handler(
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
            let id: i32 = row.get("id");
            let job = Job {
                id: id,
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

    Ok((
        StatusCode::CREATED,
        Json(BulkJobResponse {
            jobs: created_jobs,
            inserted: inserted_count,
            skipped: skipped_count,
        }),
    ))
}
