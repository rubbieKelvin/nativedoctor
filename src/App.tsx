import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import {
  Plus,
  Trash2,
  Clock,
  History,
  Globe,
  Settings,
  Copy,
  Send,
  Shield,
  Search,
  LayoutGrid,
  Activity,
  ChevronRight,
} from "lucide-react";
import { Button } from "./components/ui/button";
import { Input } from "./components/ui/input";
import { Select } from "./components/ui/select";
import { Textarea } from "./components/ui/textarea";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "./components/ui/tabs";
import {
  Table,
  TableBody,
  TableRow,
  TableCell,
} from "./components/ui/table";
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

interface HistoryItem {
  id: string;
  method: string;
  url: string;
  timestamp: number;
  status?: number;
}

const METHODS = [
  { label: "GET", value: "GET" },
  { label: "POST", value: "POST" },
  { label: "PUT", value: "PUT" },
  { label: "DELETE", value: "DELETE" },
];

function App() {
  const [method, setMethod] = useState("GET");
  const [url, setUrl] = useState(
    "https://jsonplaceholder.typicode.com/todos/1",
  );
  const [queryParams, setQueryParams] = useState<KeyValue[]>([
    { key: "", value: "" },
  ]);
  const [headers, setHeaders] = useState<KeyValue[]>([
    { key: "Content-Type", value: "application/json" },
    { key: "", value: "" },
  ]);
  const [body, setBody] = useState("");
  const [loading, setLoading] = useState(false);
  const [response, setResponse] = useState<HttpResponsePayload | null>(null);
  const [history, setHistory] = useState<HistoryItem[]>([]);
  const [activeTab, setActiveTab] = useState("history");

  const addQueryParam = () =>
    setQueryParams([...queryParams, { key: "", value: "" }]);
  const removeQueryParam = (index: number) =>
    setQueryParams(queryParams.filter((_, i) => i !== index));
  const updateQueryParam = (
    index: number,
    field: keyof KeyValue,
    value: string,
  ) => {
    const newParams = [...queryParams];
    newParams[index][field] = value;
    setQueryParams(newParams);
  };

  const addHeader = () => setHeaders([...headers, { key: "", value: "" }]);
  const removeHeader = (index: number) =>
    setHeaders(headers.filter((_, i) => i !== index));
  const updateHeader = (
    index: number,
    field: keyof KeyValue,
    value: string,
  ) => {
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
        query: queryParams.filter((p) => p.key).map((p) => [p.key, p.value]),
        headers: headers.filter((h) => h.key).map((h) => [h.key, h.value]),
        body: body || null,
      };

      const result = await invoke<HttpResponsePayload>("send_http_request", {
        payload,
      });
      setResponse(result);

      const newHistoryItem: HistoryItem = {
        id: Math.random().toString(36).substr(2, 9),
        method,
        url,
        timestamp: Date.now(),
        status: result.status_code,
      };
      setHistory([newHistoryItem, ...history.slice(0, 19)]);
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

  const loadFromHistory = (item: HistoryItem) => {
    setMethod(item.method);
    setUrl(item.url);
  };

  return (
    <div className="flex h-screen bg-background text-foreground overflow-hidden font-sans selection:bg-blue-100 selection:text-blue-700">
      {/* Left Rail - App Navigation */}
      <div className="w-14 flex flex-col items-center py-4 bg-white border-r border-border space-y-4">
        <div className="w-8 h-8 rounded-lg bg-blue-600 flex items-center justify-center text-white mb-4 shadow-sm shadow-blue-500/20">
          <Globe className="w-5 h-5" />
        </div>
        
        <div className="flex-1 w-full flex flex-col items-center space-y-1">
          <button 
            onClick={() => setActiveTab('history')}
            className={cn(
              "p-2 rounded-lg transition-colors",
              activeTab === 'history' ? "text-blue-600 bg-blue-50" : "text-slate-400 hover:text-slate-600 hover:bg-slate-50"
            )}
          >
            <Activity className="w-5 h-5" />
          </button>
          <button 
            onClick={() => setActiveTab('collections')}
            className={cn(
              "p-2 rounded-lg transition-colors",
              activeTab === 'collections' ? "text-blue-600 bg-blue-50" : "text-slate-400 hover:text-slate-600 hover:bg-slate-50"
            )}
          >
            <LayoutGrid className="w-5 h-5" />
          </button>
        </div>

        <div className="w-full flex flex-col items-center space-y-2">
          <button className="p-2 text-slate-400 hover:text-slate-600 hover:bg-slate-50 rounded-lg transition-colors">
            <Settings className="w-5 h-5" />
          </button>
        </div>
      </div>

      {/* Secondary Sidebar - List Pane */}
      <div className="w-72 flex flex-col bg-white/50 border-r border-border overflow-hidden">
        <div className="p-4 flex flex-col space-y-3">
          <div className="flex items-center justify-between">
            <h2 className="text-xs font-bold text-slate-500 uppercase tracking-widest px-1">
              {activeTab === 'history' ? 'History' : 'Collections'}
            </h2>
            <Button variant="ghost" size="icon" className="h-6 w-6 rounded-md">
              <Plus className="w-3.5 h-3.5" />
            </Button>
          </div>
          <div className="relative">
            <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 w-3.5 h-3.5 text-slate-400" />
            <Input 
              placeholder="Search..." 
              className="h-8 pl-8 text-xs bg-white border-slate-100 rounded-lg"
            />
          </div>
        </div>

        <div className="flex-1 overflow-auto px-2 pb-4">
          <div className="space-y-0.5">
            {history.length > 0 ? (
              history.map((item) => (
                <button
                  key={item.id}
                  onClick={() => loadFromHistory(item)}
                  className="w-full flex items-center space-x-3 px-3 py-2.5 rounded-xl hover:bg-white transition-all group border border-transparent hover:border-slate-100 hover:shadow-sm"
                >
                  <div className={cn(
                    "text-[10px] font-bold px-1.5 py-0.5 rounded-md w-10 text-center flex-shrink-0",
                    item.method === "GET"
                      ? "bg-blue-100 text-blue-600"
                      : item.method === "POST"
                        ? "bg-emerald-100 text-emerald-600"
                        : "bg-amber-100 text-amber-600",
                  )}>
                    {item.method}
                  </div>
                  <div className="flex-1 min-w-0">
                    <div className="text-xs truncate font-medium text-slate-700">
                      {item.url}
                    </div>
                    <div className="text-[10px] text-slate-400 mt-0.5">
                      {new Date(item.timestamp).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })}
                    </div>
                  </div>
                  <ChevronRight className="w-3 h-3 text-slate-300 opacity-0 group-hover:opacity-100 transition-opacity" />
                </button>
              ))
            ) : (
              <div className="px-4 py-12 text-center">
                <div className="w-10 h-10 bg-slate-50 rounded-full flex items-center justify-center mx-auto mb-3">
                  <History className="w-5 h-5 text-slate-300" />
                </div>
                <div className="text-xs text-slate-400 font-medium">No activity yet</div>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Main Content Area */}
      <div className="flex-1 flex flex-col min-w-0 bg-background overflow-hidden">
        {/* Address Bar Area */}
        <div className="px-6 py-4 flex flex-col space-y-4 border-b border-border bg-white">
          <div className="flex items-center space-x-2">
            <div className="flex-1 flex items-center bg-slate-50 border border-slate-100 rounded-xl p-1 overflow-hidden">
              <div className="w-24">
                <select
                  value={method}
                  onChange={(e) => setMethod(e.target.value)}
                  className="w-full h-8 px-2 bg-transparent border-none text-xs font-bold focus:ring-0 appearance-none"
                >
                  {METHODS.map(m => (
                    <option key={m.value} value={m.value}>{m.label}</option>
                  ))}
                </select>
              </div>
              <div className="w-[1px] h-4 bg-slate-200 mx-1" />
              <Input
                placeholder="https://api.example.com/v1/users"
                value={url}
                onChange={(e) => setUrl(e.target.value)}
                className="flex-1 border-none bg-transparent shadow-none ring-0 focus:ring-0 h-8 text-xs"
              />
            </div>
            <Button
              onClick={sendRequest}
              disabled={loading}
              className="h-10 px-6 rounded-xl shadow-sm bg-blue-600 hover:bg-blue-700 text-white transition-all active:scale-[0.98] font-semibold text-xs"
            >
              {loading ? (
                <div className="animate-spin rounded-full h-3.5 w-3.5 border-2 border-white/20 border-t-white" />
              ) : (
                "Send"
              )}
            </Button>
          </div>
        </div>

        {/* Workspace - Split Panes */}
        <div className="flex-1 flex flex-col overflow-hidden p-6">
          <div className="flex-1 grid grid-rows-2 gap-6 min-h-0">
            {/* Request Pane */}
            <div className="flex flex-col min-h-0 bg-white rounded-2xl border border-border shadow-sm overflow-hidden">
              <div className="px-4 py-2 flex items-center border-b border-slate-50 bg-slate-50/30">
                <Tabs defaultValue="params" className="w-full">
                  <div className="flex items-center justify-between">
                    <TabsList className="h-8 bg-transparent p-0 border-none space-x-1">
                      <TabsTrigger value="params" className="h-7 text-[10px] uppercase font-bold tracking-wider data-[selected]:bg-white px-3">Params</TabsTrigger>
                      <TabsTrigger value="headers" className="h-7 text-[10px] uppercase font-bold tracking-wider data-[selected]:bg-white px-3">Headers</TabsTrigger>
                      <TabsTrigger value="body" className="h-7 text-[10px] uppercase font-bold tracking-wider data-[selected]:bg-white px-3">Body</TabsTrigger>
                      <TabsTrigger value="auth" className="h-7 text-[10px] uppercase font-bold tracking-wider data-[selected]:bg-white px-3">Auth</TabsTrigger>
                    </TabsList>
                  </div>
                  
                  <TabsContent value="params" className="mt-0 flex-1 overflow-hidden h-full">
                    <div className="p-4 overflow-auto max-h-[160px]">
                      <Table>
                        <TableBody>
                          {queryParams.map((param, i) => (
                            <TableRow key={i} className="border-none hover:bg-transparent">
                              <TableCell className="p-1 pl-0">
                                <Input
                                  value={param.key}
                                  onChange={(e) => updateQueryParam(i, "key", e.target.value)}
                                  placeholder="Parameter"
                                  className="h-8 text-xs rounded-lg border-slate-100 shadow-none bg-slate-50/50"
                                />
                              </TableCell>
                              <TableCell className="p-1">
                                <Input
                                  value={param.value}
                                  onChange={(e) => updateQueryParam(i, "value", e.target.value)}
                                  placeholder="Value"
                                  className="h-8 text-xs rounded-lg border-slate-100 shadow-none bg-slate-50/50"
                                />
                              </TableCell>
                              <TableCell className="p-1 pr-0 w-8 text-right">
                                <Button
                                  variant="ghost"
                                  size="icon"
                                  onClick={() => removeQueryParam(i)}
                                  className="h-8 w-8 text-slate-300 hover:text-red-500 rounded-lg"
                                >
                                  <Trash2 className="w-3.5 h-3.5" />
                                </Button>
                              </TableCell>
                            </TableRow>
                          ))}
                          <TableRow className="border-none hover:bg-transparent">
                            <TableCell colSpan={3} className="p-1 pl-0">
                              <Button
                                variant="ghost"
                                size="sm"
                                onClick={addQueryParam}
                                className="h-8 text-[10px] font-bold uppercase tracking-tight text-blue-600 hover:bg-blue-50 rounded-lg px-3"
                              >
                                <Plus className="w-3 h-3 mr-1.5" />
                                Add Parameter
                              </Button>
                            </TableCell>
                          </TableRow>
                        </TableBody>
                      </Table>
                    </div>
                  </TabsContent>

                  <TabsContent value="headers" className="mt-0">
                    <div className="p-4 overflow-auto max-h-[160px]">
                      <Table>
                        <TableBody>
                          {headers.map((header, i) => (
                            <TableRow key={i} className="border-none hover:bg-transparent">
                              <TableCell className="p-1 pl-0">
                                <Input
                                  value={header.key}
                                  onChange={(e) => updateHeader(i, "key", e.target.value)}
                                  placeholder="Header Name"
                                  className="h-8 text-xs rounded-lg border-slate-100 shadow-none bg-slate-50/50"
                                />
                              </TableCell>
                              <TableCell className="p-1">
                                <Input
                                  value={header.value}
                                  onChange={(e) => updateHeader(i, "value", e.target.value)}
                                  placeholder="Value"
                                  className="h-8 text-xs rounded-lg border-slate-100 shadow-none bg-slate-50/50"
                                />
                              </TableCell>
                              <TableCell className="p-1 pr-0 w-8 text-right">
                                <Button
                                  variant="ghost"
                                  size="icon"
                                  onClick={() => removeHeader(i)}
                                  className="h-8 w-8 text-slate-300 hover:text-red-500 rounded-lg"
                                >
                                  <Trash2 className="w-3.5 h-3.5" />
                                </Button>
                              </TableCell>
                            </TableRow>
                          ))}
                          <TableRow className="border-none hover:bg-transparent">
                            <TableCell colSpan={3} className="p-1 pl-0">
                              <Button
                                variant="ghost"
                                size="sm"
                                onClick={addHeader}
                                className="h-8 text-[10px] font-bold uppercase tracking-tight text-blue-600 hover:bg-blue-50 rounded-lg px-3"
                              >
                                <Plus className="w-3 h-3 mr-1.5" />
                                Add Header
                              </Button>
                            </TableCell>
                          </TableRow>
                        </TableBody>
                      </Table>
                    </div>
                  </TabsContent>

                  <TabsContent value="body" className="mt-0">
                    <div className="p-4">
                      <Textarea
                        placeholder='{ "key": "value" }'
                        value={body}
                        onChange={(e) => setBody(e.target.value)}
                        className="min-h-[120px] font-mono text-xs rounded-xl border-slate-100 bg-slate-50/50 resize-none"
                      />
                    </div>
                  </TabsContent>
                  
                  <TabsContent value="auth" className="mt-0">
                    <div className="p-8 text-center">
                      <Shield className="w-8 h-8 text-slate-200 mx-auto mb-2" />
                      <div className="text-xs text-slate-400">Authentication settings coming soon</div>
                    </div>
                  </TabsContent>
                </Tabs>
              </div>
            </div>

            {/* Response Pane */}
            <div className="flex flex-col min-h-0 bg-white rounded-2xl border border-border shadow-sm overflow-hidden">
              <div className="px-4 py-3 flex items-center justify-between border-b border-slate-50 bg-slate-50/30">
                <div className="flex items-center space-x-4">
                  <span className="text-[10px] font-bold text-slate-400 uppercase tracking-widest">
                    Response
                  </span>
                  {response && (
                    <div className="flex items-center space-x-2">
                      <div className={cn(
                        "px-2 py-0.5 rounded-md text-[10px] font-bold",
                        response.status_code >= 200 && response.status_code < 300
                          ? "bg-emerald-50 text-emerald-600 border border-emerald-100"
                          : "bg-red-50 text-red-600 border border-red-100",
                      )}>
                        {response.status_code} {response.status_text}
                      </div>
                      <div className="text-[10px] font-medium text-slate-400 flex items-center">
                        <Clock className="w-3 h-3 mr-1" />
                        {response.time_ms}ms
                      </div>
                    </div>
                  )}
                </div>
                {response && (
                  <Button
                    variant="ghost"
                    size="sm"
                    className="h-7 text-[10px] font-bold uppercase tracking-tight text-slate-400 hover:text-slate-600"
                    onClick={() => navigator.clipboard.writeText(response.body)}
                  >
                    <Copy className="w-3 h-3 mr-1.5" />
                    Copy
                  </Button>
                )}
              </div>

              <div className="flex-1 overflow-hidden">
                {response ? (
                  <div className="flex h-full overflow-hidden">
                    <div className="w-56 border-r border-slate-50 overflow-auto p-4 bg-slate-50/20">
                      <div className="text-[9px] font-bold text-slate-400 uppercase mb-4 tracking-widest">Headers</div>
                      <div className="space-y-4">
                        {response.headers.map(([k, v], i) => (
                          <div key={i} className="text-[10px] space-y-1">
                            <div className="font-bold text-slate-500">{k}</div>
                            <div className="text-slate-400 break-all leading-relaxed">{v}</div>
                          </div>
                        ))}
                      </div>
                    </div>
                    <div className="flex-1 overflow-auto p-6 bg-white">
                      <pre className="text-xs font-mono text-slate-700 whitespace-pre-wrap leading-relaxed">
                        {response.body}
                      </pre>
                    </div>
                  </div>
                ) : (
                  <div className="h-full flex flex-col items-center justify-center text-slate-300 space-y-4">
                    <div className="w-16 h-16 rounded-3xl bg-slate-50 flex items-center justify-center">
                      <Send className="w-7 h-7" />
                    </div>
                    <div className="flex flex-col items-center">
                      <div className="text-xs font-bold uppercase tracking-widest text-slate-400">Ready to Send</div>
                      <div className="text-[10px] text-slate-300 mt-1 italic">Response will appear here</div>
                    </div>
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
