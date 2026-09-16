import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useToastStore } from "../ui/toastStore";

export interface DuplicateGroup {
  hash: string;
  metadata_key: string;
  track_ids: string[];
}

export interface HealthReport {
  total_tracks: number;
  unanalyzed_tracks: number;
  missing_file_count: number;
  duplicate_hash_groups: DuplicateGroup[];
  duplicate_metadata_groups: DuplicateGroup[];
  tracks_without_metadata: number;
}

export function HealthDashboard() {
  const [report, setReport] = useState<HealthReport | null>(null);
  const [loading, setLoading] = useState(false);
  const { addToast } = useToastStore();

  const fetchHealth = async () => {
    setLoading(true);
    try {
      const data = await invoke<HealthReport>("get_health_report");
      setReport(data);
    } catch (e) {
      console.error(e);
      addToast({ type: "error", message: "Failed to load health report" });
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchHealth();
  }, []);

  if (loading && !report) {
    return <div style={{ padding: "40px", textAlign: "center" }}>Scanning library health...</div>;
  }

  if (!report) return null;

  const totalDuplicates = report.duplicate_hash_groups.length + report.duplicate_metadata_groups.length;

  return (
    <div style={{ padding: "20px", display: "flex", flexDirection: "column", gap: "24px", height: "100%", overflowY: "auto" }}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
        <h2 style={{ margin: 0, fontSize: "24px" }}>Library Health Dashboard</h2>
        <button className="btn btn--secondary" onClick={fetchHealth} disabled={loading}>
          {loading ? "Scanning..." : "Rescan Library"}
        </button>
      </div>

      <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fit, minmax(250px, 1fr))", gap: "16px" }}>
        <div style={{ padding: "20px", backgroundColor: "var(--color-surface)", borderRadius: "8px", border: "1px solid var(--color-border)" }}>
          <h3 style={{ margin: "0 0 16px 0", fontSize: "16px", color: "var(--color-text-secondary)" }}>Missing Files</h3>
          <div style={{ fontSize: "36px", fontWeight: "bold", color: report.missing_file_count > 0 ? "var(--color-danger)" : "var(--color-success)" }}>
            {report.missing_file_count}
          </div>
          {report.missing_file_count > 0 && (
            <button className="btn btn--primary" style={{ marginTop: "16px", width: "100%" }}>Relink Files</button>
          )}
        </div>

        <div style={{ padding: "20px", backgroundColor: "var(--color-surface)", borderRadius: "8px", border: "1px solid var(--color-border)" }}>
          <h3 style={{ margin: "0 0 16px 0", fontSize: "16px", color: "var(--color-text-secondary)" }}>Duplicates</h3>
          <div style={{ fontSize: "36px", fontWeight: "bold", color: totalDuplicates > 0 ? "var(--color-warning)" : "var(--color-success)" }}>
            {totalDuplicates}
          </div>
          {totalDuplicates > 0 && (
            <button className="btn btn--primary" style={{ marginTop: "16px", width: "100%" }}>Resolve Duplicates</button>
          )}
        </div>

        <div style={{ padding: "20px", backgroundColor: "var(--color-surface)", borderRadius: "8px", border: "1px solid var(--color-border)" }}>
          <h3 style={{ margin: "0 0 16px 0", fontSize: "16px", color: "var(--color-text-secondary)" }}>Unanalyzed</h3>
          <div style={{ fontSize: "36px", fontWeight: "bold", color: report.unanalyzed_tracks > 0 ? "var(--color-info)" : "var(--color-success)" }}>
            {report.unanalyzed_tracks}
          </div>
        </div>

        <div style={{ padding: "20px", backgroundColor: "var(--color-surface)", borderRadius: "8px", border: "1px solid var(--color-border)" }}>
          <h3 style={{ margin: "0 0 16px 0", fontSize: "16px", color: "var(--color-text-secondary)" }}>Missing Metadata</h3>
          <div style={{ fontSize: "36px", fontWeight: "bold", color: report.tracks_without_metadata > 0 ? "var(--color-warning)" : "var(--color-success)" }}>
            {report.tracks_without_metadata}
          </div>
        </div>
      </div>
    </div>
  );
}
