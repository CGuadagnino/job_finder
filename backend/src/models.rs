use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
}

#[derive(Serialize)]
pub struct Job {
    pub id: i32,
    pub title: String,
    pub company: String,
    pub location: String,
    pub url: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct NewJob {
    pub title: String,
    pub company: String,
    pub location: String,
    pub url: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct JobQuery {
    pub keyword: Option<String>,
}

#[derive(Deserialize)]
pub struct IngestQuery {
    pub keyword: String,
    pub location: String,
    pub remote_only: Option<bool>,
    pub max_days_old: Option<u32>,
}

#[derive(Serialize)]
pub struct BulkJobResponse {
    pub jobs: Vec<Job>,
    pub inserted: usize,
    pub skipped: usize,
}
