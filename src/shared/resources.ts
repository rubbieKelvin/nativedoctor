import { Resource } from "./types";

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
