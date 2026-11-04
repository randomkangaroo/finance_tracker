import { QueryClient } from "@tanstack/react-query";

/**
 * Defaults for a local, single-user app. The data only changes when this app
 * changes it, so there's nothing to gain from refetching on focus or
 * reconnect, and a failed command fails the same way on retry.
 */
export function createQueryClient(): QueryClient {
  return new QueryClient({
    defaultOptions: {
      queries: {
        retry: false,
        refetchOnWindowFocus: false,
        refetchOnReconnect: false,
        staleTime: Infinity,
      },
      mutations: {
        retry: false,
      },
    },
  });
}
