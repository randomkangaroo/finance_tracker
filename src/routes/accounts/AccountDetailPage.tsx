import { useParams } from "react-router";

export function AccountDetailPage() {
  const { id } = useParams();
  return (
    <section>
      <h1>Account {id}</h1>
      <p>This account's transactions arrive in M1.</p>
    </section>
  );
}
