export type TabKind = "request" | "script";

export interface EditorTab {
    id: string;
    kind: TabKind;
    path: string;
    title: string;
    raw: string;
    ext: string;
    doc: Record<string, unknown> | null;
    parseError: string | null;
    /** JSON object of `${VAR}` overrides for this send only (request tabs). */
    overridesJson: string;
}

export type ReqSubTab =
    | "params"
    | "headers"
    | "body"
    | "input"
    | "auth";
