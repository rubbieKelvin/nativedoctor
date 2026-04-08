import {
    HighlightStyle,
    LanguageSupport,
    StreamLanguage,
    type StringStream,
    syntaxHighlighting,
} from "@codemirror/language";
import { tags } from "@lezer/highlight";

/** Parser state: track unclosed block comments across lines. */
interface RhaiState {
    inBlockComment: boolean;
}

const KEYWORDS = new Set(
    [
        "as",
        "async",
        "await",
        "break",
        "catch",
        "const",
        "continue",
        "do",
        "else",
        "export",
        "false",
        "fn",
        "for",
        "if",
        "import",
        "in",
        "is",
        "let",
        "loop",
        "module",
        "private",
        "public",
        "return",
        "silent",
        "switch",
        "throw",
        "true",
        "try",
        "type",
        "until",
        "while",
        "with",
        "go_to",
        "mod",
        "enum",
        "global",
        "super",
        "var",
    ].map((k) => k.toLowerCase()),
);

/** Nativedoctor Rhai globals (README). */
const BUILTINS = new Set(
    ["env", "set", "assert", "log", "persist", "invoke"].map((k) =>
        k.toLowerCase(),
    ),
);

/** Theme uses CSS variables so light/dark both stay readable. */
const rhaiHighlightStyle = HighlightStyle.define([
    { tag: tags.keyword, color: "hsl(var(--chart-3))" },
    { tag: tags.standard(tags.name), color: "hsl(var(--chart-1))" },
    { tag: tags.string, color: "hsl(var(--chart-2))" },
    {
        tag: tags.lineComment,
        color: "hsl(var(--muted-foreground))",
        fontStyle: "italic",
    },
    {
        tag: tags.blockComment,
        color: "hsl(var(--muted-foreground))",
        fontStyle: "italic",
    },
    { tag: tags.number, color: "hsl(var(--chart-4))" },
    { tag: tags.operator, color: "hsl(var(--muted-foreground))" },
    { tag: tags.punctuation, color: "hsl(var(--muted-foreground))" },
    { tag: tags.variableName, color: "hsl(var(--foreground))" },
]);

function tokenString(stream: StringStream): string {
    stream.next(); // opening "
    while (!stream.eol()) {
        const c = stream.next();
        if (c === "\\") {
            if (!stream.eol()) stream.next();
        } else if (c === '"') {
            return "str";
        }
    }
    return "str";
}

function tokenChar(stream: StringStream): string {
    stream.next(); // opening '
    if (stream.eol()) return "str";
    if (stream.peek() === "\\") {
        stream.next();
        if (!stream.eol()) stream.next();
    } else {
        stream.next();
    }
    if (stream.peek() === "'") stream.next();
    return "str";
}

const rhaiLanguage = StreamLanguage.define<RhaiState>({
    name: "rhai",
    startState() {
        return { inBlockComment: false };
    },
    copyState(s) {
        return { inBlockComment: s.inBlockComment };
    },
    languageData: {
        commentTokens: { line: "//", block: { open: "/*", close: "*/" } },
    },
    tokenTable: {
        kw: tags.keyword,
        builtin: tags.standard(tags.name),
        str: tags.string,
        lineComment: tags.lineComment,
        blockComment: tags.blockComment,
        num: tags.number,
        op: tags.operator,
        punct: tags.punctuation,
        ident: tags.variableName,
    },
    token(stream, state) {
        if (state.inBlockComment) {
            if (stream.match(/^.*?\*\//)) {
                state.inBlockComment = false;
                return "blockComment";
            }
            stream.skipToEnd();
            return "blockComment";
        }

        stream.eatSpace();
        if (stream.eol()) return null;

        if (stream.match("//")) {
            stream.skipToEnd();
            return "lineComment";
        }

        if (stream.match("/*")) {
            if (stream.match(/^.*?\*\//)) {
                return "blockComment";
            }
            state.inBlockComment = true;
            stream.skipToEnd();
            return "blockComment";
        }

        const p = stream.peek();
        if (p === '"') {
            return tokenString(stream);
        }
        if (p === "'") {
            return tokenChar(stream);
        }

        if (stream.match(/^0x[\da-fA-F][\da-fA-F_]*/)) {
            return "num";
        }
        if (
            stream.match(/^\d[\d_]*\.\d[\d_]*([eE][-+]?\d[\d_]*)?/) ||
            stream.match(/^\d[\d_]*[eE][-+]?\d[\d_]*/) ||
            stream.match(/^\.\d[\d_]*([eE][-+]?\d[\d_]*)?/) ||
            stream.match(/^\d[\d_]*/)
        ) {
            return "num";
        }

        if (
            stream.match("::") ||
            stream.match("->") ||
            stream.match("<<=") ||
            stream.match(">>=") ||
            stream.match("<<") ||
            stream.match(">>") ||
            stream.match("<=") ||
            stream.match(">=") ||
            stream.match("==") ||
            stream.match("!=") ||
            stream.match("&&") ||
            stream.match("||") ||
            stream.match("..") ||
            stream.match("**") ||
            stream.match("+=") ||
            stream.match("-=") ||
            stream.match("*=") ||
            stream.match("/=") ||
            stream.match("%=") ||
            stream.match("^=") ||
            stream.match("|=") ||
            stream.match("&=")
        ) {
            return "op";
        }

        const op1 = stream.peek();
        if ("+-*/%=<>&|^!~?".includes(op1 ?? "")) {
            stream.next();
            return "op";
        }

        if ("()[]{},;:@".includes(p ?? "")) {
            stream.next();
            return "punct";
        }

        if (p === ".") {
            stream.next();
            return "punct";
        }

        if (stream.match(/^[\w$][\w$]*/)) {
            const w = stream.current().toLowerCase();
            if (KEYWORDS.has(w)) return "kw";
            if (BUILTINS.has(w)) return "builtin";
            return "ident";
        }

        stream.next();
        return null;
    },
});

/**
 * Rhai syntax highlighting for CodeMirror 6 (stream tokenizer; not a full Rhai parser).
 */
export function rhai(): LanguageSupport {
    return new LanguageSupport(rhaiLanguage, [
        syntaxHighlighting(rhaiHighlightStyle, { fallback: true }),
    ]);
}
