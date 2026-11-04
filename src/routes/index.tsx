import { Navigate, type RouteObject } from "react-router";
import { AppLayout } from "../components/AppLayout";
import { AccountDetailPage } from "./accounts/AccountDetailPage";
import { AccountsPage } from "./accounts/AccountsPage";
import { NotFoundPage } from "./not-found/NotFoundPage";
import { SettingsPage } from "./settings/SettingsPage";
import { TransactionsPage } from "./transactions/TransactionsPage";

/**
 * The route tree, kept separate from the router instance so tests can mount
 * it with `createMemoryRouter`.
 */
export const routes: RouteObject[] = [
  {
    element: <AppLayout />,
    children: [
      // Until M3 adds the dashboard, the transaction list is the home screen.
      { index: true, element: <Navigate to="/transactions" replace /> },
      { path: "transactions", element: <TransactionsPage /> },
      { path: "accounts", element: <AccountsPage /> },
      { path: "accounts/:id", element: <AccountDetailPage /> },
      { path: "settings", element: <SettingsPage /> },
      { path: "*", element: <NotFoundPage /> },
    ],
  },
];
