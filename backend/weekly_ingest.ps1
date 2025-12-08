# backend/weekly_ingest.ps1

Write-Host "Starting batch job ingestion..."

# ===== REMOTE JOBS =====
Write-Host "`n Fetching remote jobs..."

# General roles
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=software+engineer&location=&remote_only=true&max_days_old=30" -Method Get

# Frontend
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=frontend+developer&location=&remote_only=true&max_days_old=30" -Method Get
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=react+developer&location=&remote_only=true&max_days_old=30" -Method Get

# Backend
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=backend+developer&location=&remote_only=true&max_days_old=30" -Method Get
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=python+developer&location=&remote_only=true&max_days_old=30" -Method Get
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=rust+developer&location=&remote_only=true&max_days_old=30" -Method Get

# Full Stack
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=full+stack+developer&location=&remote_only=true&max_days_old=30" -Method Get

# ===== CALIFORNIA JOBS =====
Write-Host "`n Fetching California jobs..."

# General roles
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=software+engineer&location=california&max_days_old=30" -Method Get

# Frontend
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=frontend+developer&location=california&max_days_old=30" -Method Get
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=react+developer&location=california&max_days_old=30" -Method Get

# Backend
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=backend+developer&location=california&max_days_old=30" -Method Get
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=python+developer&location=california&max_days_old=30" -Method Get
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=rust+developer&location=california&max_days_old=30" -Method Get

# Full Stack
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=full+stack+developer&location=california&max_days_old=30" -Method Get

Write-Host "`n Batch ingestion complete!"
Write-Host "Checking total job count..."

$totalJobs = (Invoke-RestMethod -Uri "http://127.0.0.1:3000/jobs" -Method Get).Count
Write-Host "Total jobs in database: $totalJobs"