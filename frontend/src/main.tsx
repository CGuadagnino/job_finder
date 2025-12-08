import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import './index.css';
import App from './App.tsx';

// Create React Query client with aggressive caching for speed
const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: Infinity, // Data never goes stale - we control refresh manually
      gcTime: 1000 * 60 * 60, // Keep in cache for 1 hour
      refetchOnWindowFocus: false, // Don't refetch when user comes back
    },
  },
});

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <QueryClientProvider client={queryClient}>
      <App />
    </QueryClientProvider>
  </StrictMode>
);
