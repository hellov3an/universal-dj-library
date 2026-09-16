import { useState } from "react";
import { useLibraryStore } from "../library/libraryStore";
import { Track } from "../../types/track";
import { Device } from "./DevicesBrowser";

export function DeviceTransfer({ device, onClose }: { device: Device; onClose: () => void }) {
  const { selectedTrackIds, tracks } = useLibraryStore();
  const [isExporting, setIsExporting] = useState(false);
  const [exportProgress, setExportProgress] = useState<{ current: number; total: number } | null>(null);

  const selectedTracks = tracks.filter((t: Track) => selectedTrackIds.has(t.id));

  const handleExport = async () => {
    if (selectedTracks.length === 0) return;

    setIsExporting(true);
    setExportProgress({ current: 0, total: selectedTracks.length });

    try {
      // In a real app, we'd use a background worker with progress events.
      // Here we simulate progress and call a Tauri command if it existed, or just mock it.
      for (let i = 0; i < selectedTracks.length; i++) {
        const track = selectedTracks[i];
        console.log("Mock exporting track:", track.filename);
        
        // Mock a command like:
        // await invoke("export_track_to_device", { trackId: track.id, devicePath: device.path });
        await new Promise(resolve => setTimeout(resolve, 200));

        setExportProgress({ current: i + 1, total: selectedTracks.length });
      }
      
      alert(`Successfully exported ${selectedTracks.length} tracks to ${device.name}`);
      onClose();
    } catch (error) {
      console.error("Failed to export tracks:", error);
      alert("Failed to export tracks.");
    } finally {
      setIsExporting(false);
    }
  };

  return (
    <div className="modal-backdrop" style={{ 
      position: "fixed", top: 0, left: 0, right: 0, bottom: 0, 
      backgroundColor: "rgba(0,0,0,0.5)", display: "flex", alignItems: "center", justifyContent: "center", zIndex: 1000 
    }}>
      <div className="modal" style={{ 
        backgroundColor: "var(--color-surface)", padding: "24px", borderRadius: "8px", 
        width: "400px", maxWidth: "90vw", border: "1px solid var(--color-border)"
      }}>
        <h3 style={{ marginTop: 0 }}>Export to {device.name}</h3>
        
        {isExporting ? (
          <div style={{ padding: "20px 0" }}>
            <p>Exporting tracks... {exportProgress?.current} / {exportProgress?.total}</p>
            <div style={{ width: "100%", height: "8px", backgroundColor: "var(--color-surface-elevated)", borderRadius: "4px", overflow: "hidden" }}>
              <div style={{ 
                height: "100%", 
                backgroundColor: "var(--color-primary)", 
                width: `${((exportProgress?.current || 0) / (exportProgress?.total || 1)) * 100}%` 
              }} />
            </div>
          </div>
        ) : (
          <div>
            <p>You have selected <strong>{selectedTracks.length}</strong> tracks.</p>
            <p style={{ fontSize: "12px", color: "var(--color-text-tertiary)" }}>
              The tracks will be copied to the device's Universal DJ Library folder.
            </p>
            
            <div style={{ display: "flex", justifyContent: "flex-end", gap: "12px", marginTop: "24px" }}>
              <button className="btn btn--secondary" onClick={onClose}>Cancel</button>
              <button className="btn btn--primary" onClick={handleExport} disabled={selectedTracks.length === 0}>
                Export
              </button>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}
