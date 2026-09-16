import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import { TrackBrowser } from "./features/library/TrackBrowser";
import { DevicesBrowser } from "./features/devices/DevicesBrowser";
import { MetadataEditor } from "./features/library/MetadataEditor";
import { PlaylistView } from "./features/library/PlaylistView";
import { TagsView } from "./features/library/TagsView";
import { WaveformDisplay } from "./features/library/WaveformDisplay";
import { AudioPlayer } from "./features/library/AudioPlayer";
import { CuePointEditor } from "./features/library/CuePointEditor";
import { ToastContainer } from "./features/ui/ToastContainer";
import { SettingsView } from "./features/settings/SettingsView";
import { HistoryView } from "./features/settings/HistoryView";
import { HealthDashboard } from "./features/settings/HealthDashboard";
import { ExportView } from "./features/devices/ExportView";
import { useLibraryStore } from "./features/library/libraryStore";
import { useAnalysisStore } from "./features/library/analysisStore";
import { useSettingsStore } from "./features/settings/settingsStore";
import "./App.css";

/** Library statistics from the Rust backend */
interface LibraryStats {
  track_count: number;
  playlist_count: number;
  tag_count: number;
  total_duration_ms: number;
  total_file_size: number;
}

type SidebarView = "library" | "playlists" | "tags" | "devices" | "health" | "history" | "settings" | "export";
type InspectorTab = "metadata" | "waveform" | "cues" | "analysis";

