import { NavLink, Outlet } from "react-router";
import styles from "./AppLayout.module.css";

const NAV_ITEMS = [
  { to: "/transactions", label: "Transactions" },
  { to: "/accounts", label: "Accounts" },
  { to: "/settings", label: "Settings" },
];

/** The app shell: sidebar navigation beside the current route's content. */
export function AppLayout() {
  return (
    <div className={styles.layout}>
      <nav className={styles.sidebar} aria-label="Main">
        <p className={styles.brand}>Finance Tracker</p>
        <ul className={styles.navList}>
          {NAV_ITEMS.map(({ to, label }) => (
            <li key={to}>
              <NavLink
                to={to}
                className={({ isActive }) =>
                  isActive
                    ? `${styles.navLink} ${styles.active}`
                    : styles.navLink
                }
              >
                {label}
              </NavLink>
            </li>
          ))}
        </ul>
      </nav>
      <main className={styles.content}>
        <Outlet />
      </main>
    </div>
  );
}
