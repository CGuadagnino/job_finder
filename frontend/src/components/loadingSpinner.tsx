import { useState, useEffect } from 'react';

const phrases = [
  'Loading jobs...',
  'Powered by React and Rust',
  'Hosted on Vercel',
  'Database by Supabase',
];

export function LoadingSpinner() {
  const [index, setIndex] = useState(0);

  useEffect(() => {
    const interval = setInterval(() => {
      setIndex((prev) => (prev + 1) % phrases.length);
    }, 600);

    return () => clearInterval(interval);
  }, []);

  return (
    <div className="flex flex-col items-center justify-center min-h-screen">
      <div className="h-8 overflow-hidden">
        <div
          className="transition-transform duration-300 ease-in-out"
          style={{ transform: `translateY(-${index * 2}rem)` }}
        >
          {phrases.map((phrase, i) => (
            <div
              key={i}
              className="h-8 flex items-center justify-center text-xl text-gray-600"
            >
              {phrase}
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}
