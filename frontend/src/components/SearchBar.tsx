interface SearchBarProps {
  searchTerm: string;
  onSearchChange: (value: string) => void;
  resultsCount: number;
  totalCount: number;
}

export function SearchBar({
  searchTerm,
  onSearchChange,
  resultsCount,
  totalCount,
}: SearchBarProps) {
  return (
    <div className="bg-white border-b sticky top-0 z-10">
      <div className="max-w-4xl mx-auto px-4 py-6">
        <h1 className="text-3xl font-bold text-gray-900 mb-4">⚡ Job Finder</h1>

        <input
          type="text"
          value={searchTerm}
          onChange={(e) => onSearchChange(e.target.value)}
          placeholder="Search jobs by title, company, or location..."
          autoFocus
          className="w-full px-4 py-3 text-lg border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
        />

        <p className="mt-2 text-sm text-gray-600">
          Showing {resultsCount} of {totalCount} jobs
        </p>
      </div>
    </div>
  );
}
