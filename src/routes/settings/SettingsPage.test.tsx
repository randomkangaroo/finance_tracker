import { mockIPC } from "@tauri-apps/api/mocks";
import { screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import type { AppError, AppInfo } from "../../bindings";
import { renderRoute } from "../../test/render";

const appInfo: AppInfo = {
  version: "1.2.3",
  dbPath: "/home/me/.local/share/finance/finance.db",
  schemaVersion: 1,
};

describe("SettingsPage", () => {
  it("shows the app info from the app_info command", async () => {
    mockIPC((cmd) => {
      if (cmd === "app_info") {
        return appInfo;
      }
      throw new Error(`unexpected command: ${cmd}`);
    });

    renderRoute("/settings");

    expect(await screen.findByText("1.2.3")).toBeInTheDocument();
    expect(screen.getByText(appInfo.dbPath)).toBeInTheDocument();
    expect(screen.getByText("Schema version").nextSibling).toHaveTextContent(
      "1",
    );
  });

  it("shows an error when the command fails", async () => {
    const error: AppError = { kind: "Io", message: "disk unplugged" };
    // Failed commands reject with a plain `AppError` object, not an `Error`.
    // eslint-disable-next-line @typescript-eslint/prefer-promise-reject-errors
    mockIPC(() => Promise.reject(error));

    renderRoute("/settings");

    const alert = await screen.findByRole("alert");
    expect(alert).toHaveTextContent("Couldn't load app info");
    expect(alert).toHaveTextContent("disk unplugged");
  });
});
