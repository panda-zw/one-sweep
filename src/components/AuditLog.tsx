import { useEffect } from "react";
import { useAuditStore } from "../lib/stores/audit";
import { formatBytes, formatDate } from "../lib/utils/format";

export function AuditLog() {
  const { entries, isLoading, loadEntries } = useAuditStore();

  useEffect(() => {
    loadEntries();
  }, [loadEntries]);

  if (isLoading) {
    return (
      <div className="audit-log">
        <div className="spinner" />
      </div>
    );
  }

  if (entries.length === 0) {
    return (
      <div className="audit-log">
        <p className="audit-log__empty">No cleanup history yet.</p>
      </div>
    );
  }

  return (
    <div className="audit-log">
      <h2>Cleanup History</h2>
      <div className="audit-log__table">
        <div className="audit-log__header">
          <span>Item</span>
          <span>Size</span>
          <span>Date</span>
          <span>Status</span>
        </div>
        {entries.map((entry) => (
          <div key={entry.id} className="audit-log__row">
            <span className="audit-log__name">
              {entry.item_display_name}
            </span>
            <span className="audit-log__size">
              {formatBytes(entry.size_bytes)}
            </span>
            <span className="audit-log__date">
              {formatDate(entry.deleted_at)}
            </span>
            <span
              className={`audit-log__status ${entry.success ? "success" : "failed"}`}
            >
              {entry.success ? "Removed" : "Failed"}
            </span>
          </div>
        ))}
      </div>
    </div>
  );
}
