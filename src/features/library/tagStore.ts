import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import type { Tag } from "../../types/tag";

interface TagState {
  tags: Tag[];
  isLoading: boolean;

  // Actions
  loadTags: () => Promise<void>;
  createTag: (name: string, color?: string) => Promise<string | undefined>;
  renameTag: (id: string, name: string) => Promise<void>;
  updateTagColor: (id: string, color?: string) => Promise<void>;
  deleteTag: (id: string) => Promise<void>;
  assignTag: (trackId: string, tagId: string) => Promise<void>;
  removeTagFromTrack: (trackId: string, tagId: string) => Promise<void>;
  batchAssignTag: (trackIds: string[], tagId: string) => Promise<void>;
}

export const useTagStore = create<TagState>((set, get) => ({
  tags: [],
  isLoading: false,

  loadTags: async () => {
    set({ isLoading: true });
    try {
      const tags = await invoke<Tag[]>("get_tags");
      set({ tags, isLoading: false });
    } catch (error) {
      console.error("Failed to load tags:", error);
      set({ isLoading: false });
    }
  },

  createTag: async (name: string, color?: string) => {
    try {
      const id = await invoke<string>("create_tag", { name, color: color || null });
      await get().loadTags();
      return id;
    } catch (error) {
      console.error("Failed to create tag:", error);
    }
  },

  renameTag: async (id: string, name: string) => {
    try {
      await invoke("rename_tag", { id, name });
      await get().loadTags();
    } catch (error) {
      console.error("Failed to rename tag:", error);
    }
  },

  updateTagColor: async (id: string, color?: string) => {
    try {
      await invoke("update_tag_color", { id, color: color || null });
      await get().loadTags();
    } catch (error) {
      console.error("Failed to update tag color:", error);
    }
  },

  deleteTag: async (id: string) => {
    try {
      await invoke("delete_tag", { id });
      await get().loadTags();
    } catch (error) {
      console.error("Failed to delete tag:", error);
    }
  },

  assignTag: async (trackId: string, tagId: string) => {
    try {
      await invoke("assign_tag", { trackId, tagId });
      await get().loadTags();
    } catch (error) {
      console.error("Failed to assign tag:", error);
    }
  },

  removeTagFromTrack: async (trackId: string, tagId: string) => {
    try {
      await invoke("remove_tag_from_track", { trackId, tagId });
      await get().loadTags();
    } catch (error) {
      console.error("Failed to remove tag from track:", error);
    }
  },

  batchAssignTag: async (trackIds: string[], tagId: string) => {
    try {
      await invoke("batch_assign_tag", { trackIds, tagId });
      await get().loadTags();
    } catch (error) {
      console.error("Failed to batch assign tag:", error);
    }
  },
}));
