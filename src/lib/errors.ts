// Turning thrown command errors into text a user can read.
import type { AppError } from "../bindings";

const ERROR_KINDS = new Set<string>([
  "Db",
  "Migration",
  "Io",
  "NotFound",
  "Validation",
  "Conflict",
] satisfies AppError["kind"][]);

/** True if `e` has the `{ kind, message }` shape a failed command rejects with. */
export function isAppError(e: unknown): e is AppError {
  if (typeof e !== "object" || e === null) {
    return false;
  }
  const { kind, message } = e as Record<string, unknown>;
  return typeof kind === "string" && ERROR_KINDS.has(kind) && typeof message === "string";
}

/**
 * A user-facing message for anything a command or query can throw.
 *
 * Validation, conflict and not-found errors carry messages written for the
 * user, so they're shown as-is. Storage errors get a generic prefix, with the
 * technical detail kept after it for bug reports.
 */
export function errorMessage(e: unknown): string {
  if (isAppError(e)) {
    switch (e.kind) {
      case "Validation":
      case "Conflict":
      case "NotFound":
        return e.message;
      case "Db":
      case "Migration":
      case "Io":
        return `Something went wrong while accessing your data. (${e.message})`;
    }
  }
  if (e instanceof Error) {
    return e.message;
  }
  if (typeof e === "string") {
    return e;
  }
  return "An unexpected error occurred.";
}
