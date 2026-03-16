<script setup lang="ts">
import type { KeyValue } from "./types";
import type { HttpResource } from "@/shared/types/resources";
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

const props = withDefaults(defineProps<{ resource?: HttpResource | null }>(), {
    resource: undefined,
});

const resourcesStore = useResources();

const url = ref("");
const method = ref<HttpMethodType>("GET");
const params = ref<KeyValue[]>([{ key: "", value: "", enabled: true }]);
const headers = ref<KeyValue[]>([{ key: "", value: "", enabled: true }]);
const body = ref("");

function bodyFromResource(r: HttpResource): string {
    const b = r.body;
    if (b.type === "text" || b.type === "json" || b.type === "graphql")
        return b.content;
    return "";
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
        body.value = bodyFromResource(r);
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
            body: body.value.trim()
                ? { type: "text", content: body.value }
                : { type: "none" },
        });
    },
    { deep: true },
);

const status = ref<number | undefined>(undefined);
const responseHeaders = ref<[string, string][]>([]);
const responseBody = ref("");
const durationMs = ref<number | undefined>(undefined);
const error = ref<string | undefined>(undefined);
const loading = ref(false);

const bodyDisabled = computed(
    () => method.value === "GET" || method.value === "HEAD",
);

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

    loading.value = true;
    error.value = undefined;
    status.value = undefined;
    responseHeaders.value = [];
    responseBody.value = "";
    durationMs.value = undefined;

    try {
        const { _editor_meta, ...rest } = resource;

        const payload = {
            ...rest,
            // url: baseUrl,
            // method: method.value,
            // params: params.value,
            // headers: headers.value,
            // body:
            //     bodyDisabled.value || !body.value.trim()
            //         ? undefined
            //         : body.value.trim(),
        };

        const result = await invoke<{
            status: number;
            headers: [string, string][];
            body: string;
            duration_ms: number;
        }>("send_http_request", { payload });

        status.value = result.status;
        responseHeaders.value = result.headers;
        responseBody.value = result.body;
        durationMs.value = result.duration_ms;
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
            class="p-2"
        >
            <ResponsePane
                :status="status"
                :headers="responseHeaders"
                :body="responseBody"
                :duration-ms="durationMs"
                :error="error"
            />
        </ResizablePanel>
    </ResizablePanelGroup>
</template>
