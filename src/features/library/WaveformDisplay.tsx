import { useEffect, useRef, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { useLibraryStore } from "./libraryStore";

interface TrackAnalysisData {
  waveform_data?: number[]; // Actually Vec<u8> mapped to number[]
  energy?: number;
  beatgrid_markers?: string; // JSON string
}

interface BeatgridMarker {
  position_ms: number;
  bpm: number;
}

export function WaveformDisplay() {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const { selectedTrackIds, tracks } = useLibraryStore();
  const [analysisData, setAnalysisData] = useState<TrackAnalysisData | null>(null);
  const [loading, setLoading] = useState(false);

  const selectedId = Array.from(selectedTrackIds)[0];
  const track = tracks.find(t => t.id === selectedId);

  useEffect(() => {
    if (!selectedId) {
      setAnalysisData(null);
      return;
    }

    const fetchAnalysis = async () => {
      setLoading(true);
      try {
        const data = await invoke<TrackAnalysisData>("get_track_analysis", { trackId: selectedId });
        setAnalysisData(data);
      } catch (e) {
        console.error("Failed to fetch analysis data", e);
        setAnalysisData(null);
      } finally {
        setLoading(false);
      }
    };

    fetchAnalysis();
  }, [selectedId]);

  useEffect(() => {
    if (!canvasRef.current || !analysisData || !analysisData.waveform_data) return;
    
    const canvas = canvasRef.current;
    const ctx = canvas.getContext("2d");
    if (!ctx) return;

    const waveform = analysisData.waveform_data;
    
    // Clear canvas
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    
    // Draw background
    ctx.fillStyle = "var(--color-surface-elevated)";
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    // Draw waveform
    const width = canvas.width;
    const height = canvas.height;
    const step = width / waveform.length;
    
    ctx.fillStyle = "var(--color-primary)";
    
    for (let i = 0; i < waveform.length; i++) {
      const val = waveform[i];
      // val is 0-255, map to 0-height
      const h = (val / 255) * height;
      const y = (height - h) / 2; // center vertically
      
      ctx.fillRect(i * step, y, step - 1, h);
    }
    
    // Draw beatgrid markers
    if (analysisData.beatgrid_markers && track) {
      try {
        const markers: BeatgridMarker[] = JSON.parse(analysisData.beatgrid_markers);
        const duration = track.metadata.duration_ms || 1; // avoid div by 0
        
        ctx.strokeStyle = "rgba(255, 255, 255, 0.5)";
        ctx.beginPath();
        
        for (const marker of markers) {
          // simple approximation mapping time to X position
          const x = (marker.position_ms / duration) * width;
          ctx.moveTo(x, 0);
          ctx.lineTo(x, height);
        }
        
        ctx.stroke();
      } catch (e) {
        console.error("Failed to parse beatgrid markers", e);
      }
    }
    
  }, [analysisData, track]);

  if (!selectedId) {
    return <div className="inspector__empty">Select a track to view waveform</div>;
  }

  if (loading) {
    return <div className="inspector__empty">Loading analysis...</div>;
  }

  if (!analysisData || !analysisData.waveform_data) {
    return (
      <div className="inspector__empty">
        No analysis data available.<br/>
        Analyze this track to see its waveform.
      </div>
    );
  }

  return (
    <div style={{ padding: "16px", display: "flex", flexDirection: "column", gap: "16px", height: "100%" }}>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "center" }}>
        <h3 style={{ margin: 0, fontSize: "14px" }}>Waveform</h3>
        {analysisData.energy && (
          <span className="badge badge--info">Energy: {analysisData.energy}</span>
        )}
      </div>
      
      <div style={{ flex: 1, position: "relative" }}>
        <canvas 
          ref={canvasRef} 
          width={400} 
          height={150} 
          style={{ 
            width: "100%", 
            height: "100%", 
            borderRadius: "4px",
            border: "1px solid var(--color-border)"
          }} 
        />
      </div>
    </div>
  );
}
