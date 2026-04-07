/** Final message after streamed session events (matches server `RunComplete`). */
export interface RunCompleteMessage {
    kind: "run_complete";
    ok: boolean;
    error?: string;
    /** Typed as DTO at call sites (see `@/api` `ExecutionResultDto`). */
    result?: unknown;
}

export type SessionEventListener = (event: unknown) => void;

/**
 * One WebSocket session: stream JSON events until `run_complete`, then resolve.
 * Subscribe to fan out each event to feature handlers (logs, env, future timeline, etc.).
 */
export interface SessionRun {
    subscribe(listener: SessionEventListener): () => void;
    readonly completed: Promise<RunCompleteMessage>;
}

function wsUrl(): string {
    const proto = window.location.protocol === "https:" ? "wss:" : "ws:";
    return `${proto}//${window.location.host}/api/ws`;
}

export function startSessionRun(cmd: Record<string, unknown>): SessionRun {
    const listeners = new Set<SessionEventListener>();

    const completed = new Promise<RunCompleteMessage>((resolve, reject) => {
        const ws = new WebSocket(wsUrl());
        let settled = false;

        const finish = (fn: () => void) => {
            if (settled) return;
            settled = true;
            fn();
            ws.close();
        };

        const notify = (data: unknown) => {
            for (const l of listeners) {
                try {
                    l(data);
                } catch {
                    /* isolate subscriber failures */
                }
            }
        };

        ws.onopen = () => {
            ws.send(JSON.stringify(cmd));
        };

        ws.onmessage = (ev) => {
            let data: unknown;
            try {
                data = JSON.parse(ev.data as string);
            } catch {
                finish(() => reject(new Error("Invalid JSON from server")));
                return;
            }
            const o = data as { kind?: string; message?: string };
            if (o.kind === "error") {
                finish(() => reject(new Error(o.message ?? "error")));
                return;
            }
            if (o.kind === "run_complete") {
                finish(() => resolve(data as RunCompleteMessage));
                return;
            }
            notify(data);
        };

        ws.onerror = () => {
            finish(() => reject(new Error("WebSocket connection failed")));
        };

        ws.onclose = () => {
            if (!settled) {
                finish(() =>
                    reject(new Error("WebSocket closed before run_complete")),
                );
            }
        };
    });

    completed.finally(() => {
        listeners.clear();
    });

    return {
        subscribe(listener: SessionEventListener) {
            listeners.add(listener);
            return () => {
                listeners.delete(listener);
            };
        },
        completed,
    };
}
