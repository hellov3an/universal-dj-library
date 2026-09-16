import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import type { Track } from "../../types/track";

interface LibraryState {
  tracks: Track[];
  selectedTrackIds: Set<string>;
  isLoading: boolean;
  searchQuery: string;
  sortColumn: string;
  sortDirection: 'asc' | 'desc';
  
  // Actions
  loadTracks: () => Promise<void>;
  setSearchQuery: (query: string) => void;
  setSort: (column: string) => void;
  selectTrack: (id: string, multi: boolean) => void;
  clearSelection: () => void;
  updateTrack: (id: string, updates: Partial<Track['metadata']>) => Promise<void>;
  updateTracksBulk: (ids: string[], updates: Partial<Track['metadata']>) => Promise<void>;
  deleteSelectedTracks: () => Promise<void>;
}

export const useLibraryStore = create<LibraryState>((set, get) => ({
  tracks: [],
  selectedTrackIds: new Set(),
  isLoading: false,
  searchQuery: "",
  sortColumn: "title",
  sortDirection: "asc",

  loadTracks: async () => {
    set({ isLoading: true });
    try {
      const query = get().searchQuery;
      // Backend FTS handles basic ordering, but we could add sort params later
      let tracks = await invoke<Track[]>("get_tracks", { query: query || null });
      
      // Client-side sorting
      const { sortColumn, sortDirection } = get();
      tracks.sort((a, b) => {
        let valA: any = a.metadata[sortColumn as keyof typeof a.metadata] || a[sortColumn as keyof typeof a] || '';
        let valB: any = b.metadata[sortColumn as keyof typeof b.metadata] || b[sortColumn as keyof typeof b] || '';
        
        if (typeof valA === 'string') valA = valA.toLowerCase();
        if (typeof valB === 'string') valB = valB.toLowerCase();
        
        if (valA < valB) return sortDirection === 'asc' ? -1 : 1;
        if (valA > valB) return sortDirection === 'asc' ? 1 : -1;
        return 0;
      });

      set({ tracks, isLoading: false });
    } catch (error) {
      console.error("Failed to load tracks:", error);
      set({ isLoading: false });
    }
  },

  setSearchQuery: (query: string) => {
    set({ searchQuery: query });
    get().loadTracks();
  },

  setSort: (column: string) => {
    const { sortColumn, sortDirection } = get();
    if (sortColumn === column) {
      set({ sortDirection: sortDirection === 'asc' ? 'desc' : 'asc' });
    } else {
      set({ sortColumn: column, sortDirection: 'asc' });
    }
    get().loadTracks();
  },

  selectTrack: (id: string, multi: boolean) => {
    set((state) => {
      const newSelection = new Set(multi ? state.selectedTrackIds : []);
      if (newSelection.has(id)) {
        newSelection.delete(id);
      } else {
        newSelection.add(id);
      }
      return { selectedTrackIds: newSelection };
    });
  },

  clearSelection: () => set({ selectedTrackIds: new Set() }),

  updateTrack: async (id: string, updates: Partial<Track['metadata']>) => {
    try {
      await invoke("update_track", { id, update: updates });
      await get().loadTracks();
    } catch (error) {
      console.error("Failed to update track:", error);
    }
  },

  updateTracksBulk: async (ids: string[], updates: Partial<Track['metadata']>) => {
    try {
      await invoke("update_tracks_bulk", { ids, update: updates });
      await get().loadTracks();
    } catch (error) {
      console.error("Failed to update tracks bulk:", error);
    }
  },

  deleteSelectedTracks: async () => {
    const ids = Array.from(get().selectedTrackIds);
    if (ids.length === 0) return;
    
    try {
      await invoke("delete_tracks", { ids });
      set({ selectedTrackIds: new Set() });
      await get().loadTracks();
    } catch (error) {
      console.error("Failed to delete tracks:", error);
    }
  },
}));
