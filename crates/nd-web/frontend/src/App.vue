<script setup lang="ts">
import { computed, onMounted, ref } from "vue";
import YAML from "yaml";
import {
    fetchFile,
    fetchWorkspace,
    sendHttp,
    runScript,
    type ExecutionResultDto,
    type WorkspaceSnapshot,
} from "./api";

type TabKind = "request" | "script";

interface EditorTab {
    id: string;
    kind: TabKind;
    path: string;
    title: string;
    raw: string;
    ext: string;
    doc: Record<string, unknown> | null;
    parseError: string | null;
}

const workspace = ref<WorkspaceSnapshot | null>(null);
const loadErr = ref<string | null>(null);
const tabs = ref<EditorTab[]>([]);
const activeId = ref<string | null>(null);
const reqSubTab = ref<"params" | "headers" | "body" | "auth">("params");
const response = ref<ExecutionResultDto | null>(null);
const sendErr = ref<string | null>(null);
const sending = ref(false);
const scriptOut = ref<string | null>(null);
const bodyView = ref<"pretty" | "raw">("pretty");

const activeTab = computed(
    () => tabs.value.find((t) => t.id === activeId.value) ?? null,
);

function extFromPath(p: string): string {
    const m = p.match(/\.([^.]+)$/);
    return m ? m[1].toLowerCase() : "json";
}

function parseRaw(raw: string, ext: string): Record<string, unknown> | null {
    try {
        if (ext === "yaml" || ext === "yml") {
            return YAML.parse(raw) as Record<string, unknown>;
        }
        return JSON.parse(raw) as Record<string, unknown>;
    } catch {
        return null;
    }
}

function syncDoc(tab: EditorTab): void {
    tab.doc = parseRaw(tab.raw, tab.ext);
    tab.parseError = tab.doc ? null : "Invalid JSON/YAML";
}

async function loadWorkspace() {
    loadErr.value = null;
    try {
        workspace.value = await fetchWorkspace();
    } catch (e) {
        loadErr.value = e instanceof Error ? e.message : String(e);
    }
}

async function openFile(path: string, kind: TabKind, title: string) {
    const existing = tabs.value.find((t) => t.path === path);
    if (existing) {
        activeId.value = existing.id;
        return;
    }
    const ext = extFromPath(path);
    const raw = await fetchFile(path);
    const tab: EditorTab = {
        id: path,
        kind,
        path,
        title,
        raw,
        ext,
        doc: null,
        parseError: null,
    };
    syncDoc(tab);
    tabs.value.push(tab);
    activeId.value = tab.id;
    response.value = null;
    sendErr.value = null;
    scriptOut.value = null;
}

function closeTab(id: string, ev: MouseEvent) {
    ev.stopPropagation();
    tabs.value = tabs.value.filter((t) => t.id !== id);
    if (activeId.value === id) {
        activeId.value = tabs.value.at(-1)?.id ?? null;
    }
}

const requestSpec = computed(() => {
    const t = activeTab.value;
    if (!t || t.kind !== "request" || !t.doc) return null;
    const req = t.doc.request as Record<string, unknown> | undefined;
    if (!req || typeof req !== "object") return null;
    return req;
});

function ensureRequestDoc(): Record<string, unknown> | null {
    const t = activeTab.value;
    if (!t || t.kind !== "request") return null;
    if (!t.doc) {
        t.doc = {
            version: "0.2.0",
            request: { method: "GET", url: "https://example.com" },
        };
    }
    if (!t.doc.request || typeof t.doc.request !== "object") {
        (t.doc as Record<string, unknown>).request = { method: "GET", url: "" };
    }
    return t.doc.request as Record<string, unknown>;
}

function serializeDoc(tab: EditorTab): string {
    if (tab.ext === "yaml" || tab.ext === "yml") {
        return YAML.stringify(tab.doc);
    }
    return JSON.stringify(tab.doc, null, 2);
}

function applyRequestField() {
    const t = activeTab.value;
    if (!t || t.kind !== "request" || !t.doc) return;
    t.raw = serializeDoc(t);
}

