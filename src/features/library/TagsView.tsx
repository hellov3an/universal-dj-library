import { useEffect, useState } from "react";
import { useTagStore } from "./tagStore";

export function TagsView() {
  const { tags, loadTags, createTag, deleteTag } = useTagStore();
  const [newTagName, setNewTagName] = useState("");
  const [newTagColor, setNewTagColor] = useState("#4CAF50");

  useEffect(() => {
    loadTags();
  }, [loadTags]);

  const handleCreate = async () => {
    if (newTagName.trim()) {
      await createTag(newTagName, newTagColor);
      setNewTagName("");
    }
  };

  return (
    <div style={{ padding: "20px", width: "100%", height: "100%", overflowY: "auto" }}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center", marginBottom: "20px" }}>
        <h2>Tags</h2>
        <div style={{ display: "flex", gap: "10px", alignItems: "center" }}>
          <input 
            type="color" 
            value={newTagColor}
            onChange={e => setNewTagColor(e.target.value)}
            style={{ width: "32px", height: "32px", border: "none", cursor: "pointer", background: "transparent", padding: 0 }}
          />
          <input 
            type="text" 
            value={newTagName} 
            onChange={e => setNewTagName(e.target.value)}
            placeholder="New tag name..."
            className="search-bar__input"
            style={{ padding: "6px 12px", borderRadius: "4px" }}
          />
          <button className="btn btn--primary" onClick={handleCreate}>Create</button>
        </div>
      </div>
      
      <div style={{ display: "flex", flexWrap: "wrap", gap: "12px" }}>
        {tags.map(tag => (
          <div key={tag.id} style={{ display: "flex", alignItems: "center", gap: "8px", padding: "8px 12px", background: "var(--color-surface)", borderRadius: "16px", border: `1px solid ${tag.color || "var(--color-border)"}` }}>
            <div style={{ width: "12px", height: "12px", borderRadius: "50%", background: tag.color || "gray" }} />
            <strong style={{ fontSize: "14px" }}>{tag.name}</strong>
            <span style={{ fontSize: "12px", color: "var(--color-text-tertiary)" }}>({tag.track_count})</span>
            <button 
              style={{ background: "transparent", border: "none", color: "var(--color-text-tertiary)", cursor: "pointer", marginLeft: "4px" }}
              onClick={() => deleteTag(tag.id)}
            >
              ×
            </button>
          </div>
        ))}
        {tags.length === 0 && (
          <div style={{ width: "100%", textAlign: "center", padding: "40px", color: "var(--color-text-tertiary)" }}>
            No tags yet. Create one to organize your tracks!
          </div>
        )}
      </div>
    </div>
  );
}
