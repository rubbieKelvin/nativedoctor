import type { RuntimeEnvEntry } from "@/api";

/** Matches serde JSON for `std::time::Duration`. */
type SerdeDuration = { secs: number; nanos: number };

function durationToMs(d: SerdeDuration | undefined): number {
    if (!d || typeof d.secs !== "number") return 0;
    return d.secs * 1000 + Math.floor((d.nanos ?? 0) / 1_000_000);
}

/** Returns a new env list when the event updates runtime variables; otherwise `null`. */
export function patchRuntimeEnvFromEvent(
    current: RuntimeEnvEntry[],
    data: unknown,
): RuntimeEnvEntry[] | null {
    if (!data || typeof data !== "object") return null;
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
