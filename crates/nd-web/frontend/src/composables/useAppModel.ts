import { computed, reactive, ref } from "vue";
import type { Ref } from "vue";
import YAML from "yaml";
import {
    fetchFile,
    fetchWorkspace,
    runSessionCommand,
    type ExecutionResultDto,
    type RuntimeEnvEntry,
    type WorkspaceSnapshot,
} from "@/api";
import type { EditorTab, ReqSubTab } from "@/types/editor";

/** Matches serde JSON for `std::time::Duration`. */
type SerdeDuration = { secs: number; nanos: number };

function durationToMs(d: SerdeDuration | undefined): number {
    if (!d || typeof d.secs !== "number") return 0;
    return d.secs * 1000 + Math.floor((d.nanos ?? 0) / 1_000_000);
}

/** Updates env rows from streamed `Event` JSON (serde externally-tagged enum). */
function applyRuntimeEnvFromStreamEvent(
    target: Ref<RuntimeEnvEntry[]>,
    data: unknown,
): void {
    if (!data || typeof data !== "object") return;
    const o = data as Record<string, unknown>;
    const init = o.RuntimeVariablesInitialized as
        | { entries?: [string, string][] }
        | undefined;
    if (init?.entries) {
        target.value = init.entries.map(([key, value]) => ({ key, value }));
        return;
    }
    const pushed = o.RuntimeVariablePushed as
        | { key?: string; value?: unknown }
        | undefined;
    if (pushed?.key != null) {
        const val =
            typeof pushed.value === "string"
                ? pushed.value
                : JSON.stringify(pushed.value);
        const list = [...target.value];
        const i = list.findIndex((e) => e.key === pushed.key);
        if (i >= 0) list[i] = { key: pushed.key, value: val };
        else list.push({ key: pushed.key, value: val });
        list.sort((a, b) => a.key.localeCompare(b.key));
        target.value = list;
    }
}

function appendLogFromStreamEvent(
    logs: { level: string; message: string; elapsed_ms: number }[],
    data: unknown,
): void {
    if (!data || typeof data !== "object") return;
    const o = data as Record<string, unknown>;
    const inner = o.Log as
        | { level?: string; message?: string; elapsed?: SerdeDuration }
        | undefined;
    if (inner?.message == null) return;
    logs.push({
        level: String(inner.level ?? "info"),
        message: inner.message,
        elapsed_ms: durationToMs(inner.elapsed),
    });
}

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
        runtimeEnvEntries.value = [];
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

/** Flatten overrides JSON to a string map for the KeyValue editor; invalid → {}. */
    function parseOverridesToRecord(raw: string): Record<string, string> {
        const s = raw.trim();
        if (s === "" || s === "{}") {
            return {};
        }
        try {
            const o = JSON.parse(s) as unknown;
            if (o === null || typeof o !== "object" || Array.isArray(o)) {
                return {};
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
                    return {};
                }
            }
            return out;
        } catch {
            return {};
        }
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
            const res = await runSessionCommand(
                {
                    type: "run_request",
                    source_path: t.path,
                    document: t.doc,
                    overrides: payloadOverrides ?? {},
                    stream: false,
                },
                {
                    onEvent: (ev) => {
                        applyRuntimeEnvFromStreamEvent(runtimeEnvEntries, ev);
                    },
                },
            );
            if (!res.ok) sendErr.value = res.error ?? "Request failed";
            else sendErr.value = null;
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
        scriptLogs.value = [];
        scriptRunError.value = null;
        sendErr.value = null;
        try {
            const logs: {
                level: string;
                message: string;
                elapsed_ms: number;
            }[] = [];
            const res = await runSessionCommand(
                {
                    type: "run_script",
                    path: t.path,
                },
                {
                    onEvent: (ev) => {
                        applyRuntimeEnvFromStreamEvent(runtimeEnvEntries, ev);
                        appendLogFromStreamEvent(logs, ev);
                    },
                },
            );
            scriptLogs.value = logs;
            scriptRunError.value = res.ok ? null : (res.error ?? "Script failed");
        } catch (e) {
            sendErr.value = e instanceof Error ? e.message : String(e);
        } finally {
            sending.value = false;
        }
    }

    function recordFromQueryish(
        src: Record<string, unknown> | undefined,
    ): Record<string, string> {
        const out: Record<string, string> = {};
        if (!src || typeof src !== "object") return out;
        for (const [k, v] of Object.entries(src)) {
            if (v === null || v === undefined) {
                out[k] = "";
            } else if (
                typeof v === "string" ||
                typeof v === "number" ||
                typeof v === "boolean"
            ) {
                out[k] = String(v);
            } else {
                out[k] = JSON.stringify(v);
            }
        }
        return out;
    }

    const queryRecord = computed({
        get() {
            const r = requestSpec.value;
            const q = r?.query as Record<string, unknown> | undefined;
            return recordFromQueryish(q);
        },
        set(rec: Record<string, string>) {
            const req = ensureRequestDoc();
            if (!req) return;
            req.query = { ...rec };
            applyRequestField();
        },
    });

    const headersRecord = computed({
        get() {
            const r = requestSpec.value;
            const h = r?.headers as Record<string, unknown> | undefined;
            return recordFromQueryish(h);
        },
        set(rec: Record<string, string>) {
            const req = ensureRequestDoc();
            if (!req) return;
            req.headers = { ...rec };
            applyRequestField();
        },
    });

    const overridesRecord = computed({
        get() {
            const t = activeTab.value;
            if (!t || t.kind !== "request") return {};
            return parseOverridesToRecord(t.overridesJson);
        },
        set(rec: Record<string, string>) {
            const t = activeTab.value;
            if (!t || t.kind !== "request") return;
            t.overridesJson = JSON.stringify(rec, null, 2);
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
        openFile,
        closeTab,
        requestSpec,
        ensureRequestDoc,
        applyRequestField,
        doSend,
        doRunScript,
        queryRecord,
        headersRecord,
        overridesRecord,
        overridesJsonError,
        prettyResponse,
        scriptRaw,
        HTTP_METHODS,
        setMethod,
        setUrl,
    });
}

export type AppModel = ReturnType<typeof useAppModel>;
