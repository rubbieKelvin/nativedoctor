<script setup lang="ts">
import type { KeyValue } from "./types";
import type { HttpBody, HttpResource } from "@/shared/types/resources";
import { invoke } from "@tauri-apps/api/core";
import UrlMethodBar from "./UrlMethodBar.vue";
import RequestTabs from "./RequestTabs.vue";
import ResponsePane from "./ResponsePane.vue";
import { ref, computed, watch } from "vue";
import type { HttpMethodType } from "@/shared/constants/http";
import {
    ResizableHandle,
    ResizablePanel,
    ResizablePanelGroup,
} from "@/components/ui/resizable";
import { useResources } from "@/store/resources";
import { useResponsesStore } from "@/store/responses";
import { computedHeaders } from "@/shared/resources";
import { serializeHttpBodyToRequestString } from "./body/body-helpers";
import type { KeyValuePair } from "@/shared/types/resources";

const props = withDefaults(defineProps<{ resource?: HttpResource | null }>(), {
    resource: undefined,
});

const resourcesStore = useResources();
const responsesStore = useResponsesStore();

const url = ref("");
const resposeTab = ref<"headers" | "body" | "cookies">("headers");
const method = ref<HttpMethodType>("GET");
const params = ref<KeyValue[]>([{ key: "", value: "", enabled: true }]);
const headers = ref<KeyValue[]>([{ key: "", value: "", enabled: true }]);
const body = ref<HttpBody>({ type: "none" });

const status = ref<number | undefined>(undefined);
const responseHeaders = ref<[string, string][]>([]);
const responseBody = ref("");
const durationMs = ref<number | undefined>(undefined);
const httpVersion = ref<string | undefined>(undefined);
const error = ref<string | undefined>(undefined);
const loading = ref(false);
const responseVersion = ref(0);

function applyStoredResponse(resourceId: string) {
    const stored = responsesStore.getResponse(resourceId);
    if (stored) {
        status.value = stored.status;
        responseHeaders.value = stored.headers;
        responseBody.value = stored.body;
        durationMs.value = stored.duration_ms;
        httpVersion.value = stored.http_version;
    } else {
        status.value = undefined;
        responseHeaders.value = [];
        responseBody.value = "";
        durationMs.value = undefined;
        httpVersion.value = undefined;
    }
    responseVersion.value += 1;
}

watch(
    () => props.resource,
    (r) => {
        if (!r) return;
        url.value = r.url ?? "";
        method.value = (r.method as HttpMethodType) ?? "GET";
        params.value = r.params?.length
            ? [...r.params]
            : [{ key: "", value: "", enabled: true }];
        headers.value = r.headers?.length
            ? [...r.headers]
            : [{ key: "", value: "", enabled: true }];
        const rawBody = r.body ?? { type: "none" };
        if (
            rawBody.type === "graphql" &&
            "content" in rawBody &&
            !("query" in rawBody)
        ) {
            body.value = {
                type: "graphql",
                query: (rawBody as { content?: string }).content ?? "",
                variables: "",
            };
        } else {
            body.value = rawBody;
        }
        error.value = undefined;
        applyStoredResponse(r.id);
    },
    { immediate: true },
);

watch(
    [url, method, params, headers, body],
    () => {
        const r = props.resource;
        if (!r) return;

        resourcesStore.updateHttpResource(r.id, {
            url: url.value,
            method: method.value,
            params: params.value.filter((p) => p.key.trim() || p.value.trim())
                .length
                ? params.value
                : [],
            headers: headers.value.filter((h) => h.key.trim() || h.value.trim())
                .length
                ? headers.value
                : [],
            body: body.value,
        });
    },
    { deep: true },
);

const bodyDisabled = computed(
    () => method.value === "GET" || method.value === "HEAD",
);

const computedHeadersMap = computed(() => {
    const r =
        props.resource && resourcesStore.getHttpResource(props.resource.id);
    return r ? computedHeaders(r) : {};
});

/**
 * Returns headers for the request: resource headers plus computed defaults for any header name
 * not already present (case-insensitive). Resource headers are appended after computed so they override.
 */
function normalizeHeaders(http: HttpResource): KeyValuePair[] {
    const computed = computedHeaders(http);
    const resourceHeaders = http.headers ?? [];
    const seenLower = new Set(
        resourceHeaders
            .filter((h) => h.key.trim())
            .map((h) => h.key.toLowerCase()),
    );
    const result: KeyValuePair[] = [];
    for (const [key, value] of Object.entries(computed)) {
        if (key && !seenLower.has(key.toLowerCase())) {
            result.push({ key, value, enabled: true });
        }
    }
    result.push(...resourceHeaders);
    return result;
}

async function onSend() {
    const baseUrl = url.value.trim();
    if (!baseUrl) {
        error.value = "Enter a URL";
        return;
    }
    const resource = resourcesStore.getHttpResource(props.resource?.id ?? "");
    if (!resource) {
        error.value = "No resource";
        return;
    }

    const resourceId = resource.id;
    loading.value = true;
    error.value = undefined;

    try {
        const {
            _editor_meta,
            body: bodyValue,
            headers: _h,
            ...rest
        } = resource;

        const bodyPayload = serializeHttpBodyToRequestString(bodyValue);
        const payload = {
            ...rest,
            headers: normalizeHeaders(resource),
            body: bodyPayload ?? null,
        };

        const result = await invoke<{
            status: number;
            headers: [string, string][];
            body: string;
            duration_ms: number;
            url?: string;
            size?: number;
            http_version: string;
        }>("send_http_request", { payload });

        const stored = {
            status: result.status,
            headers: result.headers,
            body: result.body,
            duration_ms: result.duration_ms,
            ...(result.url != null && { url: result.url }),
            ...(result.size != null && { size: result.size }),
            ...(result.http_version != null && {
                http_version: result.http_version,
            }),
        };
        responsesStore.setResponse(resourceId, stored);
        status.value = stored.status;
        responseHeaders.value = stored.headers;
        responseBody.value = stored.body;
        durationMs.value = stored.duration_ms;
        httpVersion.value = stored.http_version;
        responseVersion.value += 1;
    } catch (e) {
        error.value = e instanceof Error ? e.message : String(e);
    } finally {
        loading.value = false;
    }
}
</script>

<template>
    <ResizablePanelGroup direction="vertical">
        <ResizablePanel
            :default-size="50"
            :min-size="40"
            :max-size="60"
            class="flex flex-col"
        >
            <UrlMethodBar
                v-model:url="url"
                v-model:method="method"
                :loading="loading"
                @send="onSend"
            />
            <RequestTabs
                :params="params"
                :headers="headers"
                :body="body"
                :body-disabled="bodyDisabled"
                :computed-headers="computedHeadersMap"
                @update:params="params = $event"
                @update:headers="headers = $event"
                @update:body="body = $event"
            />
        </ResizablePanel>
        <ResizableHandle />
        <ResizablePanel
            :default-size="50"
            :min-size="40"
            :max-size="60"
            class=""
        >
            <ResponsePane
                v-model:tab="resposeTab"
                :key="responseVersion"
                :status="status"
                :headers="responseHeaders"
                :body="responseBody"
                :duration-ms="durationMs"
                :http-version="httpVersion"
                :error="error"
            />
        </ResizablePanel>
    </ResizablePanelGroup>
</template>
