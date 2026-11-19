// Helpers for rendering the app in component tests.
import { QueryClientProvider } from "@tanstack/react-query";
import { render } from "@testing-library/react";
import { createMemoryRouter, RouterProvider } from "react-router";
import { createQueryClient } from "../api/queryClient";
import { routes } from "../routes";

/**
 * Renders the full app (layout, routes and providers) starting at `path`.
 * Each call gets a fresh `QueryClient`, so no cached data leaks between tests.
 * Mock the commands the page calls with `mockIPC` before rendering.
 */
export function renderRoute(path: string) {
  const router = createMemoryRouter(routes, { initialEntries: [path] });
  const result = render(
    <QueryClientProvider client={createQueryClient()}>
      <RouterProvider router={router} />
    </QueryClientProvider>,
  );
  return { ...result, router };
}
