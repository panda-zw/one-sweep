import { create } from "zustand";
import { listen } from "@tauri-apps/api/event";
import type { CleanupProgress } from "../types";
import * as api from "../api";

type CleanupView = "idle" | "confirming" | "cleaning" | "completed";

interface CleanupState {
  view: CleanupView;
  progress: CleanupProgress | null;
  startCleanup: (itemIds: string[]) => Promise<void>;
  showConfirmation: () => void;
  cancelConfirmation: () => void;
  reset: () => void;
}

export const useCleanupStore = create<CleanupState>((set) => ({
  view: "idle",
  progress: null,

  showConfirmation() {
    set({ view: "confirming" });
  },

  cancelConfirmation() {
    set({ view: "idle" });
  },

  async startCleanup(itemIds: string[]) {
    set({ view: "cleaning", progress: null });

    const unlisten = await listen<CleanupProgress>(
      "cleanup:progress",
      (event) => {
        set({ progress: event.payload });
      },
    );

    try {
      const result = await api.startCleanup(itemIds);
      set({ progress: result, view: "completed" });
    } catch (e) {
      set({
        view: "completed",
        progress: {
          total_items: itemIds.length,
          completed_items: 0,
          current_item: null,
          bytes_freed: 0,
          status: { failed: { error: String(e) } },
          results: [],
        },
      });
    } finally {
      unlisten();
    }
  },

  reset() {
    set({ view: "idle", progress: null });
  },
}));
