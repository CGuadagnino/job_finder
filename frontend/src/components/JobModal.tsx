import { Job } from '../types';

interface JobModalProps {
  job: Job | null;
  onClose: () => void;
}

export function JobModal({ job, onClose }: JobModalProps) {
  if (!job) return null;

  return (
    <div
      className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center p-4 z-20"
      onClick={onClose}
    >
      <div
        className="bg-white rounded-lg max-w-2xl w-full max-h-[80vh] overflow-y-auto p-6"
        onClick={(e) => e.stopPropagation()}
      >
        <div className="flex justify-between items-start mb-4">
          <div>
            <h2 className="text-2xl font-bold text-gray-900">{job.title}</h2>
            <p className="text-lg text-gray-600 mt-1">{job.company}</p>
            <p className="text-gray-500 mt-1">{job.location}</p>
          </div>
          <button
            onClick={onClose}
            className="text-gray-400 hover:text-gray-600 text-2xl"
          >
            &times;
          </button>
        </div>

        <div className="prose prose-sm max-w-none">
          <p className="text-gray-700 whitespace-pre-wrap">{job.description}</p>
        </div>

        <a
          href={job.url}
          target="_blank"
          rel="noopener noreferrer"
          className="mt-6 inline-block bg-blue-600 text-white px-6 py-2 rounded-lg hover:bg-blue-700 transition-colors"
        >
          Apply Now →
        </a>
      </div>
    </div>
  );
}
