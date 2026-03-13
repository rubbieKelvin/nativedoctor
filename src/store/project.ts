import type { Ref } from "vue";
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
import { useWorkspaceTabs } from "./workspaceTabs";

/**
 * Finds a reource by id in the resource tree (root + all folder children).
 * Uses iterative DFS to avoid recursion depth and to stop as soon as found.
 */
function findResourceInTree<T extends Resource>(
  rootResources: Resource[],
  resourceId: string,
): T | undefined {
  const stack = [...rootResources];
  while (stack.length > 0) {
    const r = stack.pop()!;
    if (r.id === resourceId) return r as T;
    if (r.type === "folder") {
      stack.push(...(r as FolderResource).children);
    }
  }
  return undefined;
}

/**
 * Adds a resource to the project: either as a child of the given folder,
 * or at root level. Updates resource.folderId and triggers reactivity.
 * The folder is resolved recursively (nested folders are supported).
 */
function addResourceToProject(
  resourcesRef: Ref<Array<Resource>>,
  resource: Resource,
  folderId?: string,
): void {
  const folder =
    folderId != null
      ? findResourceInTree<FolderResource>(resourcesRef.value, folderId)
      : undefined;

  resource.folderId = folder?.id ?? null;

  if (folder) {
    folder.children.push(resource);
    resourcesRef.value = [...resourcesRef.value];
  } else {
    resourcesRef.value = [...resourcesRef.value, resource];
  }
}

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
    created_at: Date.now(),
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
    created_at: Date.now(),
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
    created_at: Date.now(),
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

  /** Resource file names discovered from the project directory. */
  const resourceFiles = ref<string[]>([]);

  /** Project name derived from the loaded configuration. */
  const name = computed(() => config.value?.name);

  /**
   * Resolves a resource by id from the full tree (root + all folder children).
   */
  function getResourceById(id: string): Resource | undefined {
    return findResourceInTree(resources.value, id);
  }

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
    const resource = _createHttpResource({});
    addResourceToProject(resources, resource, folderId);
    return resource.id;
  }

  /**
   * Creates a new Folder resource and adds it to the project.
   * @param folderId - Optional parent folder ID for nesting.
   * @returns The unique ID of the newly created folder.
   */
  function createFolderResource(folderId?: string) {
    const resource = _createFolderResource();
    addResourceToProject(resources, resource, folderId);
    return resource.id;
  }

  /**
   * Creates a new Sequence resource and adds it to the project.
   * @param folderId - Optional parent folder ID for organizing the resource.
   * @returns The unique ID of the newly created sequence.
   */
  function createSequenceResource(folderId?: string) {
    const resource = _createSequenceResource();
    addResourceToProject(resources, resource, folderId);
    return resource.id;
  }

  /**
   * Deletes a resource by ID from the tree (root or any folder's children).
   */
  function deleteResource(id: string) {
    function removeFromTree(nodes: Resource[]): boolean {
      for (let i = 0; i < nodes.length; i++) {
        if (nodes[i].id === id) {
          nodes.splice(i, 1);
          return true;
        }
        if (nodes[i].type === "folder") {
          if (removeFromTree((nodes[i] as FolderResource).children)) return true;
        }
      }
      return false;
    }
    const removed = removeFromTree(resources.value);
    if (!removed) return;
    resources.value = [...resources.value];
    useWorkspaceTabs().closeTab(id);
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
      r.id === id ? { ...r, name: newName, is_edited: true } : r,
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
    getResourceById,
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
