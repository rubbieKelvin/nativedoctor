import { omit } from "lodash";
import {
  NATIVE_DOCTOR_REQUEST_FILE_PUBLIC_SCHEMA_URL,
  NATIVE_DOCTOR_SEQUENCE_FILE_PUBLIC_SCHEMA_URL,
} from "./constants";
import {
  HttpResource,
  HttpResourceFileDto,
  Resource,
  ResourceFileContentDto,
  SequenceResource,
  SequenceResourceFileDto,
} from "./types";
import { matches } from "./utils";

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
 * Maps backend JSON (from read_resource_file) to frontend Resource.
 * Adds id, is_edited, folderId, created_at.
 */
export function mapBackendToResource(
  payload: ResourceFileContentDto,
  id: string,
  created_at: number,
): Resource | null {
  if (payload.type === "http") {
    return {
      id,
      type: "http",
      name: payload.name ?? "Untitled",
      method: (payload.method as HttpResource["method"]) ?? "GET",
      url: payload.url ?? "",
      params: payload.params ?? [],
      headers: payload.headers ?? [],
      body: payload.body ?? { type: "none" },
      auth: payload.auth ?? { type: "none" },
      is_edited: false,
      folder_id: null,
      created_at,
    };
  }
  if (payload.type === "sequence") {
    const flow = (payload.flow ?? []).map((n) => ({
      id: n.id ?? crypto.randomUUID(),
      resource_id: n.resource_id ?? "",
      resource_type: n.resource_type as "http" | "sequence",
    }));

    return {
      id,
      type: "sequence",
      name: payload.name ?? "Untitled sequence",
      is_edited: false,
      folder_id: null,
      flow,
      created_at,
    };
  }
  return null;
}

/**
 * Maps frontend Resource to backend payload for write_resource_file.
 * Strips id, is_edited, folderId, created_at, updated_at.
 */
export function mapResourceToBackendPayload(
  r: Resource,
): ResourceFileContentDto | null {
  return matches<string, ResourceFileContentDto | null>(r.type, {
    http: (): HttpResourceFileDto => {
      const http = r as HttpResource;
      return {
        $schema: NATIVE_DOCTOR_REQUEST_FILE_PUBLIC_SCHEMA_URL,
        ...omit(http, ["is_edited", "folder_id"]),
      };
    },
    sequence: (): SequenceResourceFileDto => {
      const sequence = r as SequenceResource;
      return {
        $schema: NATIVE_DOCTOR_SEQUENCE_FILE_PUBLIC_SCHEMA_URL,
        ...omit(sequence, ["is_edited", "folder_id"]),
      };
    },
    _: () => {
      console.error(new Error("Cannot map this resource to backend"));
      return null;
    },
  });
}
