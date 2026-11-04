import { AppInfoSection } from "./AppInfoSection";
import styles from "./SettingsPage.module.css";

export function SettingsPage() {
  return (
    <section className={styles.page}>
      <h1>Settings</h1>
      <AppInfoSection />
    </section>
  );
}
