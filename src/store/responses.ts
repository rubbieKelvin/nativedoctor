import { ref, computed } from "vue";
import { defineStore, storeToRefs } from "pinia";

export interface StoredHttpResponse {
    status: number;
    headers: [string, string][];
    body: string;
    duration_ms: number;
    url?: string;
    size?: number;
    http_version?: string;
}

const responsesStore = defineStore("responses", () => {
    const byResourceId = ref<Record<string, StoredHttpResponse>>({});

    function setResponse(resourceId: string, data: StoredHttpResponse) {
        byResourceId.value = {
            ...byResourceId.value,
            [resourceId]: data,
        };
    }

    function getResponse(resourceId: string): StoredHttpResponse | undefined {
        return byResourceId.value[resourceId];
    }

    function clearResponse(resourceId: string) {
        const next = { ...byResourceId.value };
        delete next[resourceId];
        byResourceId.value = next;
    }

    function clearAll() {
        byResourceId.value = {};
    }

    return {
        byResourceId: computed(() => byResourceId.value),
        setResponse,
        getResponse,
        clearResponse,
        clearAll,
    };
});

export function useResponsesStore() {
    return responsesStore();
}

export function useResponsesRefs() {
    return storeToRefs(responsesStore());
}
