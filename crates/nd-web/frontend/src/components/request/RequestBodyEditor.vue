<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { watchDebounced } from "@vueuse/core";
import { useEditorStore } from "@/stores/editor";
import { storeToRefs } from "pinia";
import { Card, CardContent } from "@/components/ui/card";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";

const editor = useEditorStore();
const { requestSpec, activeTab } = storeToRefs(editor);

/** Values on `request.body.type` for structured bodies; matches nd-core `RequestBodyKind`. */
type BodyKindUi =
    | "none"
    | "json"
    | "text"
    | "xml"
    | "graphql"
    | "other"
    | "x_www_form_urlencoded"
    | "form_data"
    | "binary";

const BODY_OPTIONS: { value: BodyKindUi; label: string }[] = [
    { value: "none", label: "No body" },
    { value: "json", label: "JSON" },
    { value: "graphql", label: "GraphQL" },
    { value: "text", label: "Plain text" },
    { value: "xml", label: "XML" },
    {
        value: "x_www_form_urlencoded",
        label: "URL-encoded (x-www-form-urlencoded)",
    },
    { value: "form_data", label: "Multipart (form-data)" },
    { value: "binary", label: "Binary (base64 in content)" },
    { value: "other", label: "Other" },
];

const JSON_KINDS: ReadonlySet<BodyKindUi> = new Set(["json", "graphql"]);
const STRING_KINDS: ReadonlySet<BodyKindUi> = new Set([
    "text",
    "xml",
    "other",
    "x_www_form_urlencoded",
    "form_data",
    "binary",
]);

const ALL_KINDS = new Set<BodyKindUi>(["none", ...JSON_KINDS, ...STRING_KINDS]);

function isStructuredBody(
    body: unknown,
): body is { type: string; content: unknown } {
    if (typeof body !== "object" || body === null || Array.isArray(body)) {
        return false;
    }
    const o = body as Record<string, unknown>;
    return typeof o.type === "string" && "content" in o;
}

function normalizeBodyType(raw: string): BodyKindUi | null {
    const aliases: Record<string, BodyKindUi> = {
        json: "json",
        text: "text",
        xml: "xml",
        graphql: "graphql",
        other: "other",
        binary: "binary",
        none: "none",
        x_www_form_urlencoded: "x_www_form_urlencoded",
        "x-www-form-urlencoded": "x_www_form_urlencoded",
        form_data: "form_data",
        "form-data": "form_data",
    };
    if (raw in aliases) return aliases[raw]!;
    if (ALL_KINDS.has(raw as BodyKindUi)) return raw as BodyKindUi;
    return null;
}

/** Infer UI kind from `request.body` (structured, legacy text, or legacy JSON value). */
function inferKind(body: unknown): BodyKindUi {
    if (body === undefined || body === null) return "none";
    if (typeof body === "string") return "text";
    if (isStructuredBody(body)) {
        const n = normalizeBodyType(body.type);
        if (n === "none") return "none";
        if (n !== null) return n;
    }
    return "json";
}

function isLegacyJsonShorthand(body: unknown): boolean {
    if (body === undefined || body === null) return false;
    if (typeof body === "string") return false;
    return !isStructuredBody(body);
}

function defaultContentForKind(kind: BodyKindUi): unknown {
    if (kind === "graphql") return { query: "" };
    if (kind === "json") return {};
    if (STRING_KINDS.has(kind)) return "";
    return null;
}

function structuredBody(
    kind: BodyKindUi,
    content: unknown,
): Record<string, unknown> {
    return { type: kind, content };
}

function pullJsonText(body: unknown, kind: BodyKindUi): string {
    if (kind === "none" || !JSON_KINDS.has(kind)) return "";
    if (isLegacyJsonShorthand(body)) {
        try {
            return JSON.stringify(body, null, 2);
        } catch {
            return String(body);
        }
    }
    if (isStructuredBody(body)) {
        try {
            return JSON.stringify(body.content, null, 2);
        } catch {
            return "";
        }
    }
    return kind === "graphql" ? JSON.stringify({ query: "" }, null, 2) : "{}";
}

function pullStringText(body: unknown, kind: BodyKindUi): string {
    if (!STRING_KINDS.has(kind)) return "";
    if (typeof body === "string") return body;
    if (isStructuredBody(body)) {
        const c = body.content;
        if (typeof c === "string") return c;
        if (c === null || c === undefined) return "";
        try {
            return JSON.stringify(c);
        } catch {
            return String(c);
        }
    }
    return "";
}

const localJson = ref("");
const localString = ref("");
const jsonError = ref<string | null>(null);

const inferredKind = computed(() => inferKind(requestSpec.value?.body));

function applyBody(body: unknown) {
    const req = editor.ensureRequestDoc();
    if (!req) return;
    if (body === undefined) {
        req.body = undefined;
    } else {
        (req as { body?: unknown }).body = body;
    }
    editor.applyRequestField();
}

