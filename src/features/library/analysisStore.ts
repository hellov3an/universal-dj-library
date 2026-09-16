import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import { listen, UnlistenFn } from "@tauri-apps/api/event";

interface AnalysisProgress {
  current_track: string;
  total: number;
  completed: number;
  state: "analyzing" | "paused" | "cancelled" | "finished";
}

interface AnalysisState {
  isAnalyzing: boolean;
  isPaused: boolean;
  analyzedCount: number;
  totalToAnalyze: number;
  currentTrackId: string;
  
  analyzeTracks: (trackIds: string[]) => Promise<void>;
  pauseAnalysis: () => Promise<void>;
  resumeAnalysis: () => Promise<void>;
  cancelAnalysis: () => Promise<void>;
  setupListener: () => Promise<UnlistenFn>;
}

export const useAnalysisStore = create<AnalysisState>((set) => ({
  isAnalyzing: false,
  isPaused: false,
  analyzedCount: 0,
  totalToAnalyze: 0,
  currentTrackId: "",

  analyzeTracks: async (trackIds: string[]) => {
    try {
      await invoke("analyze_tracks", { trackIds });
    } catch (error) {
      console.error("Failed to analyze tracks:", error);
    }
  },

  pauseAnalysis: async () => {
    try {
      await invoke("pause_analysis");
      set({ isPaused: true });
    } catch (error) {
      console.error("Failed to pause analysis:", error);
    }
  },

  resumeAnalysis: async () => {
    try {
      await invoke("resume_analysis");
      set({ isPaused: false });
    } catch (error) {
      console.error("Failed to resume analysis:", error);
    }
  },

  cancelAnalysis: async () => {
    try {
      await invoke("cancel_analysis");
      set({ isAnalyzing: false, isPaused: false, analyzedCount: 0, totalToAnalyze: 0, currentTrackId: "" });
    } catch (error) {
      console.error("Failed to cancel analysis:", error);
    }
  },

  setupListener: async () => {
    return listen<AnalysisProgress>("analysis_progress", (event) => {
      const p = event.payload;
      if (p.state === "finished" || p.state === "cancelled") {
        set({ 
          isAnalyzing: false, 
          isPaused: false, 
          analyzedCount: p.completed, 
          totalToAnalyze: p.total, 
          currentTrackId: "" 
        });
      } else {
        set({ 
          isAnalyzing: true, 
          isPaused: p.state === "paused",
          analyzedCount: p.completed, 
          totalToAnalyze: p.total,
          currentTrackId: p.current_track,
        });
      }
    });
  },
}));
