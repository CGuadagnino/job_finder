# SWE Finder

A full-stack job aggregation platform for software engineers, featuring a Rust backend API and React frontend. The application aggregates job listings from the Adzuna API and provides a fast, searchable interface for finding remote and California-based tech positions.

🔗 **Live Demo**: [swefinder.vercel.app](https://swefinder.vercel.app)

## Features

- **Real-time Job Search** — Instant filtering across job titles, companies, locations, and descriptions
- **Smart Tag Extraction** — Automatically detects and displays relevant tech skills (React, Python, AWS, etc.)
- **Infinite Scroll** — Smooth lazy-loading of job results for optimal performance
- **Job Detail Modal** — View full job descriptions with direct apply links
- **Bulk Job Ingestion** — Automated pipeline to fetch jobs from Adzuna API
- **Duplicate Prevention** — URL-based deduplication ensures no duplicate listings

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
| SQLx | Database driver |
| Tokio | Async runtime |
| Reqwest | HTTP client |

### Infrastructure
| Service | Purpose |
|---------|---------|
| **Vercel** | Frontend hosting |
| **Supabase** | PostgreSQL database (production) |
| SQLite | Local development database |
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
│   └── weekly_ingest.ps1     # Batch job ingestion script
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
- **Supabase account** (for production) — [Sign up](https://supabase.com/)

### Backend Setup

1. Navigate to the backend directory:
   ```bash
   cd backend
   ```

2. Create a `.env` file with your Adzuna credentials:
   ```env
   ADZUNA_APP_ID=your_app_id
   ADZUNA_APP_KEY=your_app_key
   ```

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
| `GET` | `/ingest/adzuna` | Fetch jobs from Adzuna API |

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
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    company TEXT NOT NULL,
    location TEXT NOT NULL,
    url TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL
);
```

### Supabase Setup

1. Create a new Supabase project
2. Run the schema above in the SQL editor
3. Copy your project URL and anon key to the frontend `.env`
4. Enable Row Level Security (RLS) and add a read policy for anonymous users:
   ```sql
   CREATE POLICY "Allow anonymous read access" ON jobs
       FOR SELECT USING (true);
   ```

## Job Ingestion

The `weekly_ingest.ps1` PowerShell script automates bulk job ingestion across 50+ search queries covering:

- **Remote positions**: Software engineers, frontend/backend developers, DevOps, ML engineers, mobile developers
- **California positions**: Same role categories for local opportunities

### Running the Ingestion Script

```powershell
# Ensure backend is running first
cd backend
cargo run

# In another terminal, run the ingestion script
./weekly_ingest.ps1
```

The script fetches up to 500 jobs per query (10 pages × 50 results) and handles duplicates automatically via `INSERT OR IGNORE`.

## Environment Variables

### Backend (.env)

| Variable | Description |
|----------|-------------|
| `ADZUNA_APP_ID` | Adzuna API application ID |
| `ADZUNA_APP_KEY` | Adzuna API application key |

### Frontend (.env)

| Variable | Description |
|----------|-------------|
| `VITE_SUPABASE_URL` | Supabase project URL |
| `VITE_SUPABASE_ANON_KEY` | Supabase anonymous/public key |

## Deployment

### Frontend (Vercel)

1. Connect your GitHub repository to Vercel
2. Set the root directory to `frontend`
3. Add environment variables in project settings
4. Deploy automatically on push to `main`

### Database (Supabase)

1. The production database is hosted on Supabase
2. Data can be synced by running the ingestion script locally and inserting directly to Supabase, or by setting up a separate backend deployment

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
