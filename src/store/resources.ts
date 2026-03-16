import type { Ref } from "vue";
import { ref, computed } from "vue";
import { defineStore, storeToRefs } from "pinia";
import { invoke } from "@tauri-apps/api/core";
import type {
  HttpResource,
  FolderResource,
  SequenceResource,
  Resource,
  EditorMetadata,
} from "@/shared/types/resources";
import { nanoid } from "nanoid";
import { useWorkspaceTabsActions } from "./tabs";
import { useResponsesStore } from "./responses";
import type { ResourceFileContentDto } from "@/shared/types";
import {
  buildResourceTree,
  mapBackendToResource,
  mapResourceToBackendPayload,
} from "@/shared/resources";
import {
  NATIVE_DOCTOR_REQUEST_FILE_EXT,
  NATIVE_DOCTOR_SEQUENCE_FILE_EXT,
  NATIVE_DOCTOR_FOLDER_FILE_EXT,
} from "@/shared/constants";
import { matches } from "@/shared/utils";
import { cloneDeep } from "lodash";

export function defaultEditorMeta(
  defaults?: Partial<EditorMetadata>,
): EditorMetadata {
  return {
    changes_made: true,
    ...defaults,
  };
}

/**
 * Finds a resource by id in the resource tree (root + all folder children).
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
 * Walks the resource tree with DFS and fills a map (id -> resource) and optionally a flattened array.
 */
function buildResourceIndex(rootResources: Resource[]): {
  map: Map<string, Resource>;
  flattened: Resource[];
} {
  const map = new Map<string, Resource>();
  const flattened: Resource[] = [];
  const stack = [...rootResources];
  while (stack.length > 0) {
    const r = stack.pop()!;
    map.set(r.id, r);
    flattened.push(r);
    if (r.type === "folder") {
      stack.push(...(r as FolderResource).children);
    }
  }
  return { map, flattened };
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

  resource.folder_id = folder?.id ?? null;

  if (folder) {
    folder.children.push(resource);
    resourcesRef.value = [...resourcesRef.value];
  } else {
    resourcesRef.value = [...resourcesRef.value, resource];
  }
}

/**
 * Creates a new HTTP resource with default values.
 */
function _createHttpResource(
  resource: Partial<Omit<HttpResource, "id" | "type" | "updated">>,
): HttpResource {
  return {
    id: nanoid(),
    type: "http",
    url: resource.url ?? "",
    name: resource.name ?? "Untitled",
    method: resource.method ?? "GET",
    params: resource.params ?? [],
    headers: resource.headers ?? [],
    body: resource.body ?? { type: "none" },
    auth: resource.auth ?? { type: "none" },
    folder_id: resource.folder_id ?? null,
    created_at: Date.now(),
    pre_request_script: resource.pre_request_script ?? "",
    post_request_script: resource.post_request_script ?? "",
    settings: resource.settings ?? {},
    _editor_meta: defaultEditorMeta(),
  };
}

/**
 * Creates a new Folder resource with default values.
 */
function _createFolderResource(name?: string): FolderResource {
  return {
    id: nanoid(),
    type: "folder",
    name: name ?? "New folder",
    folder_id: null,
    children: [],
    created_at: Date.now(),
    _editor_meta: defaultEditorMeta(),
  };
}

/**
 * Creates a new Sequence resource with default values.
 */
function _createSequenceResource(name?: string): SequenceResource {
  return {
    id: nanoid(),
    type: "sequence",
    name: name ?? "New sequence",
    folder_id: null,
    flow: [],
    created_at: Date.now(),
    _editor_meta: defaultEditorMeta(),
  };
}

