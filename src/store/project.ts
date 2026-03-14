import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { defineStore, storeToRefs } from "pinia";
import type { NativedoctorJson } from "@/shared/types";
import { useResources } from "./resources";

/**
 * Loads project configuration from the filesystem via Tauri backend.
 */
async function loadProject(path: string): Promise<NativedoctorJson | null> {
  if (!path) return null;

  const data = await invoke<NativedoctorJson>("read_nativedoctor", {
    path,
  });

  return data;
}

const projectStore = defineStore("project", () => {
  const path = ref<string | null>(null);
  const config = ref<NativedoctorJson | null>(null);
  const loadError = ref<string | null>(null);

  /** Resource file names discovered from the project directory. */
  const resourceFiles = ref<string[]>([]);

  /** Project name derived from the loaded configuration. */
  const name = computed(() => config.value?.name);

  /**
   * Discovers resource files in the current project directory.
   * Updates resourceFiles ref with the discovered file names.
   */
  async function discoverResources(): Promise<void> {
    if (!path.value) {
      resourceFiles.value = [];
      return;
    }

    try {
      resourceFiles.value = await invoke<string[]>("discover_resources", {
        projectPath: path.value,
      });
    } catch (e) {
      console.error("Failed to discover resources:", e);
      resourceFiles.value = [];
    }
  }

  /**
   * Sets the active project and loads its configuration.
   * Resets all project state and resources before loading the new project.
   * Automatically discovers resource files after loading.
   * @param fspath - Absolute filesystem path to the project, or null to clear.
   */
  async function setProject(fspath: string | null): Promise<void> {
    path.value = fspath;
    config.value = null;
    loadError.value = null;
    resourceFiles.value = [];

    useResources().reset();

    if (fspath !== null) {
      try {
        const _config = await loadProject(fspath);
        if (_config) {
          config.value = _config;
        }
        await discoverResources();
        await useResources().loadResourcesFromProject(
          fspath,
          resourceFiles.value,
        );
      } catch (e) {
        loadError.value = e instanceof Error ? e.message : String(e);
      }
    }
  }

  return {
    name,
    path,
    config,
    resourceFiles,
    setProject,
    discoverResources,
    loadError,
  };
});

/**
 * Returns reactive refs to project store state.
 * Use this for reading project data in components.
 */
export const useCurrentProject = () => storeToRefs(projectStore());

/**
 * Returns the project store instance with actions.
 * Use this when you need to call store methods like setProject.
 */
export const useCurrentProjectActions = () => projectStore();
