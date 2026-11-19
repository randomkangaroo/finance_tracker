import { mockIPC } from "@tauri-apps/api/mocks";
import { screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { beforeEach, describe, expect, it } from "vitest";
import { renderRoute } from "../test/render";

describe("routes", () => {
  beforeEach(() => {
    // The Settings page loads app info; nothing else calls a command yet.
    mockIPC(() => ({ version: "0.0.0", dbPath: "", schemaVersion: 0 }));
  });

  it("redirects / to the transactions page", async () => {
    const { router } = renderRoute("/");

    expect(
      await screen.findByRole("heading", { name: "Transactions" }),
    ).toBeInTheDocument();
    expect(router.state.location.pathname).toBe("/transactions");
  });

  it("navigates between pages from the sidebar", async () => {
    const user = userEvent.setup();
    renderRoute("/transactions");

    await user.click(screen.getByRole("link", { name: "Accounts" }));
    expect(
      screen.getByRole("heading", { name: "Accounts" }),
    ).toBeInTheDocument();
    expect(screen.getByRole("link", { name: "Accounts" })).toHaveAttribute(
      "aria-current",
      "page",
    );

    await user.click(screen.getByRole("link", { name: "Settings" }));
    expect(
      screen.getByRole("heading", { name: "Settings" }),
    ).toBeInTheDocument();
  });

  it("shows the account id on the account detail page", () => {
    renderRoute("/accounts/7");

    expect(
      screen.getByRole("heading", { name: "Account 7" }),
    ).toBeInTheDocument();
  });

  it("shows a not-found page for unknown paths", () => {
    renderRoute("/nope");

    expect(
      screen.getByRole("heading", { name: "Page not found" }),
    ).toBeInTheDocument();
  });
});
