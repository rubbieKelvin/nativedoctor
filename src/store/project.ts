import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { defineStore, storeToRefs } from "pinia";
import type {
  HttpResource,
  FolderResource,
  SequenceResource,
  Resource,
} from "@/shared/types/resources";
import { nanoid } from "nanoid";

/** Configuration schema for a nativedoctor project (nativedoctor.json). */
export interface NativedoctorJson {
  name: string;
  description?: string;
  metadata?: Record<string, unknown>;
  envSources?: Array<{ name: string; path: string }>;
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
    url: resource.url ?? "Untitled",
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
 * Creates a new Folder resource with default values.
 * @param name - Optional display name for the folder.
 * @returns A fully initialized FolderResource with a unique ID.
 */
function _createFolderResource(name?: string): FolderResource {
  return {
    id: nanoid(),
    type: "folder",
    name: name ?? "New folder",
    is_edited: true,
    folderId: null,
    children: [],
  };
}

/**
 * Creates a new Sequence resource with default values.
 * @param name - Optional display name for the sequence.
 * @returns A fully initialized SequenceResource with a unique ID.
 */
function _createSequenceResource(name?: string): SequenceResource {
  return {
    id: nanoid(),
    type: "sequence",
    name: name ?? "New sequence",
    is_edited: true,
    folderId: null,
    flow: [],
  };
}

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

/**
 * Discovers resource files in the project root directory.
 * @param projectPath - Absolute path to the project directory.
 * @returns Array of resource file names (*.request.yaml, *.sequence.yaml).
 */
async function discoverResourceFiles(projectPath: string): Promise<string[]> {
  return invoke<string[]>("discover_resources", { projectPath });
}

const projectStore = defineStore("project", () => {
  const path = ref<string | null>(null);
  const config = ref<NativedoctorJson | null>(null);
  const loadError = ref<string | null>(null);
  const resources = ref<Array<Resource>>([]);
  const openResources = ref<Set<string>>(new Set());

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
      resourceFiles.value = await discoverResourceFiles(path.value);
    } catch (e) {
      console.error("Failed to discover resources:", e);
      resourceFiles.value = [];
    }
  }

  /**
   * Sets the active project and loads its configuration.
   * Resets all project state before loading the new project.
   * Automatically discovers resource files after loading.
   * @param fspath - Absolute filesystem path to the project, or null to clear.
   */
  async function setProject(fspath: string | null): Promise<void> {
    path.value = fspath;
    config.value = null;
    loadError.value = null;
    resourceFiles.value = [];

    if (fspath !== null) {
      try {
        const _config = await loadProject(fspath);
        if (_config) {
          config.value = _config;
        }
        await discoverResources();
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
    resources.value = [...resources.value, resource];
    return resource.id;
  }

  /**
   * Creates a new Folder resource and adds it to the project.
   * @param folderId - Optional parent folder ID for nesting.
   * @returns The unique ID of the newly created folder.
   */
  function createFolderResource(folderId?: string) {
    const resource = _createFolderResource();
    if (folderId) resource.folderId = folderId;
    resources.value = [...resources.value, resource];
    return resource.id;
  }

  /**
   * Creates a new Sequence resource and adds it to the project.
   * @param folderId - Optional parent folder ID for organizing the resource.
   * @returns The unique ID of the newly created sequence.
   */
  function createSequenceResource(folderId?: string) {
    const resource = _createSequenceResource();
    if (folderId) resource.folderId = folderId;
    resources.value = [...resources.value, resource];
    return resource.id;
  }

  /**
   * Deletes a resource by ID.
   * @param id - The ID of the resource to delete.
   */
  function deleteResource(id: string) {
    resources.value = resources.value.filter((r) => r.id !== id);
    openResources.value.delete(id);
  }

  /**
   * Duplicates a resource by ID.
   * @param id - The ID of the resource to duplicate.
   * @returns The ID of the new resource, or undefined if not found.
   */
  function duplicateResource(id: string): string | undefined {
    const resource = resources.value.find((r) => r.id === id);
    if (!resource) return undefined;

    let newResource: Resource;

    switch (resource.type) {
      case "http":
        newResource = {
          ...resource,
          id: nanoid(),
          name: `${resource.name} (copy)`,
          is_edited: true,
        };
        break;
      case "folder":
        newResource = {
          ...resource,
          id: nanoid(),
          name: `${resource.name} (copy)`,
          is_edited: true,
          children: [],
        };
        break;
      case "sequence":
        newResource = {
          ...resource,
          id: nanoid(),
          name: `${resource.name} (copy)`,
          is_edited: true,
          flow: [...resource.flow],
        };
        break;
    }

    resources.value = [...resources.value, newResource];
    return newResource.id;
  }

  /**
   * Renames a resource.
   * @param id - The ID of the resource to rename.
   * @param newName - The new name for the resource.
   */
  function renameResource(id: string, newName: string) {
    resources.value = resources.value.map((r) =>
      r.id === id ? { ...r, name: newName, is_edited: true } : r
    );
  }

  /** ID of the resource currently being renamed, or null if none. */
  const renamingResourceId = ref<string | null>(null);

  /**
   * Starts renaming mode for a resource.
   * @param id - The ID of the resource to rename.
   */
  function startRenaming(id: string) {
    renamingResourceId.value = id;
  }

  /**
   * Stops renaming mode.
   */
  function stopRenaming() {
    renamingResourceId.value = null;
  }

  return {
    name,
    path,
    config,
    resourceFiles,
    resources,
    openResources,
    renamingResourceId,
    createHttpResource,
    createFolderResource,
    createSequenceResource,
    deleteResource,
    duplicateResource,
    renameResource,
    startRenaming,
    stopRenaming,
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
 * Use this when you need to call store methods like setProject or createHttpResource.
 */
export const useCurrentProjectActions = () => projectStore();
