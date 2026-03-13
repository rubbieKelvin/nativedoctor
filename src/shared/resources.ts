import { HttpResource, Resource, ResourceFileContentDto } from "./types";

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
      folderId: null,
      created_at,
    };
  }
  if (payload.type === "sequence") {
    const flow = (payload.flow ?? []).map((n) => ({
      id: n.id ?? crypto.randomUUID(),
      resourceId: n.resourceId ?? "",
      resourceType: n.resourceType as "http" | "sequence",
    }));
    return {
      id,
      type: "sequence",
      name: payload.name ?? "Untitled sequence",
      is_edited: false,
      folderId: null,
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
  if (r.type === "http") {
    return {
      type: "http",
      name: r.name,
      method: r.method,
      url: r.url,
      params: r.params,
      headers: r.headers,
      body: r.body,
      auth: r.auth,
    };
  }
  if (r.type === "sequence") {
    return {
      type: "sequence",
      name: r.name,
      flow: r.flow,
    };
  }
  return null;
}
