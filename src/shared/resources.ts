import { omit } from "lodash";
import {
  NATIVE_DOCTOR_REQUEST_FILE_PUBLIC_SCHEMA_URL,
  NATIVE_DOCTOR_SEQUENCE_FILE_PUBLIC_SCHEMA_URL,
} from "./constants";
import {
  FolderResource,
  FolderResourceFileDto,
  HttpResource,
  HttpResourceFileDto,
  Resource,
  ResourceFileContentDto,
  ResourceType,
  SequenceResource,
  SequenceResourceFileDto,
} from "./types";
import { matches } from "./utils";
import { defaultEditorMeta } from "@/store/resources";

type SortingOptions = {
  sorting: "NAME" | "DATE_CREATED" | "DATE_UPDATED";
  // when we group by type,
  // folder should be sorted first
  // then requests, then sequences
  grouping: "NONE" | "TYPE";
};

/**
 * Sorts and groups resources based on the provided options.
 * Folders are always prioritized at the top, followed by other resources.
 * If grouping by type is enabled, resources are grouped as folders -> requests -> sequences.
 * Within groups, items are sorted by the specified criteria (NAME, DATE_CREATED, or DATE_UPDATED).
 *
 * @param resources - The list of resources to sort.
 * @param options - Optional sorting and grouping settings.
 * @returns A new array of sorted resources.
 */
export function sortedResources(
  resources: Resource[],
  options?: SortingOptions,
): Resource[] {
  const sorting = options?.sorting ?? "NAME";
  const grouping = options?.grouping ?? "TYPE";

  return [...resources].sort((a, b) => {
    // 1. Grouping Logic (always prioritize folders at the top)
    if (grouping === "TYPE") {
      const typeOrder: Record<string, number> = {
        folder: 0,
        http: 1,
        sequence: 2,
      };

      if (typeOrder[a.type] !== typeOrder[b.type]) {
        return typeOrder[a.type] - typeOrder[b.type];
      }
    } else {
      // Even if grouping is "NONE", folders should always be at the top
      if (a.type === "folder" && b.type !== "folder") return -1;
      if (a.type !== "folder" && b.type === "folder") return 1;
    }

    // 2. Sorting Logic (within groups or overall)
    switch (sorting) {
      case "NAME":
        return a.name.localeCompare(b.name, undefined, {
          numeric: true,
          sensitivity: "base",
        });

      case "DATE_CREATED":
        return b.created_at - a.created_at; // Newest first

      case "DATE_UPDATED": {
        const dateA = a.updated_at ?? a.created_at;
        const dateB = b.updated_at ?? b.created_at;
        return dateB - dateA; // Newest first
      }

      default:
        return 0;
    }
  });
}

/**
 * Reconstructs the sidebar tree from a flat list of resources loaded from disk.
 * Folder files do not store children; parent-child links are rebuilt from folder_id.
 * Orphan resources (folder_id pointing to missing or non-folder) are placed at root so they are not lost.
 */
export function buildResourceTree(flatList: Resource[]): Resource[] {
  const map = new Map<string, Resource>();

  // index the flatlist
  for (const r of flatList) {
    map.set(r.id, r);
  }

  for (const resource of flatList) {
    if (resource.folder_id == null) continue;

    const parent = map.get(resource.folder_id);
    if (parent?.type === "folder") {
      (parent as FolderResource).children.push(resource);
    }
  }

  // return all the top level items.
  return flatList.filter((r) => {
    const parent = r.folder_id != null ? map.get(r.folder_id) : undefined;
    return r.folder_id === null || !parent || parent.type !== "folder";
  });
}

/**
 * Maps backend JSON (from read_resource_file) to frontend Resource.
 * Folder files do not store children; the tree is rebuilt from folder_id on load.
 */
export function mapBackendToResource(
  content: ResourceFileContentDto,
): Resource | null {
  return matches<ResourceType, Resource | null>(content.type, {
    http: (): HttpResource => {
      const http = content as HttpResourceFileDto;
      return {
        ...http,
        _editor_meta: defaultEditorMeta({ changes_made: false }),
      };
    },
    sequence: (): SequenceResource => {
      const sequence = content as SequenceResourceFileDto;
      return {
        ...sequence,
        _editor_meta: defaultEditorMeta({ changes_made: false }),
      };
    },
    folder: (): FolderResource => {
      const folder = content as FolderResourceFileDto;
      return {
        ...folder,
        children: [], // We'll attempt to fill this folder witht the correct children later
        _editor_meta: defaultEditorMeta({ changes_made: false }),
      };
    },
    _: () => null,
  });
}

/**
 * Maps frontend Resource to backend payload for write_resource_file.
 * Folder: children are not persisted; tree is rebuilt from folder_id on load.
 */
export function mapResourceToBackendPayload(
  resource: Resource,
): ResourceFileContentDto | null {
  return matches<string, ResourceFileContentDto | null>(resource.type, {
    http: (): HttpResourceFileDto => {
      const http = resource as HttpResource;
      return {
        $schema: NATIVE_DOCTOR_REQUEST_FILE_PUBLIC_SCHEMA_URL,
        ...omit(http, ["_editor_meta"]),
      };
    },
    sequence: (): SequenceResourceFileDto => {
      const sequence = resource as SequenceResource;
      return {
        $schema: NATIVE_DOCTOR_SEQUENCE_FILE_PUBLIC_SCHEMA_URL,
        ...omit(sequence, ["_editor_meta"]),
      };
    },
    folder: (): FolderResourceFileDto => {
      const folder = resource as FolderResource;
      return {
        ...omit(folder, ["_editor_meta", "children"]),
      };
    },
    _: () => {
      console.error(new Error("Cannot map this resource to backend"));
      return null;
    },
  });
}
