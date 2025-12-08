import { Job } from '../types';

interface JobCardProps {
  job: Job;
  onClick: () => void;
  tags: string[];
}

export function JobCard({ job, onClick, tags }: JobCardProps) {
  return (
    <div
      onClick={onClick}
      className="bg-white p-4 rounded-lg border border-gray-200 hover:border-blue-500 hover:shadow-md transition-all cursor-pointer"
    >
      <h3 className="text-lg font-semibold text-gray-900">{job.title}</h3>
      <p className="text-gray-600 mt-1">
        {job.company} • {job.location}
      </p>

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
}
