export type SafetyLevel = "green" | "yellow" | "red";

export type ScanCategory =
  | "node_dependencies"
  | "node_caches"
  | "docker_images"
  | "docker_build_cache"
  | "xcode_simulators"
  | "xcode_derived_data"
  | "gradle_cache"
  | "system_caches"
  | "homebrew_cache";

export interface ScanItem {
  id: string;
  path: string;
  display_name: string;
  description: string;
  size_bytes: number;
  safety: SafetyLevel;
  category: ScanCategory;
  last_modified: number | null;
}

export interface CategoryResult {
  category: ScanCategory;
  display_name: string;
  description: string;
  total_bytes: number;
  items: ScanItem[];
}

export interface ScanResult {
  started_at: number;
  completed_at: number | null;
  total_bytes: number;
  categories: CategoryResult[];
}

export interface CleanupProgress {
  total_items: number;
  completed_items: number;
  current_item: string | null;
  bytes_freed: number;
  status: "pending" | "in_progress" | "completed" | { failed: { error: string } };
  results: CleanupItemResult[];
}

export interface CleanupItemResult {
  item_id: string;
  path: string;
  size_bytes: number;
  success: boolean;
  error: string | null;
}

export interface AuditEntry {
  id: number;
  item_path: string;
  item_display_name: string;
  category: string;
  size_bytes: number;
  deleted_at: number;
  success: boolean;
  error_message: string | null;
}
