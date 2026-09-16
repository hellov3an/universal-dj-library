import { useEffect, useState } from "react";
import { usePlaylistStore } from "./playlistStore";

export function PlaylistView() {
  const { playlists, loadPlaylists, createPlaylist, deletePlaylist } = usePlaylistStore();
  const [newPlaylistName, setNewPlaylistName] = useState("");

  useEffect(() => {
    loadPlaylists();
  }, [loadPlaylists]);

  const handleCreate = async () => {
    if (newPlaylistName.trim()) {
      await createPlaylist(newPlaylistName);
      setNewPlaylistName("");
    }
  };

  return (
    <div style={{ padding: "20px", width: "100%", height: "100%", overflowY: "auto" }}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "20px" }}>
        <h2>Playlists</h2>
        <div style={{ display: "flex", gap: "10px" }}>
          <input 
            type="text" 
            value={newPlaylistName} 
            onChange={e => setNewPlaylistName(e.target.value)}
            placeholder="New playlist name..."
            className="search-bar__input"
            style={{ padding: "6px 12px", borderRadius: "4px" }}
          />
          <button className="btn btn--primary" onClick={handleCreate}>Create</button>
        </div>
      </div>
      
      <div style={{ display: "flex", flexDirection: "column", gap: "8px" }}>
        {playlists.map(playlist => (
          <div key={playlist.id} style={{ display: "flex", justifyContent: "space-between", padding: "12px", background: "var(--color-surface)", borderRadius: "8px", border: "1px solid var(--color-border)" }}>
            <div style={{ display: "flex", alignItems: "center", gap: "8px" }}>
              <span>{playlist.is_folder ? "📁" : playlist.is_smart ? "⚙️" : "🎵"}</span>
              <strong>{playlist.name}</strong>
              <span style={{ fontSize: "12px", color: "var(--color-text-tertiary)" }}>{playlist.track_count} tracks</span>
            </div>
            <div>
              <button 
                className="btn btn--secondary" 
                style={{ padding: "4px 8px", fontSize: "12px" }}
                onClick={() => deletePlaylist(playlist.id)}
              >
                Delete
              </button>
            </div>
          </div>
        ))}
        {playlists.length === 0 && (
          <div style={{ textAlign: "center", padding: "40px", color: "var(--color-text-tertiary)" }}>
            No playlists yet. Create one to get started!
          </div>
        )}
      </div>
    </div>
  );
}
