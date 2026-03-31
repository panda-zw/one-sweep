import { create } from "zustand";
import type { AuditEntry } from "../types";
import * as api from "../api";

interface AuditState {
  entries: AuditEntry[];
  isLoading: boolean;
  loadEntries: () => Promise<void>;
}

export const useAuditStore = create<AuditState>((set) => ({
  entries: [],
  isLoading: false,

  async loadEntries() {
    set({ isLoading: true });
    try {
      const entries = await api.getAuditLog();
      set({ entries });
    } finally {
      set({ isLoading: false });
    }
  },
}));
