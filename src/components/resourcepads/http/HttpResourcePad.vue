<script setup lang="ts">
import type { KeyValue } from "./types";
import { invoke } from "@tauri-apps/api/core";
import UrlMethodBar from "./UrlMethodBar.vue";
import RequestTabs from "./RequestTabs.vue";
import ResponsePane from "./ResponsePane.vue";
import { ref, computed } from "vue";
import { HttpMethodType } from "@/shared/constants";
import {
    ResizableHandle,
    ResizablePanel,
    ResizablePanelGroup,
} from "@/components/ui/resizable";

const url = ref("");
const method = ref<HttpMethodType>("GET");
const params = ref<KeyValue[]>([{ key: "", value: "" }]);
const headers = ref<KeyValue[]>([{ key: "", value: "" }]);
const body = ref("");

const status = ref<number | undefined>(undefined);
const responseHeaders = ref<[string, string][]>([]);
const responseBody = ref("");
const durationMs = ref<number | undefined>(undefined);
const error = ref<string | undefined>(undefined);
const loading = ref(false);

const bodyDisabled = computed(
    () => method.value === "GET" || method.value === "HEAD",
);

function buildUrlWithParams(base: string, prms: KeyValue[]): string {
    const filtered = prms.filter((p) => p.key.trim());
    if (filtered.length === 0) return base;
    const search = filtered
        .map(
            (p) =>
                `${encodeURIComponent(p.key.trim())}=${encodeURIComponent(p.value)}`,
        )
        .join("&");
    const separator = base.includes("?") ? "&" : "?";
    return `${base}${separator}${search}`;
}

function headersToObject(h: KeyValue[]): Record<string, string> {
    const out: Record<string, string> = {};
    for (const { key, value } of h) {
        const k = key.trim();
        if (k) out[k] = value;
    }
    return out;
}

async function onSend() {
    const fullUrl = buildUrlWithParams(url.value.trim(), params.value);
    if (!fullUrl || fullUrl === "?" || fullUrl.endsWith("?")) {
        error.value = "Enter a URL";
        return;
    }
    loading.value = true;
    error.value = undefined;
    status.value = undefined;
    responseHeaders.value = [];
    responseBody.value = "";
    durationMs.value = undefined;

    try {
        const headersObj = headersToObject(headers.value);
        const payload = {
            method: method.value,
            url: fullUrl,
            headers: Object.keys(headersObj).length ? headersObj : undefined,
            body:
                bodyDisabled.value || !body.value.trim()
                    ? undefined
                    : body.value.trim(),
        };
        const result = await invoke<{
            status: number;
            headers: [string, string][];
            body: string;
            duration_ms: number;
        }>("send_http_request", payload);

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
        <ResizablePanel :default-size="50" :min-size="40" :max-size="60">
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
        <ResizablePanel :default-size="50" :min-size="40" :max-size="60">
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
