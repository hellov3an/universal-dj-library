import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";
import { ImportAdapters } from "./ImportAdapters";
import { DeviceTransfer } from "./DeviceTransfer";

export interface Device {
  name: string;
  path: string;
  total_space: number;
  available_space: number;
  is_removable: boolean;
  library_type: string;
}

export function DevicesBrowser() {
  const [devices, setDevices] = useState<Device[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [selectedDevice, setSelectedDevice] = useState<Device | null>(null);

  const fetchDevices = async () => {
    setIsLoading(true);
    try {
      const result = await invoke<Device[]>("get_devices");
      setDevices(result);
    } catch (error) {
      console.error("Failed to fetch devices:", error);
    } finally {
      setIsLoading(false);
    }
  };

  useEffect(() => {
    fetchDevices();

    let unlisten: UnlistenFn | undefined;
    listen<Device[]>("devices_changed", (event) => {
      setDevices(event.payload);
    }).then(fn => unlisten = fn);

    return () => {
      if (unlisten) unlisten();
    };
  }, []);

  const formatBytes = (bytes: number) => {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + " " + sizes[i];
  };

  return (
    <div className="devices-browser" style={{ padding: "20px", display: "flex", flexDirection: "column", gap: "20px", height: "100%", overflowY: "auto" }}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
        <h2 style={{ fontSize: "24px", fontWeight: "600", margin: 0 }}>Connected Devices</h2>
        <button className="btn btn--secondary" onClick={fetchDevices} disabled={isLoading}>
          {isLoading ? "Scanning..." : "Refresh Devices"}
        </button>
      </div>

      <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fill, minmax(300px, 1fr))", gap: "16px" }}>
        {devices.map((device, i) => (
          <div key={i} style={{ 
            backgroundColor: "var(--color-surface)", 
            padding: "20px", 
            borderRadius: "8px",
            border: "1px solid var(--color-border)"
          }}>
            <div style={{ display: "flex", alignItems: "center", gap: "12px", marginBottom: "16px", justifyContent: "space-between" }}>
              <div style={{ display: "flex", alignItems: "center", gap: "12px" }}>
                <div style={{ fontSize: "24px" }}>
                  {device.is_removable ? "💾" : "💽"}
                </div>
                <div>
                  <div style={{ fontWeight: "600", fontSize: "16px" }}>{device.name || "Unknown Volume"}</div>
                  <div style={{ fontSize: "12px", color: "var(--color-text-secondary)", fontFamily: "var(--font-mono)" }}>{device.path}</div>
                </div>
              </div>
              {device.library_type !== "None" && (
                <span className="badge badge--info">{device.library_type}</span>
              )}
            </div>
            
            <div style={{ marginBottom: "8px" }}>
              <div style={{ display: "flex", justifyContent: "space-between", fontSize: "12px", color: "var(--color-text-secondary)", marginBottom: "4px" }}>
                <span>Free Space</span>
                <span>{formatBytes(device.available_space)} / {formatBytes(device.total_space)}</span>
              </div>
              <div style={{ width: "100%", height: "6px", backgroundColor: "var(--color-surface-elevated)", borderRadius: "3px", overflow: "hidden" }}>
                <div style={{ 
                  height: "100%", 
                  backgroundColor: device.is_removable ? "var(--color-accent-primary)" : "var(--color-text-tertiary)", 
                  width: `${((device.total_space - device.available_space) / device.total_space) * 100}%` 
                }}></div>
              </div>
            </div>

            {device.is_removable && (
              <div style={{ marginTop: "16px", display: "flex", gap: "8px" }}>
                <button className="btn btn--primary" style={{ flex: 1 }} onClick={() => setSelectedDevice(device)}>
                  Export to USB
                </button>
                {device.library_type === "Rekordbox" && (
                  <button className="btn btn--secondary" style={{ flex: 1 }}>
                    Sync XML
                  </button>
                )}
              </div>
            )}
          </div>
        ))}

        {devices.length === 0 && !isLoading && (
          <div style={{ color: "var(--color-text-secondary)", padding: "20px 0" }}>
            No storage devices found.
          </div>
        )}
      </div>

      <ImportAdapters />

      {selectedDevice && (
        <DeviceTransfer device={selectedDevice} onClose={() => setSelectedDevice(null)} />
      )}
    </div>
  );
}
