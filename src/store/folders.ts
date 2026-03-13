import { defineStore, storeToRefs } from "pinia";
import { ref } from "vue";

const folderStore = defineStore("folders", () => {
  const openFolders = ref<Set<string>>(new Set());
  function toggle(id: string) {
    if (openFolders.value.has(id)) {
      openFolders.value.delete(id);
    } else {
      openFolders.value.add(id);
    }
  }

  function isOpen(id: string): boolean {
    return openFolders.value.has(id);
  }

  return {
    isOpen,
    toggle,
  };
});

export const useFolderState = () => storeToRefs(folderStore());
