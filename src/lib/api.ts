import { invoke } from "@tauri-apps/api/core";
import type { ScanResult, CleanupProgress, AuditEntry } from "./types";

export async function startScan(): Promise<ScanResult> {
  return invoke<ScanResult>("start_scan");
}

export async function getCachedScan(): Promise<ScanResult | null> {
  return invoke<ScanResult | null>("get_cached_scan");
}

export async function startCleanup(itemIds: string[]): Promise<CleanupProgress> {
  return invoke<CleanupProgress>("start_cleanup", {
    request: { item_ids: itemIds },
  });
}

export async function getAuditLog(
  limit = 100,
  offset = 0,
): Promise<AuditEntry[]> {
  return invoke<AuditEntry[]>("get_audit_log", { limit, offset });
}
