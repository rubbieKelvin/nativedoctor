import { defineStore } from "pinia";
import { ref } from "vue";

const workspaceTabsStore = defineStore("workspaceTabs", () => {
  const openTabIds = ref<string[]>([]);
  const activeTabId = ref<string | null>(null);

  function openTab(id: string) {
    const idx = openTabIds.value.indexOf(id);
    if (idx !== -1) {
      activeTabId.value = id;
      return;
    }
    openTabIds.value = [...openTabIds.value, id];
    activeTabId.value = id;
  }

  function closeTab(id: string) {
    const idx = openTabIds.value.indexOf(id);
    if (idx === -1) return;
    const next = openTabIds.value.filter((tabId) => tabId !== id);
    openTabIds.value = next;
    if (activeTabId.value === id) {
      if (next.length === 0) {
        activeTabId.value = null;
      } else {
        const newIdx = Math.min(idx, next.length - 1);
        activeTabId.value = next[newIdx];
      }
    }
  }

  function setActiveTab(id: string | null) {
    if (id === null) {
      activeTabId.value = null;
      return;
    }
    if (openTabIds.value.includes(id)) {
      activeTabId.value = id;
    }
  }

  return {
    openTabIds,
    activeTabId,
    openTab,
    closeTab,
    setActiveTab,
  };
});

export const useWorkspaceTabs = () => workspaceTabsStore();
