import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { useLibraryStore } from "../library/libraryStore";

export function ImportAdapters() {
  const [isImporting, setIsImporting] = useState(false);
  const [importMessage, setImportMessage] = useState("");
  const { loadTracks } = useLibraryStore();

  const handleImportRekordbox = async () => {
    try {
      const file = await open({
        multiple: false,
        directory: false,
        filters: [{ name: "Rekordbox XML", extensions: ["xml"] }],
      });

      if (!file) return;

      setIsImporting(true);
      setImportMessage("Parsing Rekordbox XML...");
      
      const count = await invoke<number>("import_rekordbox", { xmlPath: file });
      
      setImportMessage(`Successfully imported ${count} tracks from Rekordbox!`);
      await loadTracks();
    } catch (error) {
      console.error("Rekordbox import failed:", error);
      setImportMessage(`Error: ${error}`);
    } finally {
      setIsImporting(false);
    }
  };

  const handleImportSerato = async () => {
    try {
      const folder = await open({
        multiple: false,
        directory: true,
      });

      if (!folder) return;

      setIsImporting(true);
      setImportMessage("Parsing Serato Database V2...");
      
      const count = await invoke<number>("import_serato", { folderPath: folder });
      
      setImportMessage(`Successfully imported ${count} tracks from Serato!`);
      await loadTracks();
    } catch (error) {
      console.error("Serato import failed:", error);
      setImportMessage(`Error: ${error}`);
    } finally {
      setIsImporting(false);
    }
  };

  return (
    <div style={{ marginTop: "40px" }}>
      <h2 style={{ fontSize: "24px", fontWeight: "600", marginBottom: "20px" }}>Third-Party Adapters</h2>
      
      <div style={{ display: "grid", gridTemplateColumns: "repeat(auto-fill, minmax(300px, 1fr))", gap: "16px" }}>
        {/* Rekordbox Card */}
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
            Import your Rekordbox library, cue points, and beatgrids via the standard `rekordbox.xml` export.
          </p>
          <button 
            className="btn btn--primary" 
            onClick={handleImportRekordbox}
            disabled={isImporting}
          >
            Import Rekordbox XML
          </button>
        </div>

        {/* Serato Card */}
        <div style={{ 
          backgroundColor: "var(--color-surface)", 
          padding: "20px", 
          borderRadius: "8px",
          border: "1px solid var(--color-border)",
          display: "flex",
          flexDirection: "column",
          gap: "12px"
        }}>
          <h3 style={{ margin: 0, fontSize: "18px" }}>Serato DJ</h3>
          <p style={{ color: "var(--color-text-secondary)", fontSize: "14px", flex: 1, margin: 0 }}>
            Import directly from your `_Serato_` folder. Reads `database V2` and crate files.
          </p>
          <button 
            className="btn btn--secondary" 
            onClick={handleImportSerato}
            disabled={isImporting}
          >
            Import Serato Folder
          </button>
        </div>

        {/* Traktor Card */}
        <div style={{ 
          backgroundColor: "var(--color-surface)", 
          padding: "20px", 
          borderRadius: "8px",
          border: "1px solid var(--color-border)",
          display: "flex",
          flexDirection: "column",
          gap: "12px"
        }}>
          <h3 style={{ margin: 0, fontSize: "18px" }}>Traktor Pro</h3>
          <p style={{ color: "var(--color-text-secondary)", fontSize: "14px", flex: 1, margin: 0 }}>
            Import your Traktor collection via the `collection.nml` file.
          </p>
          <button 
            className="btn btn--secondary" 
            disabled={isImporting}
          >
            Import Traktor NML
          </button>
        </div>

        {/* Engine DJ Card */}
        <div style={{ 
          backgroundColor: "var(--color-surface)", 
          padding: "20px", 
          borderRadius: "8px",
          border: "1px solid var(--color-border)",
          display: "flex",
          flexDirection: "column",
          gap: "12px"
        }}>
          <h3 style={{ margin: 0, fontSize: "18px" }}>Engine DJ (Denon)</h3>
          <p style={{ color: "var(--color-text-secondary)", fontSize: "14px", flex: 1, margin: 0 }}>
            Import from an Engine Library database (`m.db`).
          </p>
          <button 
            className="btn btn--secondary" 
            disabled={isImporting}
          >
            Import Engine DJ
          </button>
        </div>
      </div>

      {importMessage && (
        <div style={{ 
          marginTop: "20px", 
          padding: "12px", 
          backgroundColor: "var(--color-bg-secondary)", 
          borderRadius: "4px",
          borderLeft: "4px solid var(--color-accent-primary)"
        }}>
          {importMessage}
        </div>
      )}
    </div>
  );
}
