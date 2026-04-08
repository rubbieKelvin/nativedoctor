import { defineStore } from "pinia";
import { computed, reactive, ref } from "vue";
import type { ExecutionResultDto, RuntimeEnvEntry } from "@/api";
import { startSessionRun } from "@/session/sessionRun";
import {
    appendLogFromStreamEvent,
    patchRuntimeEnvFromEvent,
    type ScriptLogLine,
} from "@/utils/streamEvents";
import {
    applyStreamEventToTimeline,
    createEmptyTimelineState,
    flushOpenTimelineSpans,
    type TimelineReducerState,
} from "@/utils/streamTimeline";

export const useExecutionStore = defineStore("execution", () => {
    const response = ref<ExecutionResultDto | null>(null);
    /** Transport / validation errors keyed by source file path. */
    const sendErrByPath = reactive<Record<string, string | null>>({});
    const sending = ref(false);
    const scriptLogsByPath = reactive<Record<string, ScriptLogLine[]>>({});
    const scriptRunErrorByPath = reactive<Record<string, string | null>>({});
    const bodyView = ref<"pretty" | "raw">("pretty");
    const runtimeEnvByPath = reactive<Record<string, RuntimeEnvEntry[]>>({});
    const scriptTimelineByPath = reactive<Record<string, TimelineReducerState>>(
        {},
    );

    const prettyResponse = computed(() => {
        const res = response.value;
        if (!res?.body_text) return "";
        try {
            return JSON.stringify(JSON.parse(res.body_text), null, 2);
        } catch {
            return res.body_text;
        }
    });

    function clearKeyedExecutionState() {
        for (const k of Object.keys(scriptLogsByPath)) {
            delete scriptLogsByPath[k];
        }
        for (const k of Object.keys(runtimeEnvByPath)) {
            delete runtimeEnvByPath[k];
        }
        for (const k of Object.keys(sendErrByPath)) {
            delete sendErrByPath[k];
        }
        for (const k of Object.keys(scriptRunErrorByPath)) {
            delete scriptRunErrorByPath[k];
        }
        for (const k of Object.keys(scriptTimelineByPath)) {
            delete scriptTimelineByPath[k];
        }
    }

    /** Full reset when opening a new editor tab from the workspace (matches prior global wipe). */
    function resetAfterOpenFile() {
        response.value = null;
        sending.value = false;
        clearKeyedExecutionState();
    }

    function ensureLogs(path: string): ScriptLogLine[] {
        if (!scriptLogsByPath[path]) scriptLogsByPath[path] = [];
        return scriptLogsByPath[path];
    }

    function ensureTimeline(path: string): TimelineReducerState {
        if (!scriptTimelineByPath[path]) {
            scriptTimelineByPath[path] = createEmptyTimelineState();
        }
        return scriptTimelineByPath[path]!;
    }

    async function doSend() {
        const { useEditorStore } = await import("./editor");
        const editor = useEditorStore();
        const t = editor.activeTab;
        if (!t || t.kind !== "request") return;
        const path = t.path;
        editor.syncDoc(t);
        if (!t.doc) {
            sendErrByPath[path] = t.parseError ?? "Cannot parse document";
            return;
        }
        const parsed = editor.parseOverridesForSend(t.overridesJson);
        if (!parsed.ok) {
            sendErrByPath[path] = parsed.message;
            return;
        }
        const payloadOverrides =
            Object.keys(parsed.overrides).length > 0
                ? parsed.overrides
                : undefined;

        sending.value = true;
        sendErrByPath[path] = null;
        response.value = null;
        runtimeEnvByPath[path] = [];

        const run = startSessionRun({
            type: "run_request",
            source_path: path,
            document: t.doc,
            overrides: payloadOverrides ?? {},
            stream: false,
        });

        const unsubs: (() => void)[] = [];

        unsubs.push(
            run.subscribe((ev) => {
                const cur = runtimeEnvByPath[path] ?? [];
                const next = patchRuntimeEnvFromEvent(cur, ev);
                if (next) runtimeEnvByPath[path] = next;
            }),
        );
        // More handlers (e.g. streamed body chunks): add further `run.subscribe(...)` calls here.

        try {
            const res = await run.completed;
            if (!res.ok) sendErrByPath[path] = res.error ?? "Request failed";
            else sendErrByPath[path] = null;
            response.value = (res.result ?? null) as ExecutionResultDto | null;
        } catch (e) {
            sendErrByPath[path] = e instanceof Error ? e.message : String(e);
        } finally {
            unsubs.forEach((u) => u());
            sending.value = false;
        }
    }

    async function doRunScript() {
        const { useEditorStore } = await import("./editor");
        const editor = useEditorStore();
        const t = editor.activeTab;
        if (!t || t.kind !== "script") return;
        const path = t.path;

        sending.value = true;
        scriptLogsByPath[path] = [];
        runtimeEnvByPath[path] = [];
        scriptTimelineByPath[path] = createEmptyTimelineState();
        scriptRunErrorByPath[path] = null;
        sendErrByPath[path] = null;

        const timeline = ensureTimeline(path);

        const run = startSessionRun({
            type: "run_script",
            path,
        });

        const unsubs: (() => void)[] = [];

        unsubs.push(
            run.subscribe((ev) => {
                const cur = runtimeEnvByPath[path] ?? [];
                const next = patchRuntimeEnvFromEvent(cur, ev);
                if (next) runtimeEnvByPath[path] = next;
            }),
        );

        unsubs.push(
            run.subscribe((ev) => {
                appendLogFromStreamEvent(ensureLogs(path), ev);
            }),
        );

        unsubs.push(
            run.subscribe((ev) => {
                applyStreamEventToTimeline(timeline, ev);
            }),
        );

        let completedOk = false;
        try {
            const res = await run.completed;
            completedOk = res.ok;
            scriptRunErrorByPath[path] = res.ok
                ? null
                : (res.error ?? "Script failed");
        } catch (e) {
            sendErrByPath[path] = e instanceof Error ? e.message : String(e);
        } finally {
            unsubs.forEach((u) => u());
            const benignEnd =
                completedOk && sendErrByPath[path] == null;
            flushOpenTimelineSpans(timeline, benignEnd);
            sending.value = false;
        }
    }

    return {
        response,
        sendErrByPath,
        sending,
        scriptLogsByPath,
        scriptRunErrorByPath,
        bodyView,
        runtimeEnvByPath,
        scriptTimelineByPath,
        prettyResponse,
        resetAfterOpenFile,
        doSend,
        doRunScript,
    };
});
