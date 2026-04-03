import { computed, reactive, ref } from "vue";
import YAML from "yaml";
import {
    fetchFile,
    fetchRuntimeEnv,
    fetchWorkspace,
    runScript,
    sendHttp,
    type ExecutionResultDto,
    type RuntimeEnvEntry,
    type WorkspaceSnapshot,
} from "@/api";
import type { EditorTab, ReqSubTab } from "@/types/editor";

const HTTP_METHODS = [
    "GET",
    "POST",
    "PUT",
    "PATCH",
    "DELETE",
    "HEAD",
    "OPTIONS",
] as const;

export function useAppModel() {
    const workspace = ref<WorkspaceSnapshot | null>(null);
    const loadErr = ref<string | null>(null);
    const tabs = ref<EditorTab[]>([]);
    const activeId = ref<string | null>(null);
    const reqSubTab = ref<ReqSubTab>("params");
    const response = ref<ExecutionResultDto | null>(null);
    const sendErr = ref<string | null>(null);
    const sending = ref(false);
    const scriptLogs = ref<
        { level: string; message: string; elapsed_ms: number }[]
    >([]);
    const scriptRunError = ref<string | null>(null);
    const bodyView = ref<"pretty" | "raw">("pretty");
    const runtimeEnvEntries = ref<RuntimeEnvEntry[]>([]);
    const runtimeEnvErr = ref<string | null>(null);
    const runtimeEnvLoading = ref(false);

    const activeTab = computed(
        () => tabs.value.find((t) => t.id === activeId.value) ?? null,
    );

    function extFromPath(p: string): string {
        const m = p.match(/\.([^.]+)$/);
        return m ? m[1].toLowerCase() : "json";
    }

    function parseRaw(
        raw: string,
        ext: string,
    ): Record<string, unknown> | null {
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

    async function refreshRuntimeEnv() {
        runtimeEnvLoading.value = true;
        runtimeEnvErr.value = null;
        try {
            const r = await fetchRuntimeEnv();
            runtimeEnvEntries.value = r.entries;
        } catch (e) {
            runtimeEnvErr.value = e instanceof Error ? e.message : String(e);
            runtimeEnvEntries.value = [];
        } finally {
            runtimeEnvLoading.value = false;
        }
    }

    async function loadWorkspace() {
        loadErr.value = null;
        try {
            workspace.value = await fetchWorkspace();
        } catch (e) {
            loadErr.value = e instanceof Error ? e.message : String(e);
        }
    }

    async function openFile(
        path: string,
        kind: EditorTab["kind"],
        title: string,
    ) {
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
            overridesJson: "{}",
        };
        syncDoc(tab);
        tabs.value.push(tab);
        activeId.value = tab.id;
        response.value = null;
        sendErr.value = null;
        scriptLogs.value = [];
        scriptRunError.value = null;
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
            (t.doc as Record<string, unknown>).request = {
                method: "GET",
                url: "",
            };
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

    function parseOverridesForSend(
        raw: string,
    ): { ok: true; overrides: Record<string, string> } | { ok: false; message: string } {
        const s = raw.trim();
        if (s === "" || s === "{}") {
            return { ok: true, overrides: {} };
        }
        try {
            const o = JSON.parse(s) as unknown;
            if (o === null || typeof o !== "object" || Array.isArray(o)) {
                return {
                    ok: false,
                    message: "Overrides must be a JSON object (e.g. {\"ID\": \"42\"})",
                };
            }
            const out: Record<string, string> = {};
            for (const [k, v] of Object.entries(o as Record<string, unknown>)) {
                if (v === null || v === undefined) {
                    out[k] = "";
                } else if (
                    typeof v === "string" ||
                    typeof v === "number" ||
                    typeof v === "boolean"
                ) {
                    out[k] = String(v);
                } else {
                    return {
                        ok: false,
                        message: `Override "${k}" must be a string, number, or boolean`,
                    };
                }
            }
            return { ok: true, overrides: out };
        } catch {
            return { ok: false, message: "Invalid JSON in overrides" };
        }
    }

    async function doSend() {
        const t = activeTab.value;
        if (!t || t.kind !== "request") return;
        syncDoc(t);
        if (!t.doc) {
            sendErr.value = t.parseError ?? "Cannot parse document";
            return;
        }
        const parsed = parseOverridesForSend(t.overridesJson);
        if (!parsed.ok) {
            sendErr.value = parsed.message;
            return;
        }
        const payloadOverrides =
            Object.keys(parsed.overrides).length > 0
                ? parsed.overrides
                : undefined;

        sending.value = true;
        sendErr.value = null;
        response.value = null;
        try {
            const res = await sendHttp({
                source_path: t.path,
                document: t.doc,
                overrides: payloadOverrides,
            });
            if (res.error) sendErr.value = res.error;
            response.value = res.result ?? null;
        } catch (e) {
            sendErr.value = e instanceof Error ? e.message : String(e);
        } finally {
            sending.value = false;
            void refreshRuntimeEnv();
        }
    }

    async function doRunScript() {
        const t = activeTab.value;
        if (!t || t.kind !== "script") return;
        sending.value = true;
        scriptLogs.value = [];
        scriptRunError.value = null;
        sendErr.value = null;
        try {
            const res = await runScript(t.path);
            scriptLogs.value = res.logs;
            scriptRunError.value = res.error ?? null;
        } catch (e) {
            sendErr.value = e instanceof Error ? e.message : String(e);
        } finally {
            sending.value = false;
            void refreshRuntimeEnv();
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
            const h =
                (r?.headers as Record<string, string> | undefined) ?? {};
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

    const overridesJson = computed({
        get() {
            const t = activeTab.value;
            if (!t || t.kind !== "request") return "{}";
            return t.overridesJson;
        },
        set(v: string) {
            const t = activeTab.value;
            if (!t || t.kind !== "request") return;
            t.overridesJson = v;
        },
    });

    const overridesJsonError = computed(() => {
        const t = activeTab.value;
        if (!t || t.kind !== "request") return null;
        const parsed = parseOverridesForSend(t.overridesJson);
        return parsed.ok ? null : parsed.message;
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
        get: () =>
            activeTab.value?.kind === "script" ? activeTab.value.raw : "",
        set: setScriptRaw,
    });

    function setMethod(m: string) {
        const req = ensureRequestDoc();
        if (req) {
            req.method = m;
            applyRequestField();
        }
    }

    function setUrl(url: string) {
        const req = ensureRequestDoc();
        if (req) {
            req.url = url;
            applyRequestField();
        }
    }

    return reactive({
        workspace,
        loadErr,
        loadWorkspace,
        tabs,
        activeId,
        activeTab,
        reqSubTab,
        response,
        sendErr,
        sending,
        scriptLogs,
        scriptRunError,
        bodyView,
        runtimeEnvEntries,
        runtimeEnvErr,
        runtimeEnvLoading,
        refreshRuntimeEnv,
        openFile,
        closeTab,
        requestSpec,
        ensureRequestDoc,
        applyRequestField,
        doSend,
        doRunScript,
        queryJson,
        headersJson,
        bodyText,
        overridesJson,
        overridesJsonError,
        prettyResponse,
        scriptRaw,
        HTTP_METHODS,
        setMethod,
        setUrl,
    });
}

export type AppModel = ReturnType<typeof useAppModel>;
