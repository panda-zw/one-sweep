import { useScanStore } from "../lib/stores/scan";
import { formatBytes } from "../lib/utils/format";
import { CategoryCard } from "./CategoryCard";

export function ScanView() {
  const { result, isScanning } = useScanStore();

  if (isScanning) {
    return (
      <div className="scan-view scan-view--loading">
        <div className="spinner" />
        <p>Scanning your system for reclaimable space...</p>
      </div>
    );
  }

  if (!result) return null;

  return (
    <div className="scan-view">
      <div className="scan-summary">
        <h2>
          Found{" "}
          <span className="scan-summary__bytes">
            {formatBytes(result.total_bytes)}
          </span>{" "}
          that can be safely freed
        </h2>
        <p className="scan-summary__categories">
          across {result.categories.length} categories
        </p>
      </div>
      <div className="scan-categories">
        {result.categories.map((cat) => (
          <CategoryCard key={cat.category} category={cat} />
        ))}
      </div>
    </div>
  );
}
