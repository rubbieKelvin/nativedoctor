import { useCurrentProjectActions } from "@/store/project";
import type { HttpResource } from "@/shared/types/resources";

export function useHttpActions(resourceId: string) {
  const store = useCurrentProjectActions();

  function renameHttp() {
    store.startRenaming(resourceId);
  }

  function deleteHttp() {
    store.deleteResource(resourceId);
  }

  function duplicateHttp() {
    store.duplicateResource(resourceId);
  }

  function copyAsCurl() {
    const resource = store.resources.find(
      (r) => r.id === resourceId && r.type === "http"
    ) as HttpResource | undefined;

    if (!resource) return;

    let curl = `curl -X ${resource.method}`;

    for (const header of resource.headers) {
      if (header.key && header.enabled !== false) {
        curl += ` -H '${header.key}: ${header.value}'`;
      }
    }

    if (
      resource.body.type === "json" ||
      resource.body.type === "text" ||
      resource.body.type === "graphql"
    ) {
      curl += ` -d '${resource.body.content}'`;
    }

    let url = resource.url;
    const enabledParams = resource.params.filter(
      (p) => p.key && p.enabled !== false
    );
    if (enabledParams.length > 0) {
      const queryString = enabledParams
        .map((p) => `${encodeURIComponent(p.key)}=${encodeURIComponent(p.value)}`)
        .join("&");
      url += (url.includes("?") ? "&" : "?") + queryString;
    }

    curl += ` '${url}'`;

    navigator.clipboard.writeText(curl);
  }

  return {
    renameHttp,
    deleteHttp,
    duplicateHttp,
    copyAsCurl,
  };
}
