#!/bin/bash

echo "Starting comprehensive batch job ingestion..."
echo "Target: Remote + California positions across 50+ search queries"

# ===== REMOTE JOBS =====
echo ""
echo "========== FETCHING REMOTE JOBS =========="

# Core Software Engineering
echo ""
echo "Core Engineering Roles..."
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=software+engineer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=software+developer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=full+stack+developer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=full+stack+engineer&location=&remote_only=true&max_days_old=30"

# Frontend Specializations
echo ""
echo "Frontend Roles..."
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=frontend+developer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=frontend+engineer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=react+developer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=react+engineer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=vue+developer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=angular+developer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=javascript+developer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=typescript+developer&location=&remote_only=true&max_days_old=30"

# Backend Specializations
echo ""
echo "Backend Roles..."
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=backend+developer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=backend+engineer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=python+developer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=python+engineer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=rust+developer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=go+developer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=golang+developer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=java+developer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=node+developer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=nodejs+developer&location=&remote_only=true&max_days_old=30"

# DevOps & Infrastructure
echo ""
echo "DevOps & Infrastructure..."
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=devops+engineer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=cloud+engineer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=site+reliability+engineer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=platform+engineer&location=&remote_only=true&max_days_old=30"

# Data & ML
echo ""
echo "Data & ML Roles..."
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=data+engineer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=machine+learning+engineer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=ML+engineer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=data+scientist&location=&remote_only=true&max_days_old=30"

# Mobile
echo ""
echo "Mobile Development..."
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=mobile+developer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=ios+developer&location=&remote_only=true&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=android+developer&location=&remote_only=true&max_days_old=30"


# ===== CALIFORNIA JOBS =====
echo ""
echo "========== FETCHING CALIFORNIA JOBS =========="

# Core Software Engineering
echo ""
echo "Core Engineering Roles..."
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=software+engineer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=software+developer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=full+stack+developer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=full+stack+engineer&location=california&max_days_old=30"

# Frontend Specializations
echo ""
echo "Frontend Roles..."
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=frontend+developer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=frontend+engineer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=react+developer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=react+engineer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=vue+developer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=angular+developer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=javascript+developer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=typescript+developer&location=california&max_days_old=30"

# Backend Specializations
echo ""
echo "Backend Roles..."
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=backend+developer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=backend+engineer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=python+developer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=python+engineer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=rust+developer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=go+developer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=golang+developer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=java+developer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=node+developer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=nodejs+developer&location=california&max_days_old=30"

# DevOps & Infrastructure
echo ""
echo "DevOps & Infrastructure..."
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=devops+engineer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=cloud+engineer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=site+reliability+engineer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=platform+engineer&location=california&max_days_old=30"

# Data & ML
echo ""
echo "Data & ML Roles..."
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=data+engineer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=machine+learning+engineer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=ML+engineer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=data+scientist&location=california&max_days_old=30"

# Mobile
echo ""
echo "Mobile Development..."
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=mobile+developer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=ios+developer&location=california&max_days_old=30"
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=android+developer&location=california&max_days_old=30"

echo ""
echo "=========================================="
echo "Batch ingestion complete!"
echo "=========================================="
echo ""
echo "Checking total job count..."

totalJobs=$(curl -s "http://127.0.0.1:3000/jobs" | jq '. | length')
echo "Total jobs in database: $totalJobs"
echo ""
echo "Script finished successfully!"