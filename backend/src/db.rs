use crate::models::NewJob;
use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn init_db() -> PgPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    println!("Attempting to connect to: {}", database_url); // Add this debug line

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Create table (you might want to use migrations instead)
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS jobs (
        id SERIAL PRIMARY KEY,
        title TEXT NOT NULL,
        company TEXT NOT NULL,
        location TEXT NOT NULL,
        url TEXT NOT NULL UNIQUE,
        description TEXT NOT NULL,
        created_at TIMESTAMPTZ DEFAULT NOW()
    );
    "#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create jobs table");

    pool
}

pub async fn insert_job(pool: &PgPool, new_job: &NewJob) -> Result<i32, sqlx::Error> {
    let result = sqlx::query_scalar::<_, i32>(
        r#"
        INSERT INTO jobs (title, company, location, url, description)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id;
        "#,
    )
    .bind(&new_job.title)
    .bind(&new_job.company)
    .bind(&new_job.location)
    .bind(&new_job.url)
    .bind(&new_job.description)
    .fetch_one(pool)
    .await?;

    Ok(result)
}
