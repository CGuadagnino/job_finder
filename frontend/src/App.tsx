import { useState, useMemo } from 'react';
import { useQuery } from '@tanstack/react-query';
import axios from 'axios';
import { Job } from './types';
import { SearchBar } from './components/SearchBar';
import { JobList } from './components/JobList';
import { JobModal } from './components/JobModal';

function App() {
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedJob, setSelectedJob] = useState<Job | null>(null);

  // Fetch all jobs once on mount
  const { data: jobs = [], isLoading } = useQuery<Job[]>({
    queryKey: ['jobs'],
    queryFn: async () => {
      const response = await axios.get<Job[]>('http://localhost:3000/jobs');
      return response.data;
    },
  });

  function extractTags(job: Job): string[] {
    const keywords = [
      'React',
      'Vue',
      'Angular',
      'JavaScript',
      'TypeScript',
      'Python',
      'Rust',
      'Java',
      'Go',
      'Ruby',
      'PHP',
      'Swift',
      'Kotlin',
      'Remote',
      'Hybrid',
      'Senior',
      'Junior',
      'Full-time',
      'Contract',
      'Frontend',
      'Backend',
      'Full Stack',
      'DevOps',
      'AWS',
      'Docker',
      'Kubernetes',
    ];

    const text = `${job.title} ${job.description}`.toLowerCase();
    const found = keywords.filter((keyword) => {
      const escapedKeyword = keyword.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
      return new RegExp(`\\b${escapedKeyword}\\b`, 'i').test(text);
    });

    return [...new Set(found)].slice(0, 5);
  }

  // Only filter if there's a search term
  const filteredJobs = useMemo(() => {
    // Don't show jobs until user types something
    if (!searchTerm.trim()) return [];

    const search = searchTerm.toLowerCase();

    return jobs.filter((job) => {
      const searchableText = [
        job.title,
        job.company,
        job.location,
        job.description,
      ]
        .join(' ')
        .toLowerCase();

      const regex = new RegExp(`\\b${search}\\b`, 'i');
      return regex.test(searchableText);
    });
  }, [jobs, searchTerm]);

  return (
    <div className="min-h-screen bg-gray-50">
      {isLoading ? (
        <div className="flex items-center justify-center min-h-screen">
          <div className="text-xl text-gray-600">Loading jobs...</div>
        </div>
      ) : (
        <>
          <SearchBar
            searchTerm={searchTerm}
            onSearchChange={setSearchTerm}
            resultsCount={filteredJobs.length}
            totalCount={jobs.length}
          />

          {searchTerm.trim() ? (
            <JobList
              jobs={filteredJobs}
              onJobClick={setSelectedJob}
              searchTerm={searchTerm}
              extractTags={extractTags}
            />
          ) : (
            <div className="max-w-4xl mx-auto px-4 py-12 text-center text-gray-500">
              <p className="text-lg">
                Start typing to search through {jobs.length} jobs
              </p>
              <p className="text-sm mt-2">
                Try: "react", "python", "remote", "senior"
              </p>
            </div>
          )}

          <JobModal job={selectedJob} onClose={() => setSelectedJob(null)} />
        </>
      )}
    </div>
  );
}

export default App;
