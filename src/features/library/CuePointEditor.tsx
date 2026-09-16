import { useState } from "react";
import { useLibraryStore } from "./libraryStore";
import { useToastStore } from "../ui/toastStore";

export function CuePointEditor() {
  const { selectedTrackIds, tracks } = useLibraryStore();
  const { addToast } = useToastStore();
  
  const selectedId = Array.from(selectedTrackIds)[0];
  const track = tracks.find(t => t.id === selectedId);

  // In a real app, we'd sync this with the backend. 
  // For MVP we just show a mock UI or local state if the track had cues.
  const [cues] = useState<{ id: string; name: string; position_ms: number; color: string }[]>([
    { id: "1", name: "Intro", position_ms: 15000, color: "#FF5722" },
    { id: "2", name: "Drop", position_ms: 60000, color: "#4CAF50" }
  ]);

  if (!track) {
    return <div className="inspector__empty">Select a track to edit cue points</div>;
  }

  const handleAddCue = () => {
    // Usually adds at current player time
    addToast({ type: "info", message: "Cue point added at current position" });
  };

  const formatTime = (ms: number) => {
    const totalSecs = Math.floor(ms / 1000);
    const mins = Math.floor(totalSecs / 60);
    const secs = totalSecs % 60;
    return `${mins}:${secs.toString().padStart(2, "0")}`;
  };

  return (
    <div style={{ padding: "16px", display: "flex", flexDirection: "column", gap: "16px", height: "100%", overflowY: "auto" }}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
        <h3 style={{ margin: 0, fontSize: "14px" }}>Cue Points</h3>
        <button className="btn btn--secondary" onClick={handleAddCue}>+ Add Cue</button>
      </div>

      <div style={{ display: "flex", flexDirection: "column", gap: "8px" }}>
        {cues.map(cue => (
          <div key={cue.id} style={{ 
            display: "flex", 
            alignItems: "center", 
            justifyContent: "space-between",
            padding: "8px 12px",
            backgroundColor: "var(--color-surface-elevated)",
            borderRadius: "4px",
            borderLeft: `4px solid ${cue.color}`
          }}>
            <div style={{ display: "flex", flexDirection: "column", gap: "4px" }}>
              <span style={{ fontWeight: 500, fontSize: "14px" }}>{cue.name}</span>
              <span style={{ fontSize: "12px", color: "var(--color-text-secondary)", fontFamily: "var(--font-mono)" }}>
                {formatTime(cue.position_ms)}
              </span>
            </div>
            
            <div style={{ display: "flex", gap: "8px" }}>
              <button className="btn btn--secondary" style={{ padding: "4px 8px" }}>Jump</button>
              <button className="btn btn--danger" style={{ padding: "4px 8px" }}>X</button>
            </div>
          </div>
        ))}

        {cues.length === 0 && (
          <div style={{ padding: "20px", textAlign: "center", color: "var(--color-text-tertiary)" }}>
            No cue points set
          </div>
        )}
      </div>
    </div>
  );
}
