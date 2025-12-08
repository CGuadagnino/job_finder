# ===== REMOTE JOBS =====

# General Engineering Roles
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=software+engineer&location=&remote_only=true&max_days_old=30" -Method Get
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=software+developer&location=&remote_only=true&max_days_old=30" -Method Get

# Frontend Roles
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=frontend+engineer&location=&remote_only=true&max_days_old=30" -Method Get
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=react+developer&location=&remote_only=true&max_days_old=30" -Method Get
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=javascript+developer&location=&remote_only=true&max_days_old=30" -Method Get

# Backend Roles
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=backend+engineer&location=&remote_only=true&max_days_old=30" -Method Get
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=python+developer&location=&remote_only=true&max_days_old=30" -Method Get
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=rust+developer&location=&remote_only=true&max_days_old=30" -Method Get

# Full Stack
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=full+stack+engineer&location=&remote_only=true&max_days_old=30" -Method Get
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=fullstack+developer&location=&remote_only=true&max_days_old=30" -Method Get

# ===== CALIFORNIA JOBS =====

# General Engineering
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=software+engineer&location=california&remote_only=false&max_days_old=30" -Method Get
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=software+developer&location=california&remote_only=false&max_days_old=30" -Method Get

# Frontend
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=frontend+engineer&location=california&remote_only=false&max_days_old=30" -Method Get
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=react+developer&location=california&remote_only=false&max_days_old=30" -Method Get

# Backend
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=backend+engineer&location=california&remote_only=false&max_days_old=30" -Method Get
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=python+developer&location=california&remote_only=false&max_days_old=30" -Method Get

# Full Stack
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=full+stack+engineer&location=california&remote_only=false&max_days_old=30" -Method Get
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=fullstack+developer&location=california&remote_only=false&max_days_old=30" -Method Get

# Specialized (bonus searches if you want variety)
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=web+developer&location=california&remote_only=false&max_days_old=30" -Method Get
Invoke-RestMethod -Uri "http://127.0.0.1:3000/ingest/adzuna?keyword=typescript+developer&location=&remote_only=true&max_days_old=30" -Method Get

Write-Host "Batch ingestion complete!"