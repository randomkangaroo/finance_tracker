import { Link } from "react-router";

export function NotFoundPage() {
  return (
    <section>
      <h1>Page not found</h1>
      <p>
        <Link to="/transactions">Go to transactions</Link>
      </p>
    </section>
  );
}
