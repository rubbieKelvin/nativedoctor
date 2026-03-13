export type { KeyValuePair as KeyValue } from "@/shared/types/resources";

export interface HttpResponse {
  status: number;
  headers: [string, string][];
  body: string;
  durationMs: number;
}
