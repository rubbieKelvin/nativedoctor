import { defineStore } from "pinia";
import { computed, ref } from "vue";
import {
    runSessionCommand,
    type ExecutionResultDto,
    type RuntimeEnvEntry,
} from "@/api";
import {
    appendLogFromStreamEvent,
    patchRuntimeEnvFromEvent,
} from "@/utils/streamEvents";

export const useExecutionStore = defineStore("execution", () => {
    const response = ref<ExecutionResultDto | null>(null);
    const sendErr = ref<string | null>(null);
    const sending = ref(false);
    const scriptLogs = ref<
        { level: string; message: string; elapsed_ms: number }[]
    >([]);
    const scriptRunError = ref<string | null>(null);
    const bodyView = ref<"pretty" | "raw">("pretty");
    const runtimeEnvEntries = ref<RuntimeEnvEntry[]>([]);

    const prettyResponse = computed(() => {
        const res = response.value;
        if (!res?.body_text) return "";
        try {
            return JSON.stringify(JSON.parse(res.body_text), null, 2);
        } catch {
            return res.body_text;
        }
    });

    function resetAfterOpenFile() {
        response.value = null;
        sendErr.value = null;
        scriptLogs.value = [];
        scriptRunError.value = null;
        runtimeEnvEntries.value = [];
    }

    async function doSend() {
        const { useEditorStore } = await import("./editor");
        const editor = useEditorStore();
        const t = editor.activeTab;
        if (!t || t.kind !== "request") return;
        editor.syncDoc(t);
        if (!t.doc) {
            sendErr.value = t.parseError ?? "Cannot parse document";
            return;
        }
        const parsed = editor.parseOverridesForSend(t.overridesJson);
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
                        const next = patchRuntimeEnvFromEvent(
                            runtimeEnvEntries.value,
                            ev,
                        );
                        if (next) runtimeEnvEntries.value = next;
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
        const { useEditorStore } = await import("./editor");
        const editor = useEditorStore();
        const t = editor.activeTab;
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
                        const next = patchRuntimeEnvFromEvent(
                            runtimeEnvEntries.value,
                            ev,
                        );
                        if (next) runtimeEnvEntries.value = next;
                        appendLogFromStreamEvent(logs, ev);
                    },
                },
            );
            scriptLogs.value = logs;
            scriptRunError.value = res.ok
                ? null
                : (res.error ?? "Script failed");
        } catch (e) {
            sendErr.value = e instanceof Error ? e.message : String(e);
        } finally {
            sending.value = false;
        }
    }

    return {
        response,
        sendErr,
        sending,
        scriptLogs,
        scriptRunError,
        bodyView,
        runtimeEnvEntries,
        prettyResponse,
        resetAfterOpenFile,
        doSend,
        doRunScript,
    };
});
