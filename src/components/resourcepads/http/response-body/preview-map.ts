import type { Component } from "vue";
import JsonPreview from "./JsonPreview.vue";
import HtmlPreview from "./HtmlPreview.vue";

/**
 * Normalizes a Content-Type header value to a key for the preview map
 * (e.g. "Application/JSON; charset=utf-8" -> "application/json").
 */
export function normalizeContentType(headerValue: string | undefined): string {
    if (!headerValue?.trim()) return "";
    const main = headerValue.split(";")[0].trim().toLowerCase();
    return main;
}

/**
 * Map from normalized content-type to the Vue component used for preview.
 * Add entries here to support more types.
 */
export const contentTypePreviewMap: Record<string, Component> = {
    "application/json": JsonPreview,
    "text/html": HtmlPreview,
};

export function getPreviewComponent(contentType: string): Component | undefined {
    const normalized = normalizeContentType(contentType);
    return contentTypePreviewMap[normalized];
}