const resourcesStore = defineStore("resources", () => {
  const resources = ref<Array<Resource>>([]);

  /** Map resource id -> file name (for persistence). Updated on load and when saving new resources. */
  const resourceFileNames = ref<Map<string, string>>(new Map());

  /** O(1) id -> resource map and flattened list, derived from the tree. */
  const resourceIndex = computed(() => buildResourceIndex(resources.value));
  const resourceMap = computed(() => resourceIndex.value.map);
  const flattenedResources = computed(() => resourceIndex.value.flattened);

  /**
   * Resolves a resource by id from the full tree (O(1) via map).
   */
  function getResourceById(id: string): Resource | undefined {
    return resourceMap.value.get(id);
  }

  /**
   * Returns HTTP resource by id, or undefined if not found or not HTTP type.
   */
  function getHttpResource(id: string): HttpResource | undefined {
    const r = resourceMap.value.get(id);
    return r?.type === "http" ? (r as HttpResource) : undefined;
  }

  /**
   * Returns Sequence resource by id, or undefined if not found or not Sequence type.
   */
  function getSequenceResource(id: string): SequenceResource | undefined {
    const r = resourceMap.value.get(id);
    return r?.type === "sequence" ? (r as SequenceResource) : undefined;
  }

  /**
   * Replaces the entire resource tree (e.g. after loading from project).
   */
  function setResources(tree: Resource[]) {
    resources.value = tree;
  }

  /**
   * Files that failed to load in the last loadResourcesFromProject run.
   * Cleared on reset or on next load.
   */
  const loadFailures = ref<Array<{ fileName: string; error?: string }>>([]);

  /**
   * Clears all resources (e.g. when project is closed).
   */
  function reset() {
    resources.value = [];
    resourceFileNames.value = new Map();
    renamingResourceId.value = null;
    loadFailures.value = [];
    useResponsesStore().clearAll();
  }

  /**
   * Reads a single resource file and returns the mapped resource and its id, or null on failure.
   */
  async function loadOneResourceFile(
    projectPath: string,
    fileName: string,
  ): Promise<{ resource: Resource; id: string } | null> {
    const payload = await invoke<ResourceFileContentDto>("read_resource_file", {
      projectPath,
      fileName,
    });
    const resource = mapBackendToResource(payload);
    if (!resource) return null;
    return { resource, id: payload.id };
  }

  /**
   * Loads resources from project directory by reading each discovered file and parsing YAML.
   * Rebuilds the tree from folder_id so the sidebar hierarchy is restored.
   * Populates loadFailures with any files that could not be loaded.
   */
  async function loadResourcesFromProject(
    projectPath: string,
    fileNames: string[],
  ): Promise<void> {
    reset();
    const flatList: Resource[] = [];
    const newFileNames = new Map<string, string>();
    const failures: Array<{ fileName: string; error?: string }> = [];

    for (const fileName of fileNames) {
      try {
        const result = await loadOneResourceFile(projectPath, fileName);
        if (result) {
          flatList.push(result.resource);
          newFileNames.set(result.id, fileName);
        } else {
          failures.push({ fileName });
        }
      } catch (e) {
        console.error(`Failed to load resource file ${fileName}:`, e);
        failures.push({
          fileName,
          error: e instanceof Error ? e.message : String(e),
        });
      }
    }

    resources.value = buildResourceTree(flatList);
    resourceFileNames.value = newFileNames;
    loadFailures.value = failures;
  }

  /**
   * Saves all resources marked as edited to disk (explicit save).
   * Folders are persisted so the hierarchy can be restored on load.
   */
  async function saveResources(projectPath: string): Promise<void> {
    const fileNames = resourceFileNames.value;
    const edited = flattenedResources.value.filter(
      (r) => r._editor_meta.changes_made,
    );

    for (const resource of edited) {
      let fileName = fileNames.get(resource.id);

      if (!fileName) {
        const _name = nanoid();
        fileName = matches(resource.type, {
          http: () => `${_name}${NATIVE_DOCTOR_REQUEST_FILE_EXT}`,
          sequence: () => `${_name}${NATIVE_DOCTOR_SEQUENCE_FILE_EXT}`,
          folder: () => `${_name}${NATIVE_DOCTOR_FOLDER_FILE_EXT}`,
          _: (n) => {
            throw new Error(`Unknown type: ${n}`);
          },
        });

        // add this to our filenames map
        resourceFileNames.value = new Map(resourceFileNames.value).set(
          resource.id,
          fileName,
        );
      }

      try {
        const payload = mapResourceToBackendPayload(resource);
        if (!payload) continue;
        await invoke("write_resource_file", {
          projectPath,
          fileName,
          payload,
        });
        resource._editor_meta.changes_made = false;
        resource.updated_at = Date.now();
      } catch (e) {
        console.error(`Failed to write resource file ${fileName}:`, e);
      }
    }

    resources.value = [...resources.value];
  }

  function createHttpResource(folderId?: string) {
    const resource = _createHttpResource({});
    addResourceToProject(resources, resource, folderId);
    return resource.id;
  }

  function createFolderResource(folderId?: string) {
    const resource = _createFolderResource();
    addResourceToProject(resources, resource, folderId);
    return resource.id;
  }

  function createSequenceResource(folderId?: string) {
    const resource = _createSequenceResource();
    addResourceToProject(resources, resource, folderId);
    return resource.id;
  }

  function deleteResource(id: string) {
    // TODO: delete the file from fs
    const removeFromTree = (nodes: Resource[]): boolean => {
      for (let i = 0; i < nodes.length; i++) {
        if (nodes[i].id === id) {
          nodes.splice(i, 1);
          return true;
        }
        if (nodes[i].type === "folder") {
          if (removeFromTree((nodes[i] as FolderResource).children))
            return true;
        }
      }
      return false;
    };

    const removed = removeFromTree(resources.value);
    if (!removed) return;
    const next = new Map(resourceFileNames.value);
    next.delete(id);

    resourceFileNames.value = next;
    resources.value = [...resources.value];
    useWorkspaceTabsActions().closeTab(id);
  }

  function duplicateResource(id: string): string | undefined {
    const resource = findResourceInTree(resources.value, id);
    if (!resource) return undefined;

    let newResource: Resource;
    let name = resource.name ?? "untitled";

    switch (resource.type) {
      case "http":
        newResource = {
          ...cloneDeep(resource),
          id: nanoid(),
          name: `${name} (copy)`,
          _editor_meta: defaultEditorMeta(),
        };
        break;
      case "folder":
        throw new Error("Cannot duplicate folder");
      case "sequence":
        newResource = {
          ...cloneDeep(resource),
          id: nanoid(),
          name: `${name} (copy)`,
          _editor_meta: defaultEditorMeta(),
        };
        break;
    }

    addResourceToProject(
      resources,
      newResource,
      resource.folder_id ?? undefined,
    );

    return newResource.id;
  }

  function renameResource(id: string, newName: string) {
    const resource = findResourceInTree(resources.value, id);
    if (resource) {
      resource.name = newName;
      resource._editor_meta.changes_made = true;
      resources.value = [...resources.value];
    }
  }

  /**
   * Updates an HTTP resource by id.
   * Only in-place fields are updated; tree shape unchanged.
   */
  function updateHttpResource(
    id: string,
    patch: Partial<Omit<HttpResource, "id" | "type" | "created_at">>,
  ) {
    const resource = resourceMap.value.get(id) as HttpResource | undefined;
    if (!resource || resource.type !== "http") return;

    // resource update
    Object.assign(resource, patch, {
      updated_at: Date.now(),
    });

    // meta update
    Object.assign(resource._editor_meta, { changes_made: true });

    resources.value = [...resources.value];
  }

  /**
   * Updates a Sequence resource by id. Only in-place fields are updated; tree shape unchanged.
   */
  function updateSequenceResource(
    id: string,
    patch: Partial<Omit<SequenceResource, "id" | "type" | "created_at">>,
  ) {
    const resource = resourceMap.value.get(id) as SequenceResource | undefined;
    if (!resource || resource.type !== "sequence") return;

    // resource update
    Object.assign(resource, patch, {
      updated_at: Date.now(),
    });

    // meta update
    Object.assign(resource._editor_meta, { changes_made: true });

    resources.value = [...resources.value];
  }

  const renamingResourceId = ref<string | null>(null);

  function startRenaming(id: string) {
    renamingResourceId.value = id;
  }

  function stopRenaming() {
    renamingResourceId.value = null;
  }

  return {
    resources,
    flattenedResources,
    loadFailures,
    getResourceById,
    getHttpResource,
    getSequenceResource,
    setResources,
    reset,
    loadResourcesFromProject,
    saveResources,
    createHttpResource,
    createFolderResource,
    createSequenceResource,
    deleteResource,
    duplicateResource,
    renameResource,
    updateHttpResource,
    updateSequenceResource,
    startRenaming,
    stopRenaming,
    renamingResourceId,
  };
});

export const useResources = () => resourcesStore();
export const useResourcesState = () => storeToRefs(resourcesStore());
