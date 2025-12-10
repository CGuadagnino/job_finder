import { useState, useMemo } from 'react';
import { useQuery } from '@tanstack/react-query';
import { supabase } from './lib/supabase';
import { Job } from './types';
import { SearchBar } from './components/SearchBar';
import { JobList } from './components/JobList';
import { JobModal } from './components/JobModal';
import { LoadingSpinner } from './components/loadingSpinner';

function App() {
  const [searchTerm, setSearchTerm] = useState('');
  const [selectedJob, setSelectedJob] = useState<Job | null>(null);

  // Fetch all jobs from Supabase
  const { data: jobs = [], isLoading } = useQuery<Job[]>({
    queryKey: ['jobs'],
    queryFn: async () => {
      let allJobs: Job[] = [];
      let from = 0;
      const batchSize = 1000;

      while (true) {
        const { data, error } = await supabase
          .from('jobs')
          .select('*')
          .order('id', { ascending: true })
          .range(from, from + batchSize - 1);

        if (error) throw error;
        if (!data || data.length === 0) break;

        allJobs = [...allJobs, ...data];
        if (data.length < batchSize) break;

        from += batchSize;
      }

      return allJobs;
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

    const searchTerms = searchTerm.toLowerCase().split(/\s+/); // Split by spaces

    return jobs.filter((job) => {
      const searchableText = [
        job.title,
        job.company,
        job.location,
        job.description,
      ]
        .join(' ')
        .toLowerCase();

      // Check if ANY of the search terms match
      return searchTerms.some((term) => searchableText.includes(term));
    });
  }, [jobs, searchTerm]);

  const hasSearched = searchTerm.trim().length > 0;

  return (
    <div className="min-h-screen bg-gray-100">
      {isLoading ? (
        <LoadingSpinner />
      ) : (
        <>
          <div className={`relative ${hasSearched ? '' : 'min-h-screen'}`}>
            <div
              className={`transition-all duration-500 ${
                hasSearched
                  ? 'relative top-0 p-6'
                  : 'absolute top-1/2 left-0 right-0 -translate-y-1/2'
              }`}
            >
              <SearchBar
                searchTerm={searchTerm}
                onSearchChange={setSearchTerm}
                resultsCount={filteredJobs.length}
              />
            </div>
          </div>

          {searchTerm.trim() && (
            <JobList
              jobs={filteredJobs}
              onJobClick={setSelectedJob}
              searchTerm={searchTerm}
              extractTags={extractTags}
            />
          )}

          <JobModal job={selectedJob} onClose={() => setSelectedJob(null)} />
        </>
      )}
    </div>
  );
}

export default App;
