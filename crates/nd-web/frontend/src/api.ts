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

import {
  startSessionRun,
  type RunCompleteMessage as RunCompleteMessageBase,
  type SessionRun,
  type SessionEventListener,
} from "@/session/sessionRun";

export type { SessionRun, SessionEventListener };
export { startSessionRun };

/** WebSocket completion payload; `result` is an [`ExecutionResultDto`] when present. */
export type RunCompleteMessage = RunCompleteMessageBase & {
  result?: ExecutionResultDto;
};

/**
 * Opens `/api/ws`, sends one JSON command (`type`: `run_request` | `run_script`), streams event JSON until `run_complete`.
 * Prefer `startSessionRun` from `@/session/sessionRun` when you need multiple subscribers.
 */
export function runSessionCommand(
  cmd: Record<string, unknown>,
  options?: { onEvent?: (data: unknown) => void },
): Promise<RunCompleteMessage> {
  const run = startSessionRun(cmd);
  const off = options?.onEvent ? run.subscribe(options.onEvent) : () => {};
  return run.completed.finally(off) as Promise<RunCompleteMessage>;
}
