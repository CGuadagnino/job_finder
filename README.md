# SWE Finder

A full-stack job aggregation platform for software engineers, featuring a Rust backend API and React frontend. The application aggregates job listings from the Adzuna API and provides a fast, searchable interface for finding remote and California-based tech positions.

🔗 **Live Demo**: [swefinder.vercel.app](https://swefinder.vercel.app)

## Features

- **Real-time Job Search** — Instant filtering across job titles, companies, locations, and descriptions
- **Smart Tag Extraction** — Automatically detects and displays relevant tech skills (React, Python, AWS, etc.)
- **Infinite Scroll** — Smooth lazy-loading of job results for optimal performance
- **Job Detail Modal** — View full job descriptions with direct apply links
- **Bulk Job Ingestion** — Automated pipeline to fetch jobs from Adzuna API and ingest directly to Supabase
- **Duplicate Prevention** — URL-based deduplication ensures no duplicate listings
- **Cloud Database Integration** — Direct ingestion to Supabase PostgreSQL for production-ready deployment

## Tech Stack

### Frontend
| Technology | Purpose |
|------------|---------|
| React 19 | UI framework |
| TypeScript | Type safety |
| Vite | Build tool & dev server |
| TailwindCSS | Styling |
| TanStack Query | Server state management |
| Supabase JS | Database client |

### Backend
| Technology | Purpose |
|------------|---------|
| Rust | Backend language |
| Axum | Web framework |
| SQLx | Database driver (PostgreSQL) |
| Tokio | Async runtime |
| Reqwest | HTTP client |

### Infrastructure
| Service | Purpose |
|---------|---------|
| **Vercel** | Frontend hosting |
| **Supabase** | PostgreSQL database (production & development) |
| Adzuna API | Job data source |

## Project Structure

```
job_finder/
├── backend/
│   ├── src/
│   │   ├── main.rs           # Server entry point & routing
│   │   ├── db.rs             # Database initialization & queries
│   │   ├── models.rs         # Data structures & types
│   │   ├── handlers/
│   │   │   ├── mod.rs
│   │   │   ├── health.rs     # Health check endpoint
│   │   │   ├── jobs.rs       # CRUD operations for jobs
│   │   │   └── ingest.rs     # Adzuna ingestion handler
│   │   └── ingest/
│   │       ├── mod.rs
│   │       └── adzuna.rs     # Adzuna API client
│   ├── Cargo.toml
│   ├── batch_ingest.sh       # Batch job ingestion script (macOS/Linux)
│   └── weekly_ingest.ps1     # Batch job ingestion script (Windows)
│
└── frontend/
    ├── src/
    │   ├── main.tsx          # React entry point
    │   ├── App.tsx           # Main application component
    │   ├── types.ts          # TypeScript interfaces
    │   ├── lib/
    │   │   └── supabase.ts   # Supabase client config
    │   └── components/
    │       ├── SearchBar.tsx # Search input component
    │       ├── JobList.tsx   # Virtualized job list
    │       ├── JobCard.tsx   # Individual job card
    │       └── JobModal.tsx  # Job detail modal
    ├── package.json
    ├── vite.config.ts
    └── tailwind.config.js
```

## Getting Started

### Prerequisites

- **Rust** (latest stable) — [Install](https://rustup.rs/)
- **Node.js** v18+ — [Install](https://nodejs.org/)
- **Adzuna API credentials** — [Sign up](https://developer.adzuna.com/)
- **Supabase account** — [Sign up](https://supabase.com/)

### Backend Setup

1. Navigate to the backend directory:
   ```bash
   cd backend
   ```

2. Create a `.env` file with your Adzuna and Supabase credentials:
   ```env
   ADZUNA_APP_ID=your_app_id
   ADZUNA_APP_KEY=your_app_key
   DATABASE_URL=postgresql://postgres.[PROJECT-REF]:[YOUR-PASSWORD]@aws-0-[REGION].pooler.supabase.com:5432/postgres
   ```
   
   **Get your DATABASE_URL:**
   - Go to Supabase Dashboard → Settings → Database
   - Under "Connection string", select **Session pooler**
   - Copy the URI and replace `[YOUR-PASSWORD]` with your database password

3. Build and run the server:
   ```bash
   cargo run
   ```
   The API will start at `http://127.0.0.1:3000`

### Frontend Setup

1. Navigate to the frontend directory:
   ```bash
   cd frontend
   ```

2. Install dependencies:
   ```bash
   npm install
   ```

3. Create a `.env` file with your Supabase credentials:
   ```env
   VITE_SUPABASE_URL=your_supabase_project_url
   VITE_SUPABASE_ANON_KEY=your_supabase_anon_key
   ```

4. Start the development server:
   ```bash
   npm run dev
   ```
   The app will open at `http://localhost:5173`

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/health` | Health check |
| `GET` | `/jobs` | List all jobs (optional `?keyword=` filter) |
| `POST` | `/jobs` | Create a single job |
| `POST` | `/jobs/bulk` | Bulk create jobs (max 500) |
| `GET` | `/ingest/adzuna` | Fetch jobs from Adzuna API and insert to Supabase |

### Ingest Query Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `keyword` | string | Search term (required) |
| `location` | string | Location filter |
| `remote_only` | boolean | Filter for remote positions |
| `max_days_old` | number | Maximum age of listings |

**Example:**
```bash
curl "http://127.0.0.1:3000/ingest/adzuna?keyword=react+developer&location=california&remote_only=true&max_days_old=30"
```

## Database Schema

### Jobs Table

```sql
CREATE TABLE jobs (
    id SERIAL PRIMARY KEY,
    title TEXT NOT NULL,
    company TEXT NOT NULL,
    location TEXT NOT NULL,
    url TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
```

### Supabase Setup

1. Create a new Supabase project
2. Run the schema above in the SQL editor
3. Copy your project URL and anon key to the frontend `.env`
4. Copy your database connection string (Session pooler) to the backend `.env`
5. Enable Row Level Security (RLS) and add a read policy for anonymous users:
   ```sql
   CREATE POLICY "Allow anonymous read access" ON jobs
       FOR SELECT USING (true);
   ```

## Job Ingestion

Automated ingestion scripts are provided for both macOS/Linux (`batch_ingest.sh`) and Windows (`weekly_ingest.ps1`). These scripts perform bulk job ingestion across 50+ search queries covering:

- **Remote positions**: Software engineers, frontend/backend developers, DevOps, ML engineers, mobile developers
- **California positions**: Same role categories for local opportunities

Each query fetches up to 500 jobs (10 pages × 50 results). The backend automatically:
- Inserts jobs directly to Supabase PostgreSQL
- Handles duplicates via `ON CONFLICT (url) DO NOTHING`
- Returns counts of inserted and skipped jobs

### Running the Ingestion Script

**macOS/Linux:**
```bash
# Ensure backend is running first
cd backend
cargo run

# In another terminal, run the ingestion script
chmod +x batch_ingest.sh
./batch_ingest.sh
```

**Windows (PowerShell):**
```powershell
# Ensure backend is running first
cd backend
cargo run

# In another terminal, run the ingestion script
./weekly_ingest.ps1
```

The ingestion process typically takes 10-15 minutes and can add thousands of jobs to your database.

## Environment Variables

### Backend (.env)

| Variable | Description |
|----------|-------------|
| `ADZUNA_APP_ID` | Adzuna API application ID |
| `ADZUNA_APP_KEY` | Adzuna API application key |
| `DATABASE_URL` | Supabase PostgreSQL connection string (Session pooler) |

### Frontend (.env)

| Variable | Description |
|----------|-------------|
| `VITE_SUPABASE_URL` | Supabase project URL |
| `VITE_SUPABASE_ANON_KEY` | Supabase anonymous/public key |

## Deployment

### Frontend (Vercel)

1. Connect your GitHub repository to Vercel
2. Set the root directory to `frontend`
3. Add environment variables in project settings:
   - `VITE_SUPABASE_URL`
   - `VITE_SUPABASE_ANON_KEY`
4. Deploy automatically on push to `main`

### Backend (Optional)

The backend can be deployed to services like:
- **Fly.io** — Rust-friendly with PostgreSQL support
- **Railway** — Simple deployment with automatic HTTPS
- **Render** — Free tier available

Or run locally and use the ingestion scripts to populate Supabase directly.

### Database (Supabase)

The production database is hosted on Supabase and can be populated by:
1. Running the backend locally with `DATABASE_URL` pointing to your Supabase instance
2. Executing the ingestion scripts (`batch_ingest.sh` or `weekly_ingest.ps1`)
3. Jobs are inserted directly to Supabase in real-time

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is open source and available under the [MIT License](LICENSE).

---

Built with ☕ and Rust 🦀
