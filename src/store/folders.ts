import { defineStore, storeToRefs } from "pinia";
import { ref } from "vue";

const folderStore = defineStore("folders", () => {
  const openFolders = ref<Set<string>>(new Set());

  function toggle(id: string) {
    const next = new Set(openFolders.value);
    if (next.has(id)) {
      next.delete(id);
    } else {
      next.add(id);
    }
    openFolders.value = next;
  }

  function isOpen(id: string): boolean {
    return openFolders.value.has(id);
  }

  return {
    openFolders,
    isOpen,
    toggle,
  };
});

export const useFolders = () => folderStore();
export const useFolderState = () => storeToRefs(folderStore());
