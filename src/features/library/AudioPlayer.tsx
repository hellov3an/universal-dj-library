import { useRef, useState, useEffect } from "react";
import { convertFileSrc } from "@tauri-apps/api/core";
import { useLibraryStore } from "./libraryStore";

export function AudioPlayer() {
  const { selectedTrackIds, tracks } = useLibraryStore();
  const audioRef = useRef<HTMLAudioElement>(null);
  const [isPlaying, setIsPlaying] = useState(false);
  const [currentTime, setCurrentTime] = useState(0);
  const [duration, setDuration] = useState(0);

  const selectedId = Array.from(selectedTrackIds)[0];
  const track = tracks.find(t => t.id === selectedId);

  useEffect(() => {
    if (!track) {
      setIsPlaying(false);
      setCurrentTime(0);
      setDuration(0);
      return;
    }

    if (audioRef.current) {
      // Use Tauri's protocol to load local files safely
      audioRef.current.src = convertFileSrc(track.file_path);
      audioRef.current.load();
      if (isPlaying) {
        audioRef.current.play().catch(e => {
          console.error("Autoplay failed:", e);
          setIsPlaying(false);
        });
      }
    }
  }, [track]);

  const togglePlay = () => {
    if (!audioRef.current || !track) return;
    
    if (isPlaying) {
      audioRef.current.pause();
    } else {
      audioRef.current.play();
    }
    setIsPlaying(!isPlaying);
  };

  const handleTimeUpdate = () => {
    if (audioRef.current) {
      setCurrentTime(audioRef.current.currentTime);
    }
  };

  const handleLoadedMetadata = () => {
    if (audioRef.current) {
      setDuration(audioRef.current.duration);
    }
  };

  const handleSeek = (e: React.ChangeEvent<HTMLInputElement>) => {
    const time = parseFloat(e.target.value);
    if (audioRef.current) {
      audioRef.current.currentTime = time;
      setCurrentTime(time);
    }
  };

  const formatTime = (secs: number) => {
    if (isNaN(secs)) return "0:00";
    const mins = Math.floor(secs / 60);
    const remainingSecs = Math.floor(secs % 60);
    return `${mins}:${remainingSecs.toString().padStart(2, "0")}`;
  };

  return (
    <div style={{
      display: "flex",
      alignItems: "center",
      gap: "16px",
      padding: "12px 24px",
      backgroundColor: "var(--color-surface)",
      borderTop: "1px solid var(--color-border)",
      height: "80px", // fixed height for player dock
      zIndex: 10
    }}>
      <audio 
        ref={audioRef} 
        onTimeUpdate={handleTimeUpdate} 
        onLoadedMetadata={handleLoadedMetadata}
        onEnded={() => setIsPlaying(false)}
      />

      <div style={{ flex: "0 0 200px", display: "flex", flexDirection: "column", overflow: "hidden" }}>
        {track ? (
          <>
            <div style={{ fontWeight: 600, fontSize: "14px", whiteSpace: "nowrap", overflow: "hidden", textOverflow: "ellipsis" }}>
              {track.metadata.title || track.filename}
            </div>
            <div style={{ fontSize: "12px", color: "var(--color-text-secondary)", whiteSpace: "nowrap", overflow: "hidden", textOverflow: "ellipsis" }}>
              {track.metadata.artist || "Unknown Artist"}
            </div>
          </>
        ) : (
          <div style={{ fontSize: "14px", color: "var(--color-text-tertiary)" }}>
            No track selected
          </div>
        )}
      </div>

      <div style={{ display: "flex", alignItems: "center", gap: "16px", flex: 1, justifyContent: "center" }}>
        <button 
          className="btn btn--secondary" 
          style={{ width: "40px", height: "40px", padding: 0, borderRadius: "50%", display: "flex", alignItems: "center", justifyContent: "center" }}
          onClick={togglePlay}
          disabled={!track}
        >
          {isPlaying ? "⏸" : "▶️"}
        </button>

        <div style={{ display: "flex", alignItems: "center", gap: "12px", flex: 1, maxWidth: "600px" }}>
          <span style={{ fontSize: "12px", fontFamily: "var(--font-mono)", width: "40px", textAlign: "right" }}>
            {formatTime(currentTime)}
          </span>
          
          <input 
            type="range" 
            min={0} 
            max={duration || 100} 
            value={currentTime} 
            onChange={handleSeek}
            disabled={!track}
            style={{ flex: 1, cursor: track ? "pointer" : "default" }}
          />

          <span style={{ fontSize: "12px", fontFamily: "var(--font-mono)", width: "40px" }}>
            {formatTime(duration)}
          </span>
        </div>
      </div>
      
      <div style={{ flex: "0 0 200px" }}>
        {/* Volume control or other tools can go here */}
      </div>
    </div>
  );
}
