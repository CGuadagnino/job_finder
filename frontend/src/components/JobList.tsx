import { Job } from '../types';
import { JobCard } from './JobCard';
import { useState, useEffect, useRef } from 'react';

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
  const [visibleCount, setVisibleCount] = useState(50);
  const observerRef = useRef<IntersectionObserver | null>(null);
  const sentinelRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    setVisibleCount(50);
  }, [jobs]);

  useEffect(() => {
    if (observerRef.current) observerRef.current.disconnect();

    observerRef.current = new IntersectionObserver(
      (entries) => {
        if (entries[0].isIntersecting && visibleCount < jobs.length) {
          // Load 50 more items when sentinel comes into view
          setVisibleCount((prev) => Math.min(prev + 50, jobs.length));
        }
      },
      { rootMargin: '200px' }
    );

    if (sentinelRef.current) {
      observerRef.current.observe(sentinelRef.current);
    }

    return () => {
      if (observerRef.current) observerRef.current.disconnect();
    };
  }, [visibleCount, jobs.length]);

  if (jobs.length === 0) {
    return (
      <div className="max-w-4xl mx-auto px-4 py-6">
        <div className="text-center py-12 text-gray-500">
          No jobs found matching &quot;{searchTerm}&quot;
        </div>
      </div>
    );
  }

  const visibleJobs = jobs.slice(0, visibleCount);

  return (
    <div className="max-w-4xl mx-auto px-4 py-6 animate-fadeIn">
      <div className="space-y-3">
        {visibleJobs.map((job, index) => (
          <div
            key={job.id}
            className="animate-slideUp"
            style={{ animationDelay: `${Math.min(index * 30, 500)}ms` }}
          >
            <JobCard
              job={job}
              onClick={() => onJobClick(job)}
              tags={extractTags(job)}
            />
          </div>
        ))}

        {visibleCount < jobs.length && (
          <div ref={sentinelRef} className="py-4 text-center text-gray-500">
            Loading more jobs...
          </div>
        )}
      </div>
    </div>
  );
}
