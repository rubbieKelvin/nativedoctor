import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { defineStore, storeToRefs } from "pinia";

export interface NativedoctorJson {
  name: string;
  files?: string[];
}

const projectStore = defineStore("project", () => {
  const projectPath = ref<string | null>(null);
  const config = ref<NativedoctorJson | null>(null);
  const loadError = ref<string | null>(null);

  const files = computed(() => config.value?.files ?? []);

  function setProject(path: string | null): void {
    projectPath.value = path;
    config.value = null;
    loadError.value = null;
    if (path !== null) {
      loadProject();
    }
  }

  async function loadProject(): Promise<void> {
    const path = projectPath.value;
    if (!path) return;
    loadError.value = null;
    try {
      const data = await invoke<NativedoctorJson>("read_nativedoctor", {
        path,
      });
      config.value = data;
    } catch (e) {
      loadError.value = e instanceof Error ? e.message : String(e);
      config.value = null;
    }
  }

  return {
    projectPath,
    config,
    files,
    loadError,
    setProject,
    loadProject,
  };
});

export const useCurrentProject = () => storeToRefs(projectStore());
export const useCurrentProjectActions = () => projectStore();
