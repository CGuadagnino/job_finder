import { useState, useMemo } from 'react';
import { useQuery } from '@tanstack/react-query';
import axios from 'axios';

// Type definitions
interface Job {
  id: number;
  title: string;
  company: string;
  location: string;
  url: string;
  description: string;
}

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
    const found = keywords.filter((keyword) =>
      new RegExp(`\\b${keyword.toLowerCase()}\\b`).test(text)
    );

    return [...new Set(found)].slice(0, 5); // Max 5 unique tags
  }

  const filteredJobs = useMemo(() => {
    if (!searchTerm.trim()) return jobs;

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

      // Use word boundaries to avoid partial matches
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
          {/* Header */}
          <div className="bg-white border-b sticky top-0 z-10">
            <div className="max-w-4xl mx-auto px-4 py-6">
              <h1 className="text-3xl font-bold text-gray-900 mb-4">
                Job Finder
              </h1>

              {/* Search Input */}
              <input
                type="text"
                value={searchTerm}
                onChange={(e) => setSearchTerm(e.target.value)}
                placeholder="Search jobs by title, company, or location..."
                autoFocus
                className="w-full px-4 py-3 text-lg border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
              />

              {/* Results Count */}
              <p className="mt-2 text-sm text-gray-600">
                Showing {filteredJobs.length} of {jobs.length} jobs
              </p>
            </div>
          </div>

          {/* Job List */}
          <div className="max-w-4xl mx-auto px-4 py-6">
            <div className="space-y-3">
              {filteredJobs.map((job) => {
                const tags = extractTags(job);

                return (
                  <div
                    key={job.id}
                    onClick={() => setSelectedJob(job)}
                    className="bg-white p-4 rounded-lg border border-gray-200 hover:border-blue-500 hover:shadow-md transition-all cursor-pointer"
                  >
                    <h3 className="text-lg font-semibold text-gray-900">
                      {job.title}
                    </h3>
                    <p className="text-gray-600 mt-1">
                      {job.company} • {job.location}
                    </p>

                    {/* Tags */}
                    {tags.length > 0 && (
                      <div className="flex flex-wrap gap-2 mt-2">
                        {tags.map((tag) => (
                          <span
                            key={tag}
                            className="px-2 py-1 text-xs font-medium bg-blue-100 text-blue-700 rounded"
                          >
                            {tag}
                          </span>
                        ))}
                      </div>
                    )}
                  </div>
                );
              })}

              {filteredJobs.length === 0 && (
                <div className="text-center py-12 text-gray-500">
                  No jobs found matching &quot;{searchTerm}&quot;
                </div>
              )}
            </div>
          </div>

          {/* Job Detail Modal */}
          {selectedJob && (
            <div
              className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-20"
              onClick={() => setSelectedJob(null)}
            >
              <div
                className="bg-white rounded-lg max-w-2xl w-full max-h-[80vh] overflow-y-auto p-6"
                onClick={(e) => e.stopPropagation()}
              >
                <div className="flex justify-between items-start mb-4">
                  <div>
                    <h2 className="text-2xl font-bold text-gray-900">
                      {selectedJob.title}
                    </h2>
                    <p className="text-lg text-gray-600 mt-1">
                      {selectedJob.company}
                    </p>
                    <p className="text-gray-500 mt-1">{selectedJob.location}</p>
                  </div>
                  <button
                    onClick={() => setSelectedJob(null)}
                    className="text-gray-400 hover:text-gray-600 text-2xl"
                  >
                    &times;
                  </button>
                </div>

                <div className="prose prose-sm max-w-none">
                  <p className="text-gray-700 whitespace-pre-wrap">
                    {selectedJob.description}
                  </p>
                </div>

                <a
                  href={selectedJob.url}
                  target="_blank"
                  rel="noopener noreferrer"
                  className="mt-6 inline-block bg-blue-600 text-white px-6 py-2 rounded-lg hover:bg-blue-700 transition-colors"
                >
                  Apply Now →
                </a>
              </div>
            </div>
          )}
        </>
      )}
    </div>
  );
}

export default App;
