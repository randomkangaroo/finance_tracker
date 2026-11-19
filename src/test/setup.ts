// Runs before every test file (see `test.setupFiles` in vite.config.ts).
import "@testing-library/jest-dom/vitest";
import { clearMocks } from "@tauri-apps/api/mocks";
import { cleanup } from "@testing-library/react";
import { afterEach } from "vitest";

afterEach(() => {
  // Vitest globals are off, so Testing Library can't register this itself.
  cleanup();
  clearMocks();
});
