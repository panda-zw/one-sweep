import type { SafetyLevel } from "../lib/types";

const labels: Record<SafetyLevel, string> = {
  green: "Safe to remove",
  yellow: "Review first",
  red: "Be careful",
};

export function SafetyBadge({ level }: { level: SafetyLevel }) {
  return (
    <span className={`safety-badge safety-badge--${level}`}>
      {labels[level]}
    </span>
  );
}
