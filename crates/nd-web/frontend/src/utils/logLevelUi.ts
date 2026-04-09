/** Parse `{ Log: { level?, message? } }` from a timeline instant row's `raw`. */
export function parseLogFromTimelineInstant(
    raw: unknown,
): { level: string; message: string } | null {
    if (!raw || typeof raw !== "object") return null;
    const o = raw as Record<string, unknown>;
    const inner = o.Log;
    if (!inner || typeof inner !== "object") return null;
    const rec = inner as Record<string, unknown>;
    return {
        level: String(rec.level ?? "info"),
        message: String(rec.message ?? ""),
    };
}

/** Tailwind classes for a compact log-level chip (timeline, etc.). */
export function logLevelChipClass(level: string): string {
    const u = level.toLowerCase();
    if (u === "error" || u === "fatal") {
        return "bg-destructive/15 text-destructive ring-destructive/25 ring-1";
    }
    if (u === "warn" || u === "warning") {
        return "bg-amber-500/15 text-amber-800 ring-amber-500/30 ring-1 dark:text-amber-400";
    }
    if (u === "debug" || u === "trace") {
        return "bg-muted text-muted-foreground ring-border ring-1";
    }
    return "bg-primary/12 text-primary ring-primary/20 ring-1";
}

/** Parse `{ AssertCalled: { passed?, message? } }` from a timeline instant row's `raw`. */
export function parseAssertFromTimelineInstant(
    raw: unknown,
): { passed: boolean; message: string } | null {
    if (!raw || typeof raw !== "object") return null;
    const inner = (raw as Record<string, unknown>).AssertCalled;
    if (!inner || typeof inner !== "object") return null;
    const rec = inner as Record<string, unknown>;
    return {
        passed: Boolean(rec.passed),
        message: String(rec.message ?? ""),
    };
}

/** Chip next to the timeline marker for assertion pass (green) or fail (red). */
export function assertPassFailChipClass(passed: boolean): string {
    if (passed) {
        return "bg-emerald-600/15 text-emerald-800 ring-emerald-600/30 ring-1 dark:text-emerald-300";
    }
    return "bg-destructive/15 text-destructive ring-destructive/25 ring-1";
}
