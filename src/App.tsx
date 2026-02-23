import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { 
  Plus, Trash2, Clock, 
  History, Globe, Settings, Moon, Sun, Copy, Send,
  Shield
} from "lucide-react";
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
  const [url, setUrl] = useState("https://jsonplaceholder.typicode.com/todos/1");
  const [queryParams, setQueryParams] = useState<KeyValue[]>([{ key: "", value: "" }]);
  const [headers, setHeaders] = useState<KeyValue[]>([
    { key: "Content-Type", value: "application/json" },
    { key: "", value: "" },
  ]);
  const [body, setBody] = useState("");
  const [loading, setLoading] = useState(false);
  const [response, setResponse] = useState<HttpResponsePayload | null>(null);
  const [history, setHistory] = useState<HistoryItem[]>([]);
  const [darkMode, setDarkMode] = useState(false);

  useEffect(() => {
    const isDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
    setDarkMode(isDark);
    if (isDark) document.documentElement.classList.add("dark");
  }, []);

  const toggleDarkMode = () => {
    setDarkMode(!darkMode);
    document.documentElement.classList.toggle("dark");
  };

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
      
      // Add to history
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
    <div className="flex h-screen bg-slate-50 dark:bg-slate-950 text-slate-900 dark:text-slate-100 overflow-hidden font-sans">
      
      {/* Sidebar */}
      <div className="w-64 border-r border-slate-200 dark:border-slate-800 bg-white dark:bg-slate-900 flex flex-col shadow-sm">
        <div className="p-4 border-b border-slate-100 dark:border-slate-800 flex items-center justify-between">
          <div className="flex items-center space-x-2 font-bold text-blue-600">
            <Globe className="w-5 h-5" />
            <span>NativeDoctor</span>
          </div>
          <Button variant="ghost" size="icon" onClick={toggleDarkMode}>
            {darkMode ? <Sun className="w-4 h-4" /> : <Moon className="w-4 h-4" />}
          </Button>
        </div>
        
        <div className="flex-1 overflow-auto p-3 space-y-4">
          <div>
            <div className="flex items-center space-x-2 px-2 py-1.5 text-xs font-semibold text-slate-400 uppercase tracking-wider">
              <History className="w-3.5 h-3.5" />
              <span>Recent Requests</span>
            </div>
            <div className="mt-1 space-y-1">
              {history.length > 0 ? history.map((item) => (
                <button
                  key={item.id}
                  onClick={() => loadFromHistory(item)}
                  className="w-full flex items-center space-x-2 px-3 py-2 rounded-xl hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors text-left"
                >
                  <span className={cn(
                    "text-[10px] font-bold px-1.5 py-0.5 rounded-md w-12 text-center",
                    item.method === "GET" ? "bg-blue-100 text-blue-600 dark:bg-blue-900/30 dark:text-blue-400" :
                    item.method === "POST" ? "bg-emerald-100 text-emerald-600 dark:bg-emerald-900/30 dark:text-emerald-400" :
                    "bg-amber-100 text-amber-600 dark:bg-amber-900/30 dark:text-amber-400"
                  )}>
                    {item.method}
                  </span>
                  <div className="flex-1 min-w-0">
                    <div className="text-xs truncate font-medium">{item.url}</div>
                    <div className="text-[10px] text-slate-400">{new Date(item.timestamp).toLocaleTimeString()}</div>
                  </div>
                </button>
              )) : (
                <div className="px-3 py-4 text-xs text-slate-400 italic text-center">
                  No history yet
                </div>
              )}
            </div>
          </div>
        </div>

        <div className="p-3 border-t border-slate-100 dark:border-slate-800 space-y-1">
          <Button variant="ghost" className="w-full justify-start rounded-xl">
            <Settings className="w-4 h-4 mr-2" />
            Settings
          </Button>
          <Button variant="ghost" className="w-full justify-start rounded-xl">
            <Shield className="w-4 h-4 mr-2" />
            API Keys
          </Button>
        </div>
      </div>

      {/* Main Content */}
      <div className="flex-1 flex flex-col min-w-0 bg-slate-50 dark:bg-slate-950 overflow-hidden">
        
        {/* Header / Address Bar */}
        <div className="p-4 bg-white dark:bg-slate-900 border-b border-slate-200 dark:border-slate-800 flex items-center space-x-3 shadow-sm z-10">
          <div className="w-36">
            <Select
              options={METHODS}
              value={method}
              onChange={(e) => setMethod(e.target.value)}
              className="font-bold border-none bg-slate-50 dark:bg-slate-800"
            />
          </div>
          <Input
            placeholder="Enter request URL..."
            value={url}
            onChange={(e) => setUrl(e.target.value)}
            className="flex-1 border-none bg-slate-50 dark:bg-slate-800 shadow-none ring-0 focus:ring-2"
          />
          <Button
            onClick={sendRequest}
            disabled={loading}
            className="w-28 rounded-xl shadow-lg shadow-blue-500/20"
          >
            {loading ? (
              <div className="animate-spin rounded-full h-4 w-4 border-2 border-white/20 border-t-white" />
            ) : (
              <>
                <Send className="w-4 h-4 mr-2" />
                Send
              </>
            )}
          </Button>
        </div>

        {/* Workspace */}
        <div className="flex-1 flex flex-col overflow-hidden p-6 gap-6">
          
          {/* Top Panel: Request Configuration */}
          <div className="bg-white dark:bg-slate-900 rounded-2xl border border-slate-200 dark:border-slate-800 shadow-sm overflow-hidden flex flex-col h-[45%]">
            <div className="px-4 py-2 border-b border-slate-100 dark:border-slate-800 bg-slate-50/50 dark:bg-slate-800/30 flex items-center justify-between">
               <span className="text-xs font-bold text-slate-500 uppercase tracking-wider">Request</span>
            </div>
            <div className="flex-1 overflow-auto p-4">
              <Tabs defaultValue="params">
                <TabsList className="mb-4 bg-slate-50 dark:bg-slate-950 p-1 rounded-xl">
                  <TabsTrigger value="params" className="rounded-lg px-6">Params</TabsTrigger>
                  <TabsTrigger value="headers" className="rounded-lg px-6">Headers</TabsTrigger>
                  <TabsTrigger value="body" className="rounded-lg px-6">Body</TabsTrigger>
                </TabsList>

                <TabsContent value="params">
                  <Table>
                    <TableHeader>
                      <TableRow className="hover:bg-transparent">
                        <TableHead className="text-xs font-bold">Key</TableHead>
                        <TableHead className="text-xs font-bold">Value</TableHead>
                        <TableHead className="w-12"></TableHead>
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {queryParams.map((param, i) => (
                        <TableRow key={i} className="border-none">
                          <TableCell className="p-1">
                            <Input
                              value={param.key}
                              onChange={(e) => updateQueryParam(i, "key", e.target.value)}
                              placeholder="key"
                              className="h-9 rounded-lg border-slate-100 dark:border-slate-800 shadow-none"
                            />
                          </TableCell>
                          <TableCell className="p-1">
                            <Input
                              value={param.value}
                              onChange={(e) => updateQueryParam(i, "value", e.target.value)}
                              placeholder="value"
                              className="h-9 rounded-lg border-slate-100 dark:border-slate-800 shadow-none"
                            />
                          </TableCell>
                          <TableCell className="p-1">
                            <Button
                              variant="ghost"
                              size="icon"
                              onClick={() => removeQueryParam(i)}
                              className="h-9 w-9 text-slate-400 hover:text-red-500 hover:bg-red-50 dark:hover:bg-red-900/20"
                            >
                              <Trash2 className="w-4 h-4" />
                            </Button>
                          </TableCell>
                        </TableRow>
                      ))}
                    </TableBody>
                  </Table>
                  <Button variant="ghost" size="sm" onClick={addQueryParam} className="mt-3 text-blue-600 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded-lg">
                    <Plus className="w-4 h-4 mr-1" />
                    Add Parameter
                  </Button>
                </TabsContent>

                <TabsContent value="headers">
                  <Table>
                    <TableHeader>
                      <TableRow className="hover:bg-transparent">
                        <TableHead className="text-xs font-bold">Header Key</TableHead>
                        <TableHead className="text-xs font-bold">Header Value</TableHead>
                        <TableHead className="w-12"></TableHead>
                      </TableRow>
                    </TableHeader>
                    <TableBody>
                      {headers.map((header, i) => (
                        <TableRow key={i} className="border-none">
                          <TableCell className="p-1">
                            <Input
                              value={header.key}
                              onChange={(e) => updateHeader(i, "key", e.target.value)}
                              placeholder="Content-Type"
                              className="h-9 rounded-lg border-slate-100 dark:border-slate-800 shadow-none"
                            />
                          </TableCell>
                          <TableCell className="p-1">
                            <Input
                              value={header.value}
                              onChange={(e) => updateHeader(i, "value", e.target.value)}
                              placeholder="application/json"
                              className="h-9 rounded-lg border-slate-100 dark:border-slate-800 shadow-none"
                            />
                          </TableCell>
                          <TableCell className="p-1">
                            <Button
                              variant="ghost"
                              size="icon"
                              onClick={() => removeHeader(i)}
                              className="h-9 w-9 text-slate-400 hover:text-red-500 hover:bg-red-50 dark:hover:bg-red-900/20"
                            >
                              <Trash2 className="w-4 h-4" />
                            </Button>
                          </TableCell>
                        </TableRow>
                      ))}
                    </TableBody>
                  </Table>
                  <Button variant="ghost" size="sm" onClick={addHeader} className="mt-3 text-blue-600 hover:bg-blue-50 dark:hover:bg-blue-900/20 rounded-lg">
                    <Plus className="w-4 h-4 mr-1" />
                    Add Header
                  </Button>
                </TabsContent>

                <TabsContent value="body">
                  <Textarea
                    placeholder='{ "key": "value" }'
                    value={body}
                    onChange={(e) => setBody(e.target.value)}
                    className="min-h-[120px] font-mono text-xs rounded-xl border-slate-100 dark:border-slate-800 bg-slate-50/50 dark:bg-slate-950/50"
                  />
                </TabsContent>
              </Tabs>
            </div>
          </div>

          {/* Bottom Panel: Response */}
          <div className="bg-white dark:bg-slate-900 rounded-2xl border border-slate-200 dark:border-slate-800 shadow-sm overflow-hidden flex flex-col flex-1 min-h-0">
            <div className="px-4 py-2 border-b border-slate-100 dark:border-slate-800 bg-slate-50/50 dark:bg-slate-800/30 flex items-center justify-between">
               <span className="text-xs font-bold text-slate-500 uppercase tracking-wider">Response</span>
               {response && (
                 <div className="flex items-center space-x-3">
                    <div className={cn(
                      "px-2 py-0.5 rounded-full text-[10px] font-bold flex items-center",
                      response.status_code >= 200 && response.status_code < 300 
                        ? "bg-emerald-100 text-emerald-600 dark:bg-emerald-900/30 dark:text-emerald-400" 
                        : "bg-red-100 text-red-600 dark:bg-red-900/30 dark:text-red-400"
                    )}>
                      {response.status_code} {response.status_text}
                    </div>
                    <div className="flex items-center text-[10px] font-medium text-slate-400">
                      <Clock className="w-3 h-3 mr-1" />
                      {response.time_ms}ms
                    </div>
                 </div>
               )}
            </div>
            
            <div className="flex-1 overflow-hidden flex min-h-0">
              {response ? (
                <div className="flex-1 flex flex-col overflow-hidden">
                   <div className="flex-1 flex overflow-hidden">
                      <div className="w-48 border-r border-slate-100 dark:border-slate-800 overflow-auto p-4 bg-slate-50/30 dark:bg-slate-800/10">
                        <div className="text-[10px] font-bold text-slate-400 uppercase mb-3 tracking-widest">Headers</div>
                        <div className="space-y-3">
                          {response.headers.map(([k, v], i) => (
                            <div key={i} className="text-[10px] break-all">
                              <div className="font-bold text-slate-600 dark:text-slate-300 mb-0.5">{k}</div>
                              <div className="text-slate-400">{v}</div>
                            </div>
                          ))}
                        </div>
                      </div>
                      <div className="flex-1 flex flex-col min-w-0">
                        <div className="flex-1 overflow-auto p-4 bg-white dark:bg-slate-900">
                          <pre className="text-xs font-mono text-slate-700 dark:text-slate-300 whitespace-pre-wrap leading-relaxed">
                            {response.body}
                          </pre>
                        </div>
                        <div className="px-4 py-2 bg-slate-50 dark:bg-slate-800/50 border-t border-slate-100 dark:border-slate-800 flex justify-end">
                           <Button variant="ghost" size="sm" className="h-7 text-[10px] font-bold uppercase tracking-tighter" onClick={() => navigator.clipboard.writeText(response.body)}>
                             <Copy className="w-3 h-3 mr-1.5" />
                             Copy Body
                           </Button>
                        </div>
                      </div>
                   </div>
                </div>
              ) : (
                <div className="flex-1 flex flex-col items-center justify-center text-slate-300 dark:text-slate-700 space-y-3">
                  <div className="w-12 h-12 rounded-2xl bg-slate-50 dark:bg-slate-800/50 flex items-center justify-center">
                    <Send className="w-6 h-6" />
                  </div>
                  <div className="text-sm font-medium">Ready to send a request</div>
                  <div className="text-xs text-slate-400 italic">Response data will appear here</div>
                </div>
              )}
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
