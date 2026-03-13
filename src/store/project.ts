import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { defineStore, storeToRefs } from "pinia";
import type { HttpResource, Resource } from "@/shared/types/resources";
import { nanoid } from "nanoid";

export interface NativedoctorJson {
  name: string;
  files?: string[];
}

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

async function loadProject(path: string): Promise<NativedoctorJson | null> {
  if (!path) return null;

  const data = await invoke<NativedoctorJson>("read_nativedoctor", {
    path,
  });

  return data;
  try {
  } catch (e) {
    // loadError.value = e instanceof Error ? e.message : String(e);
    config.value = null;
  }
}

const projectStore = defineStore("project", () => {
  const path = ref<string | null>(null);
  const config = ref<NativedoctorJson | null>(null);
  const loadError = ref<string | null>(null);
  const resources = ref<Array<Resource>>([]);
  const openResources = ref<Set<string>>(new Set());

  const name = computed(() => config.value?.name);
  const files = computed(() => config.value?.files ?? []);

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
    // loadProject,
  };
});

export const useCurrentProject = () => storeToRefs(projectStore());
export const useCurrentProjectActions = () => projectStore();
