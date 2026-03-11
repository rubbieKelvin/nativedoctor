export interface KeyValue {
  key: string
  value: string
}

export interface HttpResponse {
  status: number
  headers: [string, string][]
  body: string
  duration_ms: number
}
