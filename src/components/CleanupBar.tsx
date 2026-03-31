import { useScanStore, useSelectedBytes } from "../lib/stores/scan";
import { useCleanupStore } from "../lib/stores/cleanup";
import { formatBytes } from "../lib/utils/format";

export function CleanupBar() {
  const { selectedIds } = useScanStore();
  const selectedBytes = useSelectedBytes();
  const { showConfirmation } = useCleanupStore();

  if (selectedIds.size === 0) return null;

  return (
    <div className="cleanup-bar">
      <span className="cleanup-bar__info">
        {selectedIds.size} items selected ({formatBytes(selectedBytes)})
      </span>
      <button className="cleanup-bar__btn" onClick={showConfirmation}>
        Free {formatBytes(selectedBytes)}
      </button>
    </div>
  );
}
