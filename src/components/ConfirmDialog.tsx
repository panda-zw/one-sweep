import { useScanStore, useSelectedBytes } from "../lib/stores/scan";
import { useCleanupStore } from "../lib/stores/cleanup";
import { formatBytes } from "../lib/utils/format";

export function ConfirmDialog() {
  const { selectedIds } = useScanStore();
  const selectedBytes = useSelectedBytes();
  const { cancelConfirmation, startCleanup } = useCleanupStore();

  return (
    <div className="cleanup-modal">
      <div className="cleanup-modal__content">
        <h2>Confirm Cleanup</h2>
        <p>
          This will remove <strong>{selectedIds.size} items</strong> and free
          approximately <strong>{formatBytes(selectedBytes)}</strong>.
        </p>
        <p className="confirm__note">
          These items can be re-downloaded or regenerated when needed.
        </p>
        <div className="completion__actions">
          <button
            className="btn btn--primary"
            onClick={() => startCleanup(Array.from(selectedIds))}
          >
            Free {formatBytes(selectedBytes)}
          </button>
          <button className="btn btn--secondary" onClick={cancelConfirmation}>
            Cancel
          </button>
        </div>
      </div>
    </div>
  );
}
