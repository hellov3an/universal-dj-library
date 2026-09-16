import { useState } from "react";
import { useToastStore } from "../ui/toastStore";

export function ExportView() {
  const [isExporting, setIsExporting] = useState(false);
  const { addToast } = useToastStore();

  const handleExport = async (format: string) => {
    setIsExporting(true);
    try {
      // Mock call to export
      await new Promise(r => setTimeout(r, 1000));
      addToast({ type: "success", message: `Successfully exported to ${format}` });
    } catch (e) {
      addToast({ type: "error", message: `Failed to export: ${e}` });
    } finally {
      setIsExporting(false);
    }
  };

  return (
    <div style={{ padding: "20px", display: "flex", flexDirection: "column", gap: "24px" }}>
      <h2 style={{ margin: 0, fontSize: "24px" }}>Export Library</h2>
      <p style={{ color: "var(--color-text-secondary)", margin: 0 }}>
        Export your Universal DJ Library into formats that can be read by other DJ software.
      </p>

      <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fill, minmax(300px, 1fr))", gap: "16px" }}>
        <div style={{ 
          backgroundColor: "var(--color-surface)", 
          padding: "20px", 
          borderRadius: "8px",
          border: "1px solid var(--color-border)",
          display: "flex",
          flexDirection: "column",
          gap: "12px"
        }}>
          <h3 style={{ margin: 0, fontSize: "18px" }}>Rekordbox (XML)</h3>
          <p style={{ color: "var(--color-text-secondary)", fontSize: "14px", flex: 1, margin: 0 }}>
            Export your library to Rekordbox XML format to use in Pioneer DJ setups.
          </p>
          <button 
            className="btn btn--primary" 
            onClick={() => handleExport("Rekordbox XML")}
            disabled={isExporting}
          >
            Export XML
          </button>
        </div>

        <div style={{ 
          backgroundColor: "var(--color-surface)", 
          padding: "20px", 
          borderRadius: "8px",
          border: "1px solid var(--color-border)",
          display: "flex",
          flexDirection: "column",
          gap: "12px"
        }}>
          <h3 style={{ margin: 0, fontSize: "18px" }}>Traktor (NML)</h3>
          <p style={{ color: "var(--color-text-secondary)", fontSize: "14px", flex: 1, margin: 0 }}>
            Export your library to Traktor NML format for Native Instruments hardware.
          </p>
          <button 
            className="btn btn--primary" 
            onClick={() => handleExport("Traktor NML")}
            disabled={isExporting}
          >
            Export NML
          </button>
        </div>

        <div style={{ 
          backgroundColor: "var(--color-surface)", 
          padding: "20px", 
          borderRadius: "8px",
          border: "1px solid var(--color-border)",
          display: "flex",
          flexDirection: "column",
          gap: "12px"
        }}>
          <h3 style={{ margin: 0, fontSize: "18px" }}>Engine DJ</h3>
          <p style={{ color: "var(--color-text-secondary)", fontSize: "14px", flex: 1, margin: 0 }}>
            Export directly to an Engine DJ database.
          </p>
          <button 
            className="btn btn--primary" 
            onClick={() => handleExport("Engine DJ DB")}
            disabled={isExporting}
          >
            Export DB
          </button>
        </div>
      </div>
    </div>
  );
}
