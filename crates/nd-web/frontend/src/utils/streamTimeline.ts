import {
    serdeDurationToMs,
    type SerdeDuration,
} from "@/utils/streamEvents";

export type TimelineSpanStatus = "ok" | "error" | "running" | "interrupted";

export type TimelineRowInstant = {
    kind: "instant";
    id: string;
    tMs: number;
    variant: string;
    label: string;
    raw: unknown;
};

export type TimelineRowSpan = {
    kind: "span";
    id: string;
    variant: string;
    label: string;
    startMs: number;
    endMs: number | null;
    status: TimelineSpanStatus;
    rawStart: unknown;
    rawEnd?: unknown;
};

export type TimelineRow = TimelineRowInstant | TimelineRowSpan;

export type OpenHttpEntry = { rowId: string; nameKey: string };

export type TimelineReducerState = {
    rows: TimelineRow[];
    lastElapsedMs: number;
    nextRowId: number;
    openHttpStack: OpenHttpEntry[];
    openScriptRowId: string | null;
    openSessionRowId: string | null;
    /** checkpoint_id -> span row id */
    openCheckpoints: Map<string, string>;
};

const SKIP_TIMELINE_KEYS = new Set([
    "HttpResponseStreamStarted",
    "HttpResponseStreamChunk",
    "HttpResponseStreamEnded",
]);

function nextId(state: TimelineReducerState): string {
    return `tl-${state.nextRowId++}`;
}

function bumpElapsed(state: TimelineReducerState, ms: number): void {
    if (ms > state.lastElapsedMs) state.lastElapsedMs = ms;
}

function innerPayload(data: unknown, variant: string): Record<string, unknown> {
    const o = data as Record<string, unknown>;
    const inner = o[variant];
    if (!inner || typeof inner !== "object") return {};
    return inner as Record<string, unknown>;
}

function elapsedFromInner(inner: Record<string, unknown>): number {
    return serdeDurationToMs(inner.elapsed as SerdeDuration | undefined);
}

function normRequestName(n: unknown): string {
    if (n == null || n === undefined) return "";
    return String(n);
}

/** Externally tagged session event: single variant key on the object (e.g. `Log`, `HttpRequestStarted`). */
export function getEventVariantKey(data: unknown): string | null {
    if (!data || typeof data !== "object") return null;
    const o = data as Record<string, unknown>;
    const keys = Object.keys(o).filter((k) => o[k] !== undefined);
    if (keys.length === 0) return null;
    if (keys.length === 1) return keys[0]!;
    const withoutKind = keys.filter((k) => k !== "kind");
    if (withoutKind.length === 1) return withoutKind[0]!;
    const variantLike = keys.filter(
        (k) =>
            k !== "kind" &&
            k !== "message" &&
            /^[A-Z]/.test(k) &&
            typeof o[k] === "object",
    );
    if (variantLike.length === 1) return variantLike[0]!;
    return null;
}

export function createEmptyTimelineState(): TimelineReducerState {
    return {
        rows: [],
        lastElapsedMs: 0,
        nextRowId: 1,
        openHttpStack: [],
        openScriptRowId: null,
        openSessionRowId: null,
        openCheckpoints: new Map(),
    };
}

function pushInstant(
    state: TimelineReducerState,
    variant: string,
    label: string,
    tMs: number,
    raw: unknown,
): void {
    bumpElapsed(state, tMs);
    state.rows.push({
        kind: "instant",
        id: nextId(state),
        tMs,
        variant,
        label,
        raw,
    });
}

function closeHttpSpan(
    state: TimelineReducerState,
    nameKey: string,
    endMs: number,
    rawEnd: unknown,
    httpOk: boolean,
): void {
    bumpElapsed(state, endMs);
    let idx = state.openHttpStack.findIndex((e) => e.nameKey === nameKey);
    if (idx < 0 && state.openHttpStack.length > 0) idx = 0;
    if (idx < 0) return;
    const [entry] = state.openHttpStack.splice(idx, 1);
    const row = state.rows.find((r) => r.id === entry!.rowId) as
        | TimelineRowSpan
        | undefined;
    if (!row || row.kind !== "span") return;
    row.endMs = endMs;
    row.rawEnd = rawEnd;
    row.status = httpOk ? "ok" : "error";
}

