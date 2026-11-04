import { useAppInfo } from "../../api/appInfo";
import { ErrorMessage } from "../../components/ErrorMessage";
import styles from "./AppInfoSection.module.css";

/** Version and storage details, loaded from the `app_info` command. */
export function AppInfoSection() {
  const { data: info, error, isPending } = useAppInfo();

  return (
    <section className={styles.section}>
      <h2 className={styles.heading}>About</h2>
      {isPending ? (
        <p>Loading…</p>
      ) : error ? (
        <ErrorMessage title="Couldn't load app info" error={error} />
      ) : (
        <dl className={styles.list}>
          <dt>Version</dt>
          <dd>{info.version}</dd>
          <dt>Database</dt>
          <dd className={styles.path}>{info.dbPath}</dd>
          <dt>Schema version</dt>
          <dd>{info.schemaVersion}</dd>
        </dl>
      )}
    </section>
  );
}
