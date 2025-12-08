import { Job } from '../types';
import { JobCard } from './JobCard';

interface JobListProps {
  jobs: Job[];
  onJobClick: (job: Job) => void;
  searchTerm: string;
  extractTags: (job: Job) => string[];
}

export function JobList({
  jobs,
  onJobClick,
  searchTerm,
  extractTags,
}: JobListProps) {
  if (jobs.length === 0) {
    return (
      <div className="max-w-4xl mx-auto px-4 py-6">
        <div className="text-center py-12 text-gray-500">
          No jobs found matching &quot;{searchTerm}&quot;
        </div>
      </div>
    );
  }

  return (
    <div className="max-w-4xl mx-auto px-4 py-6">
      <div className="space-y-3">
        {jobs.map((job) => (
          <JobCard
            key={job.id}
            job={job}
            onClick={() => onJobClick(job)}
            tags={extractTags(job)}
          />
        ))}
      </div>
    </div>
  );
}
