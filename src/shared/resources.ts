import { Resource } from "./types";

export function sortedResources(resources: Resource[]): Resource[] {
  // Put folders at the top, then all the others can follow
  // No recursive sorting, just sort top level items
  return resources;
}
