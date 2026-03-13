import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { defineStore, storeToRefs } from "pinia";
import type { HttpResource, Resource } from "@/shared/types/resources";
import { nanoid } from "nanoid";

/** Configuration schema for a nativedoctor project (nativedoctor.json). */
export interface NativedoctorJson {
  name: string;
  files?: string[];
}

/**
 * Creates a new HTTP resource with default values.
 * @param resource - Partial resource data to merge with defaults.
 * @returns A fully initialized HttpResource with a unique ID.
 */
function _createHttpResource(
  resource: Partial<Omit<HttpResource, "id" | "type" | "updated">>,
): HttpResource {
  return {
    id: nanoid(),
    type: "http",
    url: resource.url ?? "",
    name: resource.name ?? "",
    method: resource.method ?? "GET",
    params: resource.params ?? [],
    headers: resource.headers ?? [],
    body: resource.body ?? { type: "none" },
    auth: resource.auth ?? { type: "none" },
    is_edited: true,
    folderId: resource.folderId ?? null,
  };
}

/**
 * Loads project configuration from the filesystem via Tauri backend.
 * @param path - Absolute path to the project directory.
 * @returns The parsed project configuration, or null if path is empty.
 * @throws Error if the backend fails to read the configuration.
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
  const resources = ref<Array<Resource>>([]);
  const openResources = ref<Set<string>>(new Set());

  /** Project name derived from the loaded configuration. */
  const name = computed(() => config.value?.name);

  /** List of resource file paths from the project configuration. */
  const files = computed(() => config.value?.files ?? []);

  /**
   * Sets the active project and loads its configuration.
   * Resets all project state before loading the new project.
   */
  async function setProject(fspath: string | null): Promise<void> {
    path.value = fspath;
    config.value = null;
    loadError.value = null;

    if (fspath !== null) {
      try {
        const _config = await loadProject(fspath);
        if (_config) {
          config.value = _config;
        }
      } catch (e) {
        loadError.value = e instanceof Error ? e.message : String(e);
      }
    }
  }

  /**
   * Creates a new HTTP resource and adds it to the project.
   * @param folderId - Optional parent folder ID for organizing the resource.
   * @returns The unique ID of the newly created resource.
   */
  function createHttpResource(folderId?: string) {
    const resource = _createHttpResource({ folderId });
    resources.value.push(resource);
    return resource.id;
  }

  return {
    name,
    path,
    config,
    files,
    resources,
    openResources,
    createHttpResource,
    setProject,
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
 * Use this when you need to call store methods like setProject or createHttpResource.
 */
export const useCurrentProjectActions = () => projectStore();
