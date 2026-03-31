import { useEffect, useState } from "react";
import { useScanStore } from "./lib/stores/scan";
import { useCleanupStore } from "./lib/stores/cleanup";
import { Header } from "./components/Header";
import { EmptyState } from "./components/EmptyState";
import { ScanView } from "./components/ScanView";
import { CleanupBar } from "./components/CleanupBar";
import { ConfirmDialog } from "./components/ConfirmDialog";
import { CleanupProgressModal } from "./components/CleanupProgress";
import { CompletionScreen } from "./components/CompletionScreen";
import { AuditLog } from "./components/AuditLog";
import { Guide } from "./components/Guide";

function App() {
  const [view, setView] = useState<"scan" | "audit" | "guide">("scan");
  const { result, isScanning, loadCachedScan } = useScanStore();
  const cleanupView = useCleanupStore((s) => s.view);

  useEffect(() => {
    loadCachedScan();
  }, [loadCachedScan]);

  return (
    <div className="app">
      <Header view={view} onViewChange={setView} />

      <main className="main">
        {view === "guide" ? (
          <Guide />
        ) : view === "audit" ? (
          <AuditLog />
        ) : !result && !isScanning ? (
          <EmptyState />
        ) : (
          <ScanView />
        )}
      </main>

      {view === "scan" && cleanupView === "idle" && <CleanupBar />}
      {cleanupView === "confirming" && <ConfirmDialog />}
      {cleanupView === "cleaning" && <CleanupProgressModal />}
      {cleanupView === "completed" && <CompletionScreen />}
    </div>
  );
}

export default App;
