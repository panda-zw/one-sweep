import { useCleanupStore } from "../lib/stores/cleanup";
import { useScanStore } from "../lib/stores/scan";
import { formatBytes } from "../lib/utils/format";

export function CompletionScreen() {
  const { progress, reset } = useCleanupStore();
  const { startScan } = useScanStore();

  if (!progress) return null;

  const failed = progress.results.filter((r) => !r.success);
  const isFailed = typeof progress.status === "object" && "failed" in progress.status;

  return (
    <div className="completion">
      <div className="completion__content">
        {isFailed ? (
          <>
            <h2 className="completion__title completion__title--error">
              Cleanup failed
            </h2>
            <p>
              {typeof progress.status === "object" && "failed" in progress.status
                ? progress.status.failed.error
                : "Unknown error"}
            </p>
          </>
        ) : (
          <>
            <h2 className="completion__title">Done!</h2>
            <p className="completion__freed">
              You freed{" "}
              <span className="completion__bytes">
                {formatBytes(progress.bytes_freed)}
              </span>
            </p>
          </>
        )}

        {failed.length > 0 && (
          <div className="completion__failures">
            <p>{failed.length} items could not be removed:</p>
            <ul>
              {failed.map((f) => (
                <li key={f.item_id}>
                  {f.path}: {f.error}
                </li>
              ))}
            </ul>
          </div>
        )}

        <div className="completion__actions">
          <button
            className="btn btn--primary"
            onClick={() => {
              reset();
              startScan();
            }}
          >
            Scan again
          </button>
          <button className="btn btn--secondary" onClick={reset}>
            Close
          </button>
        </div>
      </div>
    </div>
  );
}
