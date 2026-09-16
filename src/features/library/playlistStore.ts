import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import type { Playlist, SmartRules } from "../../types/playlist";

interface PlaylistState {
  playlists: Playlist[];
  isLoading: boolean;
  selectedPlaylistId: string | null;

  // Actions
  loadPlaylists: () => Promise<void>;
  selectPlaylist: (id: string | null) => void;
  createPlaylist: (name: string, parentId?: string, isFolder?: boolean) => Promise<string | undefined>;
  createSmartPlaylist: (name: string, rules: SmartRules, parentId?: string) => Promise<string | undefined>;
  renamePlaylist: (id: string, name: string) => Promise<void>;
  deletePlaylist: (id: string) => Promise<void>;
  addTrackToPlaylist: (playlistId: string, trackId: string) => Promise<void>;
  removeTrackFromPlaylist: (playlistId: string, trackId: string) => Promise<void>;
}

export const usePlaylistStore = create<PlaylistState>((set, get) => ({
  playlists: [],
  isLoading: false,
  selectedPlaylistId: null,

  loadPlaylists: async () => {
    set({ isLoading: true });
    try {
      const playlists = await invoke<Playlist[]>("get_playlists");
      
      // Build tree structure
      const playlistMap = new Map<string, Playlist>();
      const rootPlaylists: Playlist[] = [];

      playlists.forEach((p) => {
        p.children = [];
        playlistMap.set(p.id, p);
      });

      playlists.forEach((p) => {
        if (p.parent_id && playlistMap.has(p.parent_id)) {
          playlistMap.get(p.parent_id)!.children!.push(p);
        } else {
          rootPlaylists.push(p);
        }
      });

      set({ playlists: rootPlaylists, isLoading: false });
    } catch (error) {
      console.error("Failed to load playlists:", error);
      set({ isLoading: false });
    }
  },

  selectPlaylist: (id: string | null) => {
    set({ selectedPlaylistId: id });
  },

  createPlaylist: async (name: string, parentId?: string, isFolder = false) => {
    try {
      const id = await invoke<string>("create_playlist", { 
        request: { name, parent_id: parentId, is_folder: isFolder } 
      });
      await get().loadPlaylists();
      return id;
    } catch (error) {
      console.error("Failed to create playlist:", error);
    }
  },

  createSmartPlaylist: async (name: string, rules: SmartRules, parentId?: string) => {
    try {
      const rulesJson = JSON.stringify(rules);
      const id = await invoke<string>("create_smart_playlist", {
        request: { name, parent_id: parentId, rules_json: rulesJson }
      });
      await get().loadPlaylists();
      return id;
    } catch (error) {
      console.error("Failed to create smart playlist:", error);
    }
  },

  renamePlaylist: async (id: string, name: string) => {
    try {
      await invoke("rename_playlist", { id, name });
      await get().loadPlaylists();
    } catch (error) {
      console.error("Failed to rename playlist:", error);
    }
  },

  deletePlaylist: async (id: string) => {
    try {
      await invoke("delete_playlist", { id });
      if (get().selectedPlaylistId === id) {
        set({ selectedPlaylistId: null });
      }
      await get().loadPlaylists();
    } catch (error) {
      console.error("Failed to delete playlist:", error);
    }
  },

  addTrackToPlaylist: async (playlistId: string, trackId: string) => {
    try {
      await invoke("add_track_to_playlist", { playlistId, trackId });
      await get().loadPlaylists(); // update track_count
    } catch (error) {
      console.error("Failed to add track to playlist:", error);
    }
  },

  removeTrackFromPlaylist: async (playlistId: string, trackId: string) => {
    try {
      await invoke("remove_track_from_playlist", { playlistId, trackId });
      await get().loadPlaylists(); // update track_count
    } catch (error) {
      console.error("Failed to remove track from playlist:", error);
    }
  },
}));
