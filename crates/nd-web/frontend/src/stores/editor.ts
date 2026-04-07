import { defineStore } from "pinia";
import { computed, ref } from "vue";
import YAML from "yaml";
import { fetchFile } from "@/api";
import type { EditorTab, ReqSubTab } from "@/types/editor";

export const HTTP_METHODS = [
    "GET",
    "POST",
    "PUT",
    "PATCH",
    "DELETE",
    "HEAD",
    "OPTIONS",
] as const;

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

export const useEditorStore = defineStore("editor", () => {
    const tabs = ref<EditorTab[]>([]);
    const activeId = ref<string | null>(null);
    const reqSubTab = ref<ReqSubTab>("params");

    const activeTab = computed(
        () => tabs.value.find((t) => t.id === activeId.value) ?? null,
    );

    function syncDoc(tab: EditorTab): void {
        tab.doc = parseRaw(tab.raw, tab.ext);
        tab.parseError = tab.doc ? null : "Invalid JSON/YAML";
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
        const { useExecutionStore } = await import("./execution");
        useExecutionStore().resetAfterOpenFile();
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
    ):
        | { ok: true; overrides: Record<string, string> }
        | { ok: false; message: string } {
        const s = raw.trim();
        if (s === "" || s === "{}") {
            return { ok: true, overrides: {} };
        }
        try {
            const o = JSON.parse(s) as unknown;
            if (o === null || typeof o !== "object" || Array.isArray(o)) {
                return {
                    ok: false,
                    message:
                        'Overrides must be a JSON object (e.g. {"ID": "42"})',
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

    return {
        tabs,
        activeId,
        reqSubTab,
        activeTab,
        openFile,
        closeTab,
        requestSpec,
        ensureRequestDoc,
        applyRequestField,
        syncDoc,
        parseOverridesForSend,
        queryRecord,
        headersRecord,
        overridesRecord,
        overridesJsonError,
        scriptRaw,
        HTTP_METHODS,
        setMethod,
        setUrl,
    };
});
