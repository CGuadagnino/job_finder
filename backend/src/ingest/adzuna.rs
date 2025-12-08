use crate::models::NewJob;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AdzunaResponse {
    pub results: Vec<AdzunaJob>,
}

#[derive(Debug, Deserialize)]
pub struct AdzunaJob {
    pub title: String,
    pub company: AdzunaCompany,
    pub location: AdzunaLocation,
    pub redirect_url: String,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct AdzunaCompany {
    pub display_name: String,
}

#[derive(Debug, Deserialize)]
pub struct AdzunaLocation {
    pub display_name: String,
}

impl From<AdzunaJob> for NewJob {
    fn from(adzuna_job: AdzunaJob) -> Self {
        NewJob {
            title: adzuna_job.title,
            company: adzuna_job.company.display_name,
            location: adzuna_job.location.display_name,
            url: adzuna_job.redirect_url,
            description: adzuna_job.description,
        }
    }
}

pub async fn fetch_jobs_from_adzuna(
    keyword: &str,
    location: &str,
    page: u32,
    remote_only: bool,
    max_days_old: Option<u32>,
) -> Result<Vec<NewJob>, Box<dyn std::error::Error>> {
    let app_id = "92f05d40";
    let app_key = "5c5582e18f9aa65604110fd0f58e9347";

    let mut url = format!(
        "http://api.adzuna.com/v1/api/jobs/us/search/{page}?app_id={app_id}&app_key={app_key}&results_per_page=500&what={keyword}"
    );

    if !location.is_empty() {
        url.push_str(&format!("&where={location}"));
    }

    if remote_only {
        url.push_str("&what_phrase=remote&full_time=1&permanent=1");
    }

    if let Some(days) = max_days_old {
        url.push_str(&format!("&max_days_old={days}"));
    }

    let response: AdzunaResponse = reqwest::get(url).await?.json().await?;

    let new_jobs: Vec<NewJob> = response.results.into_iter().map(|job| job.into()).collect();

    Ok(new_jobs)
}
