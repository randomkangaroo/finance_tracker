// Temporary page proving the typed command pipeline works end to end.
// Replaced by the real routes and query hooks in M0 0.9.
import { useEffect, useState } from "react";
import { commands, type AppError, type AppInfo } from "./bindings";

export function AppInfoPage() {
  const [info, setInfo] = useState<AppInfo | null>(null);
  const [error, setError] = useState<AppError | null>(null);

  useEffect(() => {
    commands
      .appInfo()
      .then(setInfo)
      .catch((e: AppError) => setError(e));
  }, []);

  if (error) {
    return (
      <p>
        Error ({error.kind}): {error.message}
      </p>
    );
  }
  if (!info) {
    return <p>Loading…</p>;
  }
  return (
    <dl>
      <dt>Version</dt>
      <dd>{info.version}</dd>
      <dt>Database</dt>
      <dd>{info.dbPath}</dd>
      <dt>Schema version</dt>
      <dd>{info.schemaVersion}</dd>
    </dl>
  );
}