function syncEditorsFromBody() {
    const body = requestSpec.value?.body;
    const kind = inferKind(body);
    jsonError.value = null;
    if (kind === "none") {
        localJson.value = "";
        localString.value = "";
        return;
    }
    if (JSON_KINDS.has(kind)) {
        localJson.value = pullJsonText(body, kind);
        localString.value = "";
        return;
    }
    if (STRING_KINDS.has(kind)) {
        localString.value = pullStringText(body, kind);
        localJson.value = "";
    }
}

watch(
    () => [activeTab.value?.id, requestSpec.value?.body] as const,
    () => syncEditorsFromBody(),
    { deep: true, immediate: true },
);

function onKindChange(value: unknown) {
    if (value === undefined || value === null) return;
    const kind = value as BodyKindUi;
    if (!ALL_KINDS.has(kind)) return;
    if (kind === inferKind(requestSpec.value?.body)) {
        return;
    }
    if (kind === "none") {
        applyBody(undefined);
        syncEditorsFromBody();
        return;
    }
    if (JSON_KINDS.has(kind)) {
        const def = defaultContentForKind(kind);
        applyBody(structuredBody(kind, def));
        syncEditorsFromBody();
        return;
    }
    if (STRING_KINDS.has(kind)) {
        applyBody(structuredBody(kind, ""));
        syncEditorsFromBody();
    }
}

function commitJson() {
    const kind = inferredKind.value;
    if (!JSON_KINDS.has(kind)) return;
    const raw = localJson.value.trim();
    let parsed: unknown;
    if (raw === "") {
        parsed = kind === "graphql" ? { query: "" } : {};
    } else {
        try {
            parsed = JSON.parse(raw) as unknown;
        } catch (e) {
            jsonError.value = e instanceof Error ? e.message : "Invalid JSON";
            return;
        }
    }
    jsonError.value = null;
    applyBody(structuredBody(kind, parsed));
    syncEditorsFromBody();
}

function commitString() {
    const kind = inferredKind.value;
    if (!STRING_KINDS.has(kind)) return;
    applyBody(structuredBody(kind, localString.value));
}

watchDebounced(
    localString,
    () => {
        if (!STRING_KINDS.has(inferredKind.value)) return;
        commitString();
    },
    { debounce: 300 },
);

const hint = computed(() => {
    const k = inferredKind.value;
    if (k === "binary") {
        return "Put base64 in the field; it is sent as the raw decoded bytes after expansion.";
    }
    if (k === "x_www_form_urlencoded") {
        return "Use key=value pairs separated by & (e.g. a=1&b=two). Template variables are expanded in the string.";
    }
    if (k === "form_data") {
        return "Multipart payload as a string after expansion; for complex uploads prefer editing the file on disk.";
    }
    return "Default Content-Type follows this type unless you override it under Headers.";
});
</script>

<template>
    <Card class="h-full border-none!">
        <CardContent class="space-y-3 h-full flex flex-col gap-1 p-0!">
            <div
                class="flex flex-col gap-1.5 sm:flex-row sm:items-center sm:gap-3"
            >
                <span
                    class="text-muted-foreground shrink-0 text-[11px] font-medium"
                >
                    Body type
                </span>
                <Select
                    :model-value="inferredKind"
                    @update:model-value="onKindChange"
                >
                    <SelectTrigger class="h-9 max-w-md font-sans text-xs">
                        <SelectValue placeholder="Body type" />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem
                            v-for="opt in BODY_OPTIONS"
                            :key="opt.value"
                            :value="opt.value"
                            class="font-sans text-xs"
                        >
                            {{ opt.label }}
                        </SelectItem>
                    </SelectContent>
                </Select>
            </div>

            <p class="text-muted-foreground text-[11px] leading-snug">
                {{ hint }}
            </p>

            <template v-if="JSON_KINDS.has(inferredKind)">
                <textarea
                    v-model="localJson"
                    class="border-input bg-background focus-visible:ring-ring min-h-52 w-full resize-y rounded-md border p-2 font-mono text-[11px] focus-visible:outline-none focus-visible:ring-1 grow"
                    spellcheck="false"
                    @blur="commitJson"
                />
                <p v-if="jsonError" class="text-sm text-destructive">
                    {{ jsonError }}
                </p>
                <p class="text-muted-foreground text-[11px]">
                    JSON is validated on blur. Use a valid JSON value or object
                    (GraphQL often uses
                    <code class="rounded bg-muted px-1">query</code> /
                    <code class="rounded bg-muted px-1">variables</code>).
                </p>
            </template>

            <template v-else-if="STRING_KINDS.has(inferredKind)">
                <textarea
                    v-model="localString"
                    class="border-input bg-background focus-visible:ring-ring min-h-52 w-full resize-y rounded-md border p-2 font-mono text-[11px] focus-visible:outline-none focus-visible:ring-1 h-full"
                    spellcheck="false"
                />
            </template>
        </CardContent>
    </Card>
</template>
