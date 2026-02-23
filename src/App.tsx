import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Play, Plus, Trash2, Clock, CheckCircle, AlertCircle } from "lucide-react";
import { Button } from "./components/ui/button";
import { Input } from "./components/ui/input";
import { Select } from "./components/ui/select";
import { Textarea } from "./components/ui/textarea";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "./components/ui/tabs";
import { Table, TableHeader, TableBody, TableHead, TableRow, TableCell } from "./components/ui/table";
import { cn } from "./lib/utils";

interface KeyValue {
  key: string;
  value: string;
}

interface SendRequestPayload {
  method: string;
  url: string;
  query: [string, string][];
  headers: [string, string][];
  body: string | null;
}

interface HttpResponsePayload {
  status_code: number;
  status_text: string;
  headers: [string, string][];
  body: string;
  time_ms: number;
  error: string | null;
}

const METHODS = [
  { label: "GET", value: "GET" },
  { label: "POST", value: "POST" },
  { label: "PUT", value: "PUT" },
  { label: "DELETE", value: "DELETE" },
];

function App() {
  const [method, setMethod] = useState("GET");
  const [url, setUrl] = useState("https://jsonplaceholder.typicode.com/todos/1");
  const [queryParams, setQueryParams] = useState<KeyValue[]>([{ key: "", value: "" }]);
  const [headers, setHeaders] = useState<KeyValue[]>([
    { key: "Content-Type", value: "application/json" },
    { key: "", value: "" },
  ]);
  const [body, setBody] = useState("");
  const [loading, setLoading] = useState(false);
  const [response, setResponse] = useState<HttpResponsePayload | null>(null);

  const addQueryParam = () => setQueryParams([...queryParams, { key: "", value: "" }]);
  const removeQueryParam = (index: number) => setQueryParams(queryParams.filter((_, i) => i !== index));
  const updateQueryParam = (index: number, field: keyof KeyValue, value: string) => {
    const newParams = [...queryParams];
    newParams[index][field] = value;
    setQueryParams(newParams);
  };

  const addHeader = () => setHeaders([...headers, { key: "", value: "" }]);
  const removeHeader = (index: number) => setHeaders(headers.filter((_, i) => i !== index));
  const updateHeader = (index: number, field: keyof KeyValue, value: string) => {
    const newHeaders = [...headers];
    newHeaders[index][field] = value;
    setHeaders(newHeaders);
  };

  const sendRequest = async () => {
    if (!url) return;
    setLoading(true);
    setResponse(null);

    try {
      const payload: SendRequestPayload = {
        method,
        url,
        query: queryParams.filter(p => p.key).map(p => [p.key, p.value]),
        headers: headers.filter(h => h.key).map(h => [h.key, h.value]),
        body: body || null,
      };

      const result = await invoke<HttpResponsePayload>("send_http_request", { payload });
      setResponse(result);
    } catch (err) {
      console.error(err);
      setResponse({
        status_code: 0,
        status_text: "Error",
        headers: [],
        body: String(err),
        time_ms: 0,
        error: String(err),
      });
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="flex flex-col h-screen bg-white dark:bg-slate-950 text-slate-900 dark:text-slate-100 font-sans">
      {/* Top Bar */}
      <div className="flex items-center p-4 border-b border-slate-200 dark:border-slate-800 space-x-2">
        <div className="w-32">
          <Select
            options={METHODS}
            value={method}
            onChange={(e) => setMethod(e.target.value)}
          />
        </div>
        <Input
          placeholder="https://api.example.com"
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          className="flex-1"
        />
        <Button
          onClick={sendRequest}
          disabled={loading}
          className="w-24"
        >
          {loading ? "Sending..." : (
            <>
              <Play className="w-4 h-4 mr-2" />
              Send
            </>
          )}
        </Button>
      </div>

      {/* Main Content */}
      <div className="flex-1 flex flex-col overflow-hidden">
        <div className="flex-1 overflow-auto p-4">
          <Tabs defaultValue="params">
            <TabsList className="mb-4">
              <TabsTrigger value="params">Query Params</TabsTrigger>
              <TabsTrigger value="headers">Headers</TabsTrigger>
              <TabsTrigger value="body">Body</TabsTrigger>
            </TabsList>

            <TabsContent value="params">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Key</TableHead>
                    <TableHead>Value</TableHead>
                    <TableHead className="w-10"></TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {queryParams.map((param, i) => (
                    <TableRow key={i}>
                      <TableCell className="p-1">
                        <Input
                          value={param.key}
                          onChange={(e) => updateQueryParam(i, "key", e.target.value)}
                          placeholder="Key"
                          className="h-8 border-none shadow-none focus:ring-1"
                        />
                      </TableCell>
                      <TableCell className="p-1">
                        <Input
                          value={param.value}
                          onChange={(e) => updateQueryParam(i, "value", e.target.value)}
                          placeholder="Value"
                          className="h-8 border-none shadow-none focus:ring-1"
                        />
                      </TableCell>
                      <TableCell className="p-1">
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => removeQueryParam(i)}
                          className="text-slate-400 hover:text-red-500"
                        >
                          <Trash2 className="w-4 h-4" />
                        </Button>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
              <Button variant="outline" size="sm" onClick={addQueryParam} className="mt-2">
                <Plus className="w-4 h-4 mr-2" />
                Add Param
              </Button>
            </TabsContent>

            <TabsContent value="headers">
              <Table>
                <TableHeader>
                  <TableRow>
                    <TableHead>Key</TableHead>
                    <TableHead>Value</TableHead>
                    <TableHead className="w-10"></TableHead>
                  </TableRow>
                </TableHeader>
                <TableBody>
                  {headers.map((header, i) => (
                    <TableRow key={i}>
                      <TableCell className="p-1">
                        <Input
                          value={header.key}
                          onChange={(e) => updateHeader(i, "key", e.target.value)}
                          placeholder="Key"
                          className="h-8 border-none shadow-none focus:ring-1"
                        />
                      </TableCell>
                      <TableCell className="p-1">
                        <Input
                          value={header.value}
                          onChange={(e) => updateHeader(i, "value", e.target.value)}
                          placeholder="Value"
                          className="h-8 border-none shadow-none focus:ring-1"
                        />
                      </TableCell>
                      <TableCell className="p-1">
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={() => removeHeader(i)}
                          className="text-slate-400 hover:text-red-500"
                        >
                          <Trash2 className="w-4 h-4" />
                        </Button>
                      </TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
              <Button variant="outline" size="sm" onClick={addHeader} className="mt-2">
                <Plus className="w-4 h-4 mr-2" />
                Add Header
              </Button>
            </TabsContent>

            <TabsContent value="body">
              <Textarea
                placeholder='{ "key": "value" }'
                value={body}
                onChange={(e) => setBody(e.target.value)}
                className="min-h-[200px] font-mono"
              />
            </TabsContent>
          </Tabs>
        </div>

        {/* Response Panel */}
        <div className="h-1/3 border-t border-slate-200 dark:border-slate-800 flex flex-col bg-slate-50 dark:bg-slate-900 overflow-hidden">
          {response ? (
            <>
              <div className="flex items-center justify-between p-2 px-4 border-b border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-950">
                <div className="flex items-center space-x-4">
                  <div className={cn(
                    "flex items-center font-bold",
                    response.status_code >= 200 && response.status_code < 300 ? "text-green-600" : "text-red-600"
                  )}>
                    {response.status_code >= 200 && response.status_code < 300 ? (
                      <CheckCircle className="w-4 h-4 mr-1" />
                    ) : (
                      <AlertCircle className="w-4 h-4 mr-1" />
                    )}
                    {response.status_code} {response.status_text}
                  </div>
                  <div className="flex items-center text-slate-500 text-sm">
                    <Clock className="w-4 h-4 mr-1" />
                    {response.time_ms} ms
                  </div>
                </div>
              </div>
              <div className="flex-1 overflow-hidden flex">
                <div className="w-1/3 border-r border-slate-200 dark:border-slate-800 overflow-auto p-2">
                  <div className="text-xs font-bold text-slate-400 uppercase mb-2">Headers</div>
                  {response.headers.map(([k, v], i) => (
                    <div key={i} className="text-xs mb-1">
                      <span className="font-semibold text-slate-600 dark:text-slate-400">{k}:</span> {v}
                    </div>
                  ))}
                </div>
                <div className="flex-1 overflow-auto p-2">
                  <div className="text-xs font-bold text-slate-400 uppercase mb-2">Body</div>
                  <pre className="text-xs font-mono whitespace-pre-wrap break-all">
                    {response.body}
                  </pre>
                </div>
              </div>
            </>
          ) : (
            <div className="flex-1 flex items-center justify-center text-slate-400 italic">
              Send a request to see the response
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default App;
