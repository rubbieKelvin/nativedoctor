import { defineStore } from "pinia";
import { ref } from "vue";
import { fetchWorkspace, type WorkspaceSnapshot } from "@/api";

export const useWorkspaceStore = defineStore("workspace", () => {
    const workspace = ref<WorkspaceSnapshot | null>(null);
    const loadErr = ref<string | null>(null);

    async function loadWorkspace() {
        loadErr.value = null;
        try {
            workspace.value = await fetchWorkspace();
        } catch (e) {
            loadErr.value = e instanceof Error ? e.message : String(e);
        }
    }

    return { workspace, loadErr, loadWorkspace };
});
