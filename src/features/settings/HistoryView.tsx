import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

interface HistoryEvent {
  id: string;
  event_type: string;
  description: string;
  details: string | null;
  timestamp: string;
}

export function HistoryView() {
  const [events, setEvents] = useState<HistoryEvent[]>([]);
  const [loading, setLoading] = useState(false);

  const fetchHistory = async () => {
    setLoading(true);
    try {
      const result = await invoke<HistoryEvent[]>("get_history");
      setEvents(result);
    } catch (e) {
      console.error("Failed to load history", e);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchHistory();
  }, []);

  return (
    <div style={{ padding: "20px", height: "100%", overflowY: "auto" }}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "20px" }}>
        <h2 style={{ margin: 0 }}>Activity History</h2>
        <button className="btn btn--secondary" onClick={fetchHistory}>Refresh</button>
      </div>

      {loading && <p>Loading history...</p>}

      <div style={{ display: "flex", flexDirection: "column", gap: "12px" }}>
        {events.map(event => (
          <div key={event.id} style={{ 
            padding: "16px", 
            backgroundColor: "var(--color-surface)", 
            borderRadius: "8px",
            border: "1px solid var(--color-border)",
            display: "flex",
            flexDirection: "column",
            gap: "8px"
          }}>
            <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
              <strong>{event.event_type}</strong>
              <span style={{ fontSize: "12px", color: "var(--color-text-tertiary)" }}>
                {new Date(event.timestamp).toLocaleString()}
              </span>
            </div>
            <div style={{ color: "var(--color-text-secondary)" }}>
              {event.description}
            </div>
            {event.details && (
              <pre style={{ 
                margin: 0, 
                padding: "8px", 
                backgroundColor: "var(--color-surface-elevated)", 
                borderRadius: "4px",
                fontSize: "12px",
                overflowX: "auto"
              }}>
                {event.details}
              </pre>
            )}
          </div>
        ))}

        {events.length === 0 && !loading && (
          <div style={{ textAlign: "center", color: "var(--color-text-tertiary)", padding: "40px" }}>
            No history events recorded yet.
          </div>
        )}
      </div>
    </div>
  );
}
