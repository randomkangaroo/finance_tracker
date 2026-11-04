import { errorMessage } from "../lib/errors";
import styles from "./ErrorMessage.module.css";

type ErrorMessageProps = {
  /** Anything thrown by a command or query; it's turned into readable text. */
  error: unknown;
  /** Optional context shown before the message, e.g. "Couldn't load accounts". */
  title?: string;
};

export function ErrorMessage({ error, title }: ErrorMessageProps) {
  return (
    <div role="alert" className={styles.error}>
      {title && <p className={styles.title}>{title}</p>}
      <p>{errorMessage(error)}</p>
    </div>
  );
}
