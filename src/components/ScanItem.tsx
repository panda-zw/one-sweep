import type { ScanItem as ScanItemType } from "../lib/types";
import { formatBytes } from "../lib/utils/format";
import { SafetyBadge } from "./SafetyBadge";

interface Props {
  item: ScanItemType;
  selected: boolean;
  onToggle: () => void;
}

export function ScanItemRow({ item, selected, onToggle }: Props) {
  return (
    <label className="scan-item">
      <input
        type="checkbox"
        checked={selected}
        onChange={onToggle}
        disabled={item.safety === "red"}
      />
      <div className="scan-item__info">
        <div className="scan-item__header">
          <span className="scan-item__name">{item.display_name}</span>
          <SafetyBadge level={item.safety} />
        </div>
        <span className="scan-item__desc">{item.description}</span>
        <span className="scan-item__path">{item.path}</span>
      </div>
      <span className="scan-item__size">{formatBytes(item.size_bytes)}</span>
    </label>
  );
}
