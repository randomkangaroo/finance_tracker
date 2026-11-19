import { describe, expect, it } from "vitest";
import { errorMessage, isAppError } from "./errors";

describe("isAppError", () => {
  it("accepts the shape a failed command rejects with", () => {
    expect(isAppError({ kind: "Validation", message: "bad" })).toBe(true);
  });

  it.each([
    null,
    "Validation",
    new Error("boom"),
    { kind: "Validation" },
    { kind: "Unknown", message: "bad" },
  ])("rejects %o", (value) => {
    expect(isAppError(value)).toBe(false);
  });
});

describe("errorMessage", () => {
  it.each(["Validation", "Conflict", "NotFound"] as const)(
    "shows %s messages as-is",
    (kind) => {
      expect(errorMessage({ kind, message: "Name must not be empty" })).toBe(
        "Name must not be empty",
      );
    },
  );

  it.each(["Db", "Migration", "Io"] as const)(
    "prefixes %s messages with a generic explanation",
    (kind) => {
      expect(errorMessage({ kind, message: "disk I/O error" })).toBe(
        "Something went wrong while accessing your data. (disk I/O error)",
      );
    },
  );

  it("falls back for errors, strings and anything else", () => {
    expect(errorMessage(new Error("boom"))).toBe("boom");
    expect(errorMessage("plain text")).toBe("plain text");
    expect(errorMessage(42)).toBe("An unexpected error occurred.");
  });
});
