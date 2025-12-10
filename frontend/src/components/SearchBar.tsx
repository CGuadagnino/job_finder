import faviconSrc from '../assets/swe-favicon.svg';

interface SearchBarProps {
  searchTerm: string;
  onSearchChange: (value: string) => void;
  resultsCount: number;
}

export function SearchBar({
  searchTerm,
  onSearchChange,
  resultsCount,
}: SearchBarProps) {
  return (
    <div
      className={`bg-white border rounded-lg shadow-sm mx-auto transition-all duration-500 ${
        searchTerm.trim() ? 'w-full' : 'w-[80vw]'
      }`}
    >
      <div className="max-w-4xl mx-auto px-4 py-6">
        <div className="flex items-center gap-3 mb-4">
          <img src={faviconSrc} alt="SWE Favicon" className="w-8 h-8" />
          <h1 className="text-3xl font-bold text-gray-900">
            Software Job Finder
          </h1>
        </div>

        <input
          type="text"
          value={searchTerm}
          onChange={(e) => onSearchChange(e.target.value)}
          placeholder="Search jobs by title, company, or location..."
          autoFocus
          className="w-full px-4 py-3 text-lg border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
        />
        {resultsCount > 0 && (
          <p className="mt-2 text-sm text-gray-600">
            Showing {resultsCount} Jobs
          </p>
        )}
      </div>
    </div>
  );
}