function closeScriptSpan(
    state: TimelineReducerState,
    endMs: number,
    rawEnd: unknown,
    success: boolean,
): void {
    const id = state.openScriptRowId;
    state.openScriptRowId = null;
    if (!id) return;
    bumpElapsed(state, endMs);
    const row = state.rows.find((r) => r.id === id) as TimelineRowSpan | undefined;
    if (!row || row.kind !== "span") return;
    row.endMs = endMs;
    row.rawEnd = rawEnd;
    row.status = success ? "ok" : "error";
}

function closeSessionSpan(state: TimelineReducerState, endMs: number, rawEnd: unknown): void {
    const id = state.openSessionRowId;
    state.openSessionRowId = null;
    if (!id) return;
    bumpElapsed(state, endMs);
    const row = state.rows.find((r) => r.id === id) as TimelineRowSpan | undefined;
    if (!row || row.kind !== "span") return;
    row.endMs = endMs;
    row.rawEnd = rawEnd;
    row.status = "ok";
}

function closeCheckpointSpan(
    state: TimelineReducerState,
    checkpointId: string,
    endMs: number,
    rawEnd: unknown,
): void {
    const rowId = state.openCheckpoints.get(checkpointId);
    state.openCheckpoints.delete(checkpointId);
    if (!rowId) return;
    bumpElapsed(state, endMs);
    const row = state.rows.find((r) => r.id === rowId) as TimelineRowSpan | undefined;
    if (!row || row.kind !== "span") return;
    row.endMs = endMs;
    row.rawEnd = rawEnd;
    row.status = "ok";
}

