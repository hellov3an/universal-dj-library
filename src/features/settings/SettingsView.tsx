import { useState } from "react";
import { useToastStore } from "../ui/toastStore";
import { useSettingsStore } from "./settingsStore";

export function SettingsView() {
  const [dbPath, setDbPath] = useState("~/.universal-dj-library/database.sqlite");
  const addToast = useToastStore(state => state.addToast);
  const { theme, setTheme } = useSettingsStore();

  const handleSave = () => {
    addToast("Settings saved successfully", "success");
  };

  return (
    <div style={{ padding: "var(--space-4)", maxWidth: "600px", margin: "0 auto" }}>
      <h2 style={{ fontSize: "28px", marginBottom: "32px" }}>Settings</h2>

      <div style={{ display: "flex", flexDirection: "column", gap: "24px" }}>
        <div className="form-group" style={{ display: "flex", flexDirection: "column", gap: "8px" }}>
          <label style={{ fontWeight: 500 }}>Database Path</label>
          <input 
            type="text" 
            value={dbPath}
            onChange={(e) => setDbPath(e.target.value)}
            className="search-bar__input"
            style={{ padding: "8px 12px", borderRadius: "4px" }}
          />
          <span style={{ fontSize: "12px", color: "var(--color-text-tertiary)" }}>
            Restart required to apply changes to database path.
          </span>
        </div>

        <div className="form-group" style={{ display: "flex", flexDirection: "column", gap: "8px" }}>
          <label style={{ fontWeight: 500 }}>Theme</label>
          <select 
            value={theme}
            onChange={(e) => setTheme(e.target.value as "dark" | "light")}
            style={{ 
              padding: "8px 12px", 
              borderRadius: "4px", 
              backgroundColor: "var(--color-bg-secondary)",
              color: "var(--color-text-primary)",
              border: "1px solid var(--color-border)"
            }}
          >
            <option value="dark">Dark</option>
            <option value="light">Light</option>
          </select>
        </div>

        <div style={{ marginTop: "24px", paddingTop: "24px", borderTop: "1px solid var(--color-border)" }}>
          <button className="btn btn--primary" onClick={handleSave}>Save Settings</button>
        </div>
      </div>
    </div>
  );
}
