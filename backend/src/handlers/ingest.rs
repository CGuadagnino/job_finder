use crate::handlers::jobs::AppState;
use crate::ingest::adzuna;
use crate::models::{BulkJobResponse, IngestQuery, Job};
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
};

pub async fn ingest_adzuna_handler(
    state: State<AppState>,
    params: Query<IngestQuery>,
) -> Result<(StatusCode, Json<BulkJobResponse>), StatusCode> {
    let params = params.0;

    let new_jobs = adzuna::fetch_jobs_from_adzuna(
        &params.keyword,
        &params.location,
        1,
        params.remote_only.unwrap_or(false),
        params.max_days_old,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let pool = &state.pool;

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
