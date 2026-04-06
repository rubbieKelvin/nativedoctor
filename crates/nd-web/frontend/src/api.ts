const API = "/api";

export interface WorkspaceSnapshot {
  roots: { index: number; path: string; label: string }[];
  requests: GroupedFiles[];
  scripts: GroupedFiles[];
  skipped_requests: { path: string; message: string }[];
}

export interface GroupedFiles {
  root_index: number;
  root_label: string;
  entries: { path: string; name: string }[];
}

export async function fetchWorkspace(): Promise<WorkspaceSnapshot> {
  const r = await fetch(`${API}/workspace`);
  if (!r.ok) throw new Error(await r.text());
  return r.json();
}

export interface RuntimeEnvEntry {
  key: string;
  value: string;
}

export async function fetchRuntimeEnv(): Promise<{
  entries: RuntimeEnvEntry[];
}> {
  const r = await fetch(`${API}/runtime-env`);
  if (!r.ok) throw new Error(await r.text());
  return r.json();
}

export async function fetchFile(path: string): Promise<string> {
  const q = new URLSearchParams({ path });
  const r = await fetch(`${API}/file?${q}`);
  if (!r.ok) throw new Error(await r.text());
  return r.text();
}

export interface SendRequestPayload {
  source_path: string;
  document?: unknown;
  overrides?: Record<string, string>;
}

export interface ExecutionResultDto {
  status: number;
  duration_ms: number;
  final_url: string;
  method: string;
  request_name: string | null;
  headers: [string, string][];
  body_text: string | null;
  body_base64: string | null;
  body_utf8: boolean;
}

export interface HttpSendResponse {
  ok: boolean;
  error?: string;
  result?: ExecutionResultDto;
}

export async function sendHttp(
  body: SendRequestPayload,
): Promise<HttpSendResponse> {
  const r = await fetch(`${API}/requests/send`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
  const j = (await r.json()) as HttpSendResponse & { error?: string };
  if (!r.ok) {
    throw new Error(j.error ?? r.statusText);
  }
  return j;
}

export interface ScriptRunResponse {
  ok: boolean;
  error?: string;
  logs: { level: string; message: string; elapsed_ms: number }[];
}

export async function runScript(path: string): Promise<ScriptRunResponse> {
  const r = await fetch(`${API}/scripts/run`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify({ path }),
  });
  return r.json() as Promise<ScriptRunResponse>;
}

function wsUrl(): string {
  const proto = window.location.protocol === "https:" ? "wss:" : "ws:";
  return `${proto}//${window.location.host}${API}/ws`;
}

/** Final message after streamed session events (matches server `RunComplete`). */
export interface RunCompleteMessage {
  kind: "run_complete";
  ok: boolean;
  error?: string;
  result?: ExecutionResultDto;
}

/**
 * Opens `/api/ws`, sends one JSON command (`type`: `run_request` | `run_script`), streams event JSON until `run_complete`.
 */
export function runSessionCommand(
  cmd: Record<string, unknown>,
  options?: { onEvent?: (data: unknown) => void },
): Promise<RunCompleteMessage> {
  return new Promise((resolve, reject) => {
    const ws = new WebSocket(wsUrl());
    let settled = false;

    const finish = (fn: () => void) => {
      if (settled) return;
      settled = true;
      fn();
      ws.close();
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
      options?.onEvent?.(data);
    };

    ws.onerror = () => {
      finish(() => reject(new Error("WebSocket connection failed")));
    };

    ws.onclose = () => {
      if (!settled) {
        finish(() => reject(new Error("WebSocket closed before run_complete")));
      }
    };
  });
}
