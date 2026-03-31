import { useCleanupStore } from "../lib/stores/cleanup";
import { formatBytes } from "../lib/utils/format";

export function CleanupProgressModal() {
  const { progress } = useCleanupStore();

  if (!progress) {
    return (
      <div className="cleanup-modal">
        <div className="cleanup-modal__content">
          <div className="spinner" />
          <p>Preparing cleanup...</p>
        </div>
      </div>
    );
  }

  const pct =
    progress.total_items > 0
      ? Math.round((progress.completed_items / progress.total_items) * 100)
      : 0;

  return (
    <div className="cleanup-modal">
      <div className="cleanup-modal__content">
        <h2>Cleaning up...</h2>
        <div className="progress-bar">
          <div className="progress-bar__fill" style={{ width: `${pct}%` }} />
        </div>
        <p className="cleanup-modal__status">
          {progress.completed_items} / {progress.total_items} items
        </p>
        {progress.current_item && (
          <p className="cleanup-modal__current">{progress.current_item}</p>
        )}
        <p className="cleanup-modal__freed">
          Freed {formatBytes(progress.bytes_freed)} so far
        </p>
      </div>
    </div>
  );
}