async function doSend() {
    const t = activeTab.value;
    if (!t || t.kind !== "request") return;
    syncDoc(t);
    if (!t.doc) {
        sendErr.value = t.parseError ?? "Cannot parse document";
        return;
    }
    sending.value = true;
    sendErr.value = null;
    response.value = null;
    try {
        const res = await sendHttp({
            source_path: t.path,
            document: t.doc,
        });
        if (res.error) sendErr.value = res.error;
        response.value = res.result ?? null;
    } catch (e) {
        sendErr.value = e instanceof Error ? e.message : String(e);
    } finally {
        sending.value = false;
    }
}

async function doRunScript() {
    const t = activeTab.value;
    if (!t || t.kind !== "script") return;
    sending.value = true;
    scriptOut.value = null;
    sendErr.value = null;
    try {
        const res = await runScript(t.path);
        const lines = res.logs.map((l) => `[${l.level}] ${l.message}`);
        if (res.error) lines.push("— Error —\n" + res.error);
        scriptOut.value =
            lines.join("\n") || (res.ok ? "(no output)" : (res.error ?? ""));
    } catch (e) {
        sendErr.value = e instanceof Error ? e.message : String(e);
    } finally {
        sending.value = false;
    }
}

const queryJson = computed({
    get() {
        const r = requestSpec.value;
        const q = (r?.query as Record<string, string> | undefined) ?? {};
        return JSON.stringify(q, null, 2);
    },
    set(v: string) {
        const req = ensureRequestDoc();
        if (!req) return;
        try {
            req.query = JSON.parse(v) as unknown;
        } catch {
            /* keep previous */
        }
        applyRequestField();
    },
});

const headersJson = computed({
    get() {
        const r = requestSpec.value;
        const h = (r?.headers as Record<string, string> | undefined) ?? {};
        return JSON.stringify(h, null, 2);
    },
    set(v: string) {
        const req = ensureRequestDoc();
        if (!req) return;
        try {
            req.headers = JSON.parse(v) as unknown;
        } catch {
            /* keep previous */
        }
        applyRequestField();
    },
});

const bodyText = computed({
    get() {
        const r = requestSpec.value;
        if (!r?.body) return "";
        try {
            return JSON.stringify(r.body, null, 2);
        } catch {
            return String(r.body);
        }
    },
    set(v: string) {
        const req = ensureRequestDoc();
        if (!req) return;
        const t = v.trim();
        if (!t) {
            req.body = undefined;
        } else {
            try {
                req.body = JSON.parse(t) as unknown;
            } catch {
                req.body = t;
            }
        }
        applyRequestField();
    },
});

const prettyResponse = computed(() => {
    const res = response.value;
    if (!res?.body_text) return "";
    try {
        return JSON.stringify(JSON.parse(res.body_text), null, 2);
    } catch {
        return res.body_text;
    }
});

function setScriptRaw(v: string) {
    const t = activeTab.value;
    if (t && t.kind === "script") t.raw = v;
}

const scriptRaw = computed({
    get: () => (activeTab.value?.kind === "script" ? activeTab.value.raw : ""),
    set: setScriptRaw,
});

onMounted(loadWorkspace);
</script>

