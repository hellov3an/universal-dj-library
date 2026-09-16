import React, { useState, useEffect } from "react";
import { useLibraryStore } from "./libraryStore";

export function MetadataEditor() {
  const { tracks, selectedTrackIds, updateTrack } = useLibraryStore();
  const [formData, setFormData] = useState<any>({});
  
  // Get the selected track (or the first one if multiple are selected)
  const selectedId = Array.from(selectedTrackIds)[0];
  const track = tracks.find(t => t.id === selectedId);

  useEffect(() => {
    if (track) {
      setFormData({
        title: track.metadata.title || "",
        artist: track.metadata.artist || "",
        album: track.metadata.album || "",
        genre: track.metadata.genre || "",
        label: track.metadata.label || "",
        year: track.metadata.year || "",
        comment: track.metadata.comment || "",
      });
    } else {
      setFormData({});
    }
  }, [track]);

  if (!track) {
    return <div className="inspector__empty">Select a track to edit metadata</div>;
  }

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target;
    setFormData((prev: any) => ({ ...prev, [name]: value }));
  };

  const handleSave = async () => {
    if (selectedId) {
      await updateTrack(selectedId, formData);
    }
  };

  return (
    <div className="metadata-editor" style={{ padding: "16px", display: "flex", flexDirection: "column", gap: "12px" }}>
      <div className="form-group" style={{ display: "flex", flexDirection: "column", gap: "4px" }}>
        <label style={{ fontSize: "12px", color: "var(--color-text-tertiary)" }}>Title</label>
        <input 
          type="text" 
          name="title"
          value={formData.title} 
          onChange={handleChange}
          onBlur={handleSave}
          style={{ background: "var(--color-surface-elevated)", border: "1px solid var(--color-border)", color: "var(--color-text-primary)", padding: "4px 8px", borderRadius: "4px" }}
        />
      </div>
      <div className="form-group" style={{ display: "flex", flexDirection: "column", gap: "4px" }}>
        <label style={{ fontSize: "12px", color: "var(--color-text-tertiary)" }}>Artist</label>
        <input 
          type="text" 
          name="artist"
          value={formData.artist} 
          onChange={handleChange}
          onBlur={handleSave}
          style={{ background: "var(--color-surface-elevated)", border: "1px solid var(--color-border)", color: "var(--color-text-primary)", padding: "4px 8px", borderRadius: "4px" }}
        />
      </div>
      <div className="form-group" style={{ display: "flex", flexDirection: "column", gap: "4px" }}>
        <label style={{ fontSize: "12px", color: "var(--color-text-tertiary)" }}>Album</label>
        <input 
          type="text" 
          name="album"
          value={formData.album} 
          onChange={handleChange}
          onBlur={handleSave}
          style={{ background: "var(--color-surface-elevated)", border: "1px solid var(--color-border)", color: "var(--color-text-primary)", padding: "4px 8px", borderRadius: "4px" }}
        />
      </div>
      <div className="form-group" style={{ display: "flex", flexDirection: "column", gap: "4px" }}>
        <label style={{ fontSize: "12px", color: "var(--color-text-tertiary)" }}>Genre</label>
        <input 
          type="text" 
          name="genre"
          value={formData.genre} 
          onChange={handleChange}
          onBlur={handleSave}
          style={{ background: "var(--color-surface-elevated)", border: "1px solid var(--color-border)", color: "var(--color-text-primary)", padding: "4px 8px", borderRadius: "4px" }}
        />
      </div>
      <div className="form-group" style={{ display: "flex", flexDirection: "column", gap: "4px" }}>
        <label style={{ fontSize: "12px", color: "var(--color-text-tertiary)" }}>Year</label>
        <input 
          type="number" 
          name="year"
          value={formData.year} 
          onChange={handleChange}
          onBlur={handleSave}
          style={{ background: "var(--color-surface-elevated)", border: "1px solid var(--color-border)", color: "var(--color-text-primary)", padding: "4px 8px", borderRadius: "4px" }}
        />
      </div>
      <div className="form-group" style={{ display: "flex", flexDirection: "column", gap: "4px" }}>
        <label style={{ fontSize: "12px", color: "var(--color-text-tertiary)" }}>Comment</label>
        <textarea 
          name="comment"
          value={formData.comment} 
          onChange={handleChange}
          onBlur={handleSave}
          style={{ background: "var(--color-surface-elevated)", border: "1px solid var(--color-border)", color: "var(--color-text-primary)", padding: "4px 8px", borderRadius: "4px", minHeight: "60px", resize: "vertical" }}
        />
      </div>
    </div>
  );
}