export function applyStreamEventToTimeline(
    state: TimelineReducerState,
    data: unknown,
): void {
    const variant = getEventVariantKey(data);
    if (!variant) return;
    if (SKIP_TIMELINE_KEYS.has(variant)) return;

    const inner = innerPayload(data, variant);
    const tMs = elapsedFromInner(inner);

    switch (variant) {
        case "SessionStarted": {
            bumpElapsed(state, tMs);
            const id = nextId(state);
            state.openSessionRowId = id;
            state.rows.push({
                kind: "span",
                id,
                variant: "Session",
                label: "Session",
                startMs: tMs,
                endMs: null,
                status: "running",
                rawStart: data,
            });
            break;
        }
        case "SessionEnded": {
            closeSessionSpan(state, tMs, data);
            break;
        }
        case "HttpRequestStarted": {
            bumpElapsed(state, tMs);
            const method = String(inner.method ?? "?");
            const url = String(inner.url ?? "");
            const shortUrl =
                url.length > 48 ? `${url.slice(0, 45)}…` : url;
            const label = `${method} ${shortUrl}`;
            const nameKey = normRequestName(inner.request_name);
            const id = nextId(state);
            state.openHttpStack.push({ rowId: id, nameKey });
            state.rows.push({
                kind: "span",
                id,
                variant: "HttpRequest",
                label,
                startMs: tMs,
                endMs: null,
                status: "running",
                rawStart: data,
            });
            break;
        }
        case "HttpResponseCompleted": {
            const status = Number(inner.status ?? 0);
            const httpOk =
                status === 0 || (status >= 200 && status < 400);
            closeHttpSpan(
                state,
                normRequestName(inner.request_name),
                tMs,
                data,
                httpOk,
            );
            break;
        }
        case "ScriptStarted": {
            bumpElapsed(state, tMs);
            const script = String(inner.script ?? "script");
            const id = nextId(state);
            state.openScriptRowId = id;
            state.rows.push({
                kind: "span",
                id,
                variant: "Script",
                label: script.split("/").pop() ?? script,
                startMs: tMs,
                endMs: null,
                status: "running",
                rawStart: data,
            });
            break;
        }
        case "ScriptFinished": {
            const success = Boolean(inner.success);
            closeScriptSpan(state, tMs, data, success);
            break;
        }
        case "CheckpointWaiting": {
            bumpElapsed(state, tMs);
            const cpId = String(inner.checkpoint_id ?? "");
            const msg = String(inner.message ?? "checkpoint");
            const id = nextId(state);
            if (cpId) state.openCheckpoints.set(cpId, id);
            state.rows.push({
                kind: "span",
                id,
                variant: "Checkpoint",
                label: msg.length > 56 ? `${msg.slice(0, 53)}…` : msg,
                startMs: tMs,
                endMs: null,
                status: "running",
                rawStart: data,
            });
            break;
        }
        case "CheckpointResumed": {
            const cpId = String(inner.checkpoint_id ?? "");
            closeCheckpointSpan(state, cpId, tMs, data);
            break;
        }
        case "Log": {
            const msg = String(inner.message ?? "");
            pushInstant(state, variant, msg, tMs, data);
            break;
        }
        case "RuntimeVariablesInitialized": {
            const entries = inner.entries as unknown[] | undefined;
            const n = entries?.length ?? 0;
            pushInstant(
                state,
                variant,
                `Runtime env (${n} variable${n === 1 ? "" : "s"})`,
                tMs,
                data,
            );
            break;
        }
        case "RuntimeVariablePushed": {
            const key = String(inner.key ?? "?");
            pushInstant(state, variant, `Set ${key}`, tMs, data);
            break;
        }
        case "AssertCalled": {
            const passed = Boolean(inner.passed);
            const msg = String(inner.message ?? "");
            pushInstant(
                state,
                variant,
                passed ? `Assert ok${msg ? `: ${msg}` : ""}` : `Assert failed: ${msg}`,
                tMs,
                data,
            );
            break;
        }
        case "NewStepEncountered": {
            const name = String(inner.name ?? "step");
            pushInstant(state, variant, name, tMs, data);
            break;
        }
        case "FileLoaded": {
            const path = String(inner.path ?? "");
            const base = path.split("/").pop() ?? path;
            pushInstant(state, variant, `Load ${base}`, tMs, data);
            break;
        }
        case "Error": {
            const msg = String(inner.message ?? "Error");
            pushInstant(
                state,
                variant,
                msg.length > 72 ? `${msg.slice(0, 69)}…` : msg,
                tMs,
                data,
            );
            break;
        }
        default: {
            pushInstant(state, variant, variant, tMs, data);
        }
    }
}

/** Close any spans still open after the run ends (WebSocket finished). */
export function flushOpenTimelineSpans(
    state: TimelineReducerState,
    runOk: boolean,
): void {
    const endMs = state.lastElapsedMs;
    const status: TimelineSpanStatus = runOk ? "interrupted" : "error";

    for (const { rowId } of [...state.openHttpStack]) {
        const row = state.rows.find((r) => r.id === rowId) as
            | TimelineRowSpan
            | undefined;
        if (row?.kind === "span" && row.endMs == null) {
            row.endMs = endMs;
            row.status = status;
        }
    }
    state.openHttpStack = [];

    if (state.openScriptRowId) {
        const row = state.rows.find((r) => r.id === state.openScriptRowId) as
            | TimelineRowSpan
            | undefined;
        if (row?.kind === "span" && row.endMs == null) {
            row.endMs = endMs;
            row.status = status;
        }
        state.openScriptRowId = null;
    }

    if (state.openSessionRowId) {
        const row = state.rows.find((r) => r.id === state.openSessionRowId) as
            | TimelineRowSpan
            | undefined;
        if (row?.kind === "span" && row.endMs == null) {
            row.endMs = endMs;
            row.status = status;
        }
        state.openSessionRowId = null;
    }

    for (const rowId of state.openCheckpoints.values()) {
        const row = state.rows.find((r) => r.id === rowId) as
            | TimelineRowSpan
            | undefined;
        if (row?.kind === "span" && row.endMs == null) {
            row.endMs = endMs;
            row.status = status;
        }
    }
    state.openCheckpoints.clear();
}