<template>
    <div class="flex h-full min-h-0 font-mono text-xs">
        <aside
            class="flex w-56 shrink-0 flex-col border-r border-neutral-300 bg-white"
        >
            <div
                class="border-b border-neutral-300 px-2 py-1 text-[11px] uppercase tracking-wide text-neutral-500"
            >
                NativeDoctor
            </div>
            <div v-if="loadErr" class="p-2 text-red-700">{{ loadErr }}</div>
            <div class="min-h-0 flex-1 overflow-y-auto">
                <div class="px-2 py-1 font-semibold text-neutral-600">
                    Requests
                </div>
                <template
                    v-for="g in workspace?.requests ?? []"
                    :key="'rq-' + g.root_index"
                >
                    <div
                        v-if="(workspace?.roots.length ?? 0) > 1"
                        class="px-2 py-0.5 text-[10px] text-neutral-400"
                    >
                        {{ g.root_label }}
                    </div>
                    <button
                        v-for="e in g.entries"
                        :key="e.path"
                        type="button"
                        class="block w-full truncate px-2 py-0.5 text-left hover:bg-neutral-100"
                        :class="activeId === e.path ? 'bg-neutral-200' : ''"
                        :title="e.path"
                        @click="openFile(e.path, 'request', e.name)"
                    >
                        {{ e.name }}
                    </button>
                </template>
                <div class="mt-3 px-2 py-1 font-semibold text-neutral-600">
                    Scripts
                </div>
                <template
                    v-for="g in workspace?.scripts ?? []"
                    :key="'sc-' + g.root_index"
                >
                    <div
                        v-if="(workspace?.roots.length ?? 0) > 1"
                        class="px-2 py-0.5 text-[10px] text-neutral-400"
                    >
                        {{ g.root_label }}
                    </div>
                    <button
                        v-for="e in g.entries"
                        :key="e.path"
                        type="button"
                        class="block w-full truncate px-2 py-0.5 text-left hover:bg-neutral-100"
                        :class="activeId === e.path ? 'bg-neutral-200' : ''"
                        :title="e.path"
                        @click="openFile(e.path, 'script', e.name)"
                    >
                        {{ e.name }}
                    </button>
                </template>
            </div>
        </aside>

        <main class="flex min-w-0 flex-1 flex-col bg-neutral-50">
            <div
                v-if="!activeTab"
                class="flex flex-1 items-center justify-center text-neutral-400"
            >
                Select a request or script
            </div>

            <template v-else>
                <div
                    class="flex flex-wrap border-b border-neutral-300 bg-white"
                >
                    <button
                        v-for="t in tabs"
                        :key="t.id"
                        type="button"
                        class="flex max-w-45 items-center gap-1 border-r border-neutral-200 px-2 py-1"
                        :class="activeId === t.id ? 'bg-neutral-100' : ''"
                        @click="activeId = t.id"
                    >
                        <span class="truncate">{{ t.title }}</span>
                        <span
                            class="text-neutral-400 hover:text-neutral-700"
                            @click="closeTab(t.id, $event)"
                            >×</span
                        >
                    </button>
                </div>

                <template v-if="activeTab.kind === 'request'">
                    <div
                        class="flex flex-wrap items-center gap-2 border-b border-neutral-300 bg-white px-2 py-1"
                    >
                        <select
                            v-if="requestSpec"
                            class="border border-neutral-300 bg-white px-1 py-0.5"
                            :value="String(requestSpec.method ?? 'GET')"
                            @change="
                                (e) => {
                                    const req = ensureRequestDoc();
                                    if (req)
                                        req.method = (
                                            e.target as HTMLSelectElement
                                        ).value;
                                    applyRequestField();
                                }
                            "
                        >
                            <option
                                v-for="m in [
                                    'GET',
                                    'POST',
                                    'PUT',
                                    'PATCH',
                                    'DELETE',
                                    'HEAD',
                                    'OPTIONS',
                                ]"
                                :key="m"
                            >
                                {{ m }}
                            </option>
                        </select>
                        <input
                            v-if="requestSpec"
                            class="min-w-50 flex-1 border border-neutral-300 bg-white px-1 py-0.5"
                            :value="String(requestSpec.url ?? '')"
                            @input="
                                (e) => {
                                    const req = ensureRequestDoc();
                                    if (req)
                                        req.url = (
                                            e.target as HTMLInputElement
                                        ).value;
                                    applyRequestField();
                                }
                            "
                        />
                        <button
                            type="button"
                            class="border border-neutral-400 bg-neutral-100 px-2 py-0.5 hover:bg-neutral-200 disabled:opacity-50"
                            :disabled="sending"
                            @click="doSend"
                        >
                            Send
                        </button>
                    </div>

                    <div class="flex border-b border-neutral-300 bg-white px-2">
                        <button
                            v-for="sub in [
                                'params',
                                'headers',
                                'body',
                                'auth',
                            ] as const"
                            :key="sub"
                            type="button"
                            class="px-2 py-1 capitalize"
                            :class="
                                reqSubTab === sub
                                    ? 'border-b border-neutral-800'
                                    : 'text-neutral-500'
                            "
                            @click="reqSubTab = sub"
                        >
                            {{ sub }}
                        </button>
                    </div>

                    <div class="min-h-0 flex-1 overflow-auto p-2">
                        <div v-if="activeTab.parseError" class="text-red-700">
                            {{ activeTab.parseError }} — fix document
                        </div>

                        <div v-show="reqSubTab === 'params'">
                            <textarea
                                v-model="queryJson"
                                class="h-40 w-full border border-neutral-300 bg-white p-1 font-mono text-[11px]"
                                spellcheck="false"
                            />
                        </div>

                        <div v-show="reqSubTab === 'headers'">
                            <textarea
                                v-model="headersJson"
                                class="h-40 w-full border border-neutral-300 bg-white p-1 font-mono text-[11px]"
                                spellcheck="false"
                            />
                        </div>

                        <div v-show="reqSubTab === 'body'">
                            <textarea
                                v-model="bodyText"
                                class="h-48 w-full border border-neutral-300 bg-white p-1"
                                spellcheck="false"
                            />
                        </div>

                        <div
                            v-show="reqSubTab === 'auth'"
                            class="text-neutral-500"
                        >
                            Use Headers for <code>Authorization</code>, or env
                            vars in the URL/body.
                        </div>
                    </div>

                    <div
                        class="flex max-h-[45vh] min-h-30 flex-col border-t border-neutral-300 bg-white"
                    >
                        <div
                            class="flex items-center gap-2 border-b border-neutral-200 px-2 py-1"
                        >
                            <span class="font-semibold">Response</span>
                            <span v-if="response" class="text-neutral-600">
                                {{ response.status }} ·
                                {{ response.duration_ms }}ms
                            </span>
                            <span v-if="sendErr" class="text-red-700">
                                {{ sendErr }}
                            </span>
                        </div>
                        <div
                            class="flex gap-2 border-b border-neutral-200 px-2"
                        >
                            <button
                                type="button"
                                :class="
                                    bodyView === 'pretty'
                                        ? 'border-b border-neutral-800'
                                        : 'text-neutral-500'
                                "
                                @click="bodyView = 'pretty'"
                            >
                                Body
                            </button>
                            <button
                                type="button"
                                :class="
                                    bodyView === 'raw'
                                        ? 'border-b border-neutral-800'
                                        : 'text-neutral-500'
                                "
                                @click="bodyView = 'raw'"
                            >
                                Raw
                            </button>
                        </div>
                        <div
                            v-if="response"
                            class="min-h-0 flex-1 overflow-auto p-2"
                        >
                            <pre
                                class="whitespace-pre-wrap break-all text-[11px]"
                                >{{
                                    bodyView === "pretty"
                                        ? prettyResponse
                                        : (response.body_text ??
                                          (response.body_base64
                                              ? "[binary base64]"
                                              : ""))
                                }}</pre
                            >
                            <details class="mt-2">
                                <summary
                                    class="cursor-pointer text-neutral-500"
                                >
                                    Headers
                                </summary>
                                <pre class="mt-1 text-[11px]">{{
                                    JSON.stringify(response.headers, null, 2)
                                }}</pre>
                            </details>
                        </div>
                    </div>
                </template>

                <template v-else>
                    <div
                        class="flex items-center gap-2 border-b border-neutral-300 bg-white px-2 py-1"
                    >
                        <button
                            type="button"
                            class="border border-neutral-400 bg-neutral-100 px-2 py-0.5 hover:bg-neutral-200 disabled:opacity-50"
                            :disabled="sending"
                            @click="doRunScript"
                        >
                            Run
                        </button>
                        <span v-if="sendErr" class="text-red-700">{{
                            sendErr
                        }}</span>
                    </div>
                    <textarea
                        v-model="scriptRaw"
                        class="min-h-50 w-full flex-1 border-0 bg-white p-2 font-mono text-xs"
                        spellcheck="false"
                    />
                    <pre
                        class="max-h-[40vh] min-h-20 overflow-auto border-t border-neutral-300 bg-neutral-100 p-2 text-[11px]"
                        >{{ scriptOut }}</pre
                    >
                </template>
            </template>
        </main>
    </div>
</template>
