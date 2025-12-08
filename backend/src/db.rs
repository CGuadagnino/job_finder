use crate::models::NewJob;
use sqlx::sqlite::SqlitePool;

pub async fn init_db() -> SqlitePool {
    let pool = SqlitePool::connect("sqlite:jobs.db")
        .await
        .expect("Failed to connect to SQLite");

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

pub async fn insert_job(pool: &SqlitePool, new_job: &NewJob) -> Result<i64, sqlx::Error> {
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
