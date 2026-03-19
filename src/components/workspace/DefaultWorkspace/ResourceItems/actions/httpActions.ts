import type { HttpResource } from "@/shared/types/resources";
import { useResources } from "@/store/resources";
import { serializeHttpBodyToRequestString } from "@/components/resourcepads/http/body/body-helpers";

export function useHttpActions(resourceId: string) {
  const store = useResources();

  function renameHttp() {
    // Defer so the context menu closes and restores focus first; then the input can receive focus without an immediate blur.
    setTimeout(() => store.startRenaming(resourceId), 0);
  }

  function deleteHttp() {
    store.deleteResource(resourceId);
  }

  function duplicateHttp() {
    store.duplicateResource(resourceId);
  }

  function copyAsCurl() {
    const resource = store.getResourceById(resourceId);
    if (!resource || resource.type !== "http") return;
    const http = resource as HttpResource;

    let curl = `curl -X ${http.method}`;

    for (const header of http.headers) {
      if (header.key && header.enabled !== false) {
        curl += ` -H '${header.key}: ${header.value}'`;
      }
    }

    const bodyStr = serializeHttpBodyToRequestString(http.body);
    if (bodyStr != null) {
      curl += ` -d '${bodyStr.replace(/'/g, "'\\''")}'`;
    }

    let url = http.url;
    const enabledParams = http.params.filter(
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
