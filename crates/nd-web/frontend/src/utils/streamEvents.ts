import type { RuntimeEnvEntry } from "@/api";

/** One line from a streamed `Log` event (Rhai `log(...)`). */
export type ScriptLogLine = {
    level: string;
    message: string;
    elapsed_ms: number;
};

/** Matches serde JSON for `std::time::Duration`. */
type SerdeDuration = { secs: number; nanos: number };

function durationToMs(d: SerdeDuration | undefined): number {
    if (!d || typeof d.secs !== "number") return 0;
    return d.secs * 1000 + Math.floor((d.nanos ?? 0) / 1_000_000);
}

/** Externally tagged `Event::Log` from the server (`{ "Log": { ... } }`). */
export function isLogEventData(data: unknown): data is { Log: Record<string, unknown> } {
    if (!data || typeof data !== "object") return false;
    return "Log" in (data as object);
}

/** `RuntimeVariablesInitialized` or `RuntimeVariablePushed` payload keys on the event envelope. */
export function isRuntimeEnvEventData(data: unknown): boolean {
    if (!data || typeof data !== "object") return false;
    const o = data as Record<string, unknown>;
    return (
        "RuntimeVariablesInitialized" in o || "RuntimeVariablePushed" in o
    );
}

/** Returns a new env list when the event updates runtime variables; otherwise `null`. */
export function patchRuntimeEnvFromEvent(
    current: RuntimeEnvEntry[],
    data: unknown,
): RuntimeEnvEntry[] | null {
    if (!isRuntimeEnvEventData(data)) return null;
    const o = data as Record<string, unknown>;
    const init = o.RuntimeVariablesInitialized as
        | { entries?: [string, string][] }
        | undefined;
    if (init?.entries) {
        return init.entries.map(([key, value]) => ({ key, value }));
    }
    const pushed = o.RuntimeVariablePushed as
        | { key?: string; value?: unknown }
        | undefined;
    if (pushed?.key != null) {
        const val =
            typeof pushed.value === "string"
                ? pushed.value
                : JSON.stringify(pushed.value);
        const list = [...current];
        const i = list.findIndex((e) => e.key === pushed.key);
        if (i >= 0) list[i] = { key: pushed.key, value: val };
        else list.push({ key: pushed.key, value: val });
        list.sort((a, b) => a.key.localeCompare(b.key));
        return list;
    }
    return null;
}

export function appendLogFromStreamEvent(
    logs: ScriptLogLine[],
    data: unknown,
): void {
    if (!isLogEventData(data)) return;
    const inner = data.Log as
        | { level?: string; message?: string; elapsed?: SerdeDuration }
        | undefined;
    if (inner?.message == null) return;
    logs.push({
        level: String(inner.level ?? "info"),
        message: inner.message,
        elapsed_ms: durationToMs(inner.elapsed),
    });
}
