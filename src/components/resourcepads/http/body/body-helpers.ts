import type {
  BodyType,
  HttpBody,
  HttpBodyBinary,
  HttpBodyFormData,
  HttpBodyFormUrlencoded,
  HttpBodyGraphql,
  HttpBodyNone,
  HttpBodyText,
} from "@/shared/types/resources";
import { BODY_TYPES } from "@/shared/types/resources";

/**
 * Serialize HttpBody to the string sent as the HTTP request body (or null for none).
 * Used when building the payload for send_http_request so the backend receives a plain string.
 */
export function serializeHttpBodyToRequestString(body: HttpBody): string | null {
  switch (body.type) {
    case "none":
      return null;
    case "json":
    case "text":
    case "xml":
    case "other":
      return body.content?.trim() ? body.content : null;
    case "graphql": {
      const s = JSON.stringify({
        query: body.query,
        variables: body.variables?.trim()
          ? (() => {
              try {
                return JSON.parse(body.variables);
              } catch {
                return {};
              }
            })()
          : {},
      });
      return s;
    }
    case "x-www-form-urlencoded": {
      const parts = body.fields
        .filter((f) => f.enabled && String(f.key ?? "").trim() !== "")
        .map(
          (f) =>
            `${encodeURIComponent(String(f.key).trim())}=${encodeURIComponent(f.value ?? "")}`,
        );
      return parts.length > 0 ? parts.join("&") : null;
    }
    case "form-data":
    case "binary":
      return null;
    default:
      return null;
  }
}

export const BODY_TYPE_OPTIONS: { value: BodyType; label: string }[] = [
  { value: BODY_TYPES.NONE, label: "None" },
  { value: BODY_TYPES.JSON, label: "JSON" },
  { value: BODY_TYPES.TEXT, label: "Text" },
  { value: BODY_TYPES.XML, label: "XML" },
  { value: BODY_TYPES.OTHER, label: "Other" },
  { value: BODY_TYPES.GRAPHQL, label: "GraphQL" },
  { value: BODY_TYPES.FORM_URLENCODED, label: "Form URL Encoded" },
  { value: BODY_TYPES.FORM_DATA, label: "Multipart" },
  { value: BODY_TYPES.BINARY, label: "Binary" },
];

export function defaultBodyForType(type: BodyType): HttpBody {
  switch (type) {
    case "none":
      return { type: "none" } satisfies HttpBodyNone;
    case "json":
    case "text":
    case "xml":
    case "other":
      return { type, content: "" } satisfies HttpBodyText;
    case "graphql":
      return {
        type: "graphql",
        query: "",
        variables: "",
      } satisfies HttpBodyGraphql;
    case "form-data":
      return { type: "form-data", fields: [] } satisfies HttpBodyFormData;
    case "x-www-form-urlencoded":
      return {
        type: "x-www-form-urlencoded",
        fields: [],
      } satisfies HttpBodyFormUrlencoded;
    case "binary":
      return { type: "binary", file_path: "" } satisfies HttpBodyBinary;
    default:
      return { type: "none" };
  }
}