function App() {
  const [stats, setStats] = useState<LibraryStats | null>(null);
  const [version, setVersion] = useState<string>("");
  const [activeView, setActiveView] = useState<SidebarView>("library");
  const [activeInspectorTab, setActiveInspectorTab] = useState<InspectorTab>("metadata");

  const { loadTracks, setSearchQuery, searchQuery, selectedTrackIds, tracks } = useLibraryStore();
  const { analyzeTracks, isAnalyzing, isPaused, analyzedCount, totalToAnalyze, pauseAnalysis, resumeAnalysis, cancelAnalysis, setupListener } = useAnalysisStore();
  const { theme, setTheme } = useSettingsStore();

  useEffect(() => {
    // Apply initial theme
    document.documentElement.setAttribute("data-theme", theme);
    
    // Load initial stats
    invoke<LibraryStats>("get_library_stats").then(setStats).catch(console.error);
    invoke<string>("get_app_version").then(setVersion).catch(console.error);
    
    // Setup analysis progress listener
    let unlisten: any;
    setupListener().then(fn => unlisten = fn);

    // Global keyboard shortcuts
    const handleKeyDown = (e: KeyboardEvent) => {
      // Undo: Ctrl/Cmd + Z
      if ((e.ctrlKey || e.metaKey) && e.key === 'z' && !e.shiftKey) {
        e.preventDefault();
        console.log("Undo action");
      }
      // Redo: Ctrl/Cmd + Shift + Z or Ctrl/Cmd + Y
      if ((e.ctrlKey || e.metaKey) && ((e.key === 'z' && e.shiftKey) || e.key === 'y')) {
        e.preventDefault();
        console.log("Redo action");
      }
    };
    window.addEventListener("keydown", handleKeyDown);

    return () => {
      if (unlisten) unlisten();
      window.removeEventListener("keydown", handleKeyDown);
    };
  }, [setupListener]);

  const handleImportMusic = async () => {
    try {
      const selectedPath = await open({
        directory: true,
        multiple: false,
        title: "Select Music Folder",
      });

      if (selectedPath) {
        const numTracks = await invoke<number>("scan_directory", { path: selectedPath });
        console.log(`Imported ${numTracks} tracks`);
        
        await loadTracks();
        const updatedStats = await invoke<LibraryStats>("get_library_stats");
        setStats(updatedStats);
      }
    } catch (err) {
      console.error("Failed to import music:", err);
    }
  };

  const handleAnalyzeSelected = async () => {
    if (selectedTrackIds.size === 0) return;
    const ids = Array.from(selectedTrackIds);
    await analyzeTracks(ids);
  };

  const handleAnalyzeAll = async () => {
    const ids = tracks.map(t => t.id);
    if (ids.length === 0) return;
    await analyzeTracks(ids);
  };

  const formatDuration = (ms: number): string => {
    const hours = Math.floor(ms / 3600000);
    const minutes = Math.floor((ms % 3600000) / 60000);
    if (hours > 0) return `${hours}h ${minutes}m`;
    return `${minutes}m`;
  };

  const formatSize = (bytes: number): string => {
    if (bytes === 0) return "0 B";
    const units = ["B", "KB", "MB", "GB", "TB"];
    const i = Math.floor(Math.log(bytes) / Math.log(1024));
    return `${(bytes / Math.pow(1024, i)).toFixed(1)} ${units[i]}`;
  };

  const sidebarItems: {
    view: SidebarView;
    label: string;
    icon: string;
    count?: number;
  }[] = [
    { view: "library", label: "All Tracks", icon: "🎵", count: stats?.track_count },
    { view: "playlists", label: "Playlists", icon: "📋", count: stats?.playlist_count },
    { view: "tags", label: "Tags", icon: "🏷️", count: stats?.tag_count },
    { view: "devices", label: "Devices", icon: "💾" },
    { view: "export", label: "Export", icon: "📤" },
  ];

  const bottomSidebarItems: {
    view: SidebarView;
    label: string;
    icon: string;
  }[] = [
    { view: "health", label: "Health", icon: "❤️" },
    { view: "history", label: "History", icon: "🕒" },
    { view: "settings", label: "Settings", icon: "⚙️" },
  ];

  const inspectorTabs: { tab: InspectorTab; label: string }[] = [
    { tab: "metadata", label: "Metadata" },
    { tab: "waveform", label: "Waveform" },
    { tab: "cues", label: "Cue Points" },
    { tab: "analysis", label: "Analysis" },
  ];

  return (
    <div className="app">
      {/* ── Toolbar ── */}
      <header className="toolbar">
        <div className="toolbar__left">
          <span className="toolbar__title">Universal DJ Library</span>
          {stats && stats.track_count > 0 && (
            <span className="toolbar__stats">
              {stats.track_count.toLocaleString()} tracks ·{" "}
              {formatDuration(stats.total_duration_ms)} ·{" "}
              {formatSize(stats.total_file_size)}
            </span>
          )}
        </div>
        <div className="toolbar__center">
          <div className="search-bar">
            <span className="search-bar__icon">🔍</span>
            <input
              className="search-bar__input"
              type="text"
              placeholder="Search tracks... (artist:, genre:, bpm:)"
              aria-label="Search tracks"
              value={searchQuery}
              onChange={(e) => setSearchQuery(e.target.value)}
            />
          </div>
        </div>
        <div className="toolbar__right">
          {version && (
            <span className="badge badge--info">v{version}</span>
          )}
        </div>
      </header>

      {/* ── Main Content ── */}
      <div className="main">
        {/* ── Sidebar ── */}
        <nav className="sidebar" aria-label="Library navigation">
          <div className="sidebar__section">
            <div className="sidebar__heading">Library</div>
            {sidebarItems.map((item) => (
              <button
                key={item.view}
                className={`sidebar__item ${activeView === item.view ? "sidebar__item--active" : ""}`}
                onClick={() => setActiveView(item.view)}
                aria-current={activeView === item.view ? "page" : undefined}
              >
                <span className="sidebar__item-icon">{item.icon}</span>
                <span>{item.label}</span>
                {item.count !== undefined && (
                  <span className="sidebar__item-count">{item.count.toLocaleString()}</span>
                )}
              </button>
            ))}
          </div>

          <div className="sidebar__section" style={{ marginTop: "auto" }}>
            {bottomSidebarItems.map((item) => (
              <button
                key={item.view}
                className={`sidebar__item ${activeView === item.view ? "sidebar__item--active" : ""}`}
                onClick={() => setActiveView(item.view)}
              >
                <span className="sidebar__item-icon">{item.icon}</span>
                <span>{item.label}</span>
              </button>
            ))}
          </div>
        </nav>

        {/* ── Content ── */}
        <div className="content">
          {/* ── Track Browser ── */}
          <div className="browser" style={{ padding: 0, overflow: "hidden" }}>
            {activeView === "devices" ? (
              <DevicesBrowser />
            ) : activeView === "export" ? (
              <ExportView />
            ) : activeView === "health" ? (
              <HealthDashboard />
            ) : activeView === "history" ? (
              <HistoryView />
            ) : activeView === "settings" ? (
              <SettingsView />
            ) : activeView === "playlists" ? (
              <PlaylistView />
            ) : activeView === "tags" ? (
              <TagsView />
            ) : activeView === "library" ? (
              <>
                <div className="browser-toolbar">
                   <button className="btn btn--secondary" onClick={handleImportMusic}>Import Folder</button>
                   {!isAnalyzing ? (
                     <>
                       <button 
                          className="btn btn--primary" 
                          onClick={handleAnalyzeSelected}
                          disabled={selectedTrackIds.size === 0}
                       >
                         Analyze Selected ({selectedTrackIds.size})
                       </button>
                       <button 
                          className="btn btn--secondary" 
                          onClick={handleAnalyzeAll}
                          disabled={tracks.length === 0}
                       >
                         Analyze All
                       </button>
                     </>
                   ) : (
                     <div style={{ display: "flex", gap: "8px", alignItems: "center" }}>
                        <span style={{ fontSize: "12px", color: "var(--color-text-secondary)" }}>
                          Analyzing {analyzedCount} / {totalToAnalyze}
                        </span>
                        {isPaused ? (
                          <button className="btn btn--primary" onClick={resumeAnalysis}>Resume</button>
                        ) : (
                          <button className="btn btn--secondary" onClick={pauseAnalysis}>Pause</button>
                        )}
                        <button className="btn btn--danger" onClick={cancelAnalysis}>Cancel</button>
                     </div>
                   )}
                </div>
                {stats && stats.track_count === 0 ? (
                  <div className="browser__empty">
                    <div className="browser__empty-icon">🎵</div>
                    <div className="browser__empty-title">
                      Welcome to Universal DJ Library
                    </div>
                    <div className="browser__empty-subtitle">
                      Your music belongs to you — not to your DJ software.
                      <br />
                      Import your music folder to get started.
                    </div>
                    <button className="btn btn--primary" onClick={handleImportMusic}>Import Music</button>
                  </div>
                ) : (
                  <TrackBrowser />
                )}
              </>
            ) : (
              <div className="browser__empty">
                <div className="browser__empty-icon">📁</div>
                <div className="browser__empty-title">Work in Progress</div>
                <div className="browser__empty-subtitle">This view is coming soon.</div>
              </div>
            )}
          </div>

          {/* ── Inspector ── */}
          <div className="inspector">
            <div className="inspector__header">
              {inspectorTabs.map((tab) => (
                <button
                  key={tab.tab}
                  className={`inspector__tab ${
                    activeInspectorTab === tab.tab
                      ? "inspector__tab--active"
                      : ""
                  }`}
                  onClick={() => setActiveInspectorTab(tab.tab)}
                >
                  {tab.label}
                </button>
              ))}
            </div>
            <div className="inspector__body">
              {activeInspectorTab === "metadata" ? (
                <MetadataEditor />
              ) : activeInspectorTab === "waveform" ? (
                <WaveformDisplay />
              ) : activeInspectorTab === "cues" ? (
                <CuePointEditor />
              ) : (
                <div className="inspector__empty">
                  Select a track to view details
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
      
      <AudioPlayer />
      <ToastContainer />
    </div>
  );
}

export default App;
