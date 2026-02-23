import type { HandleClose } from "./hooks/useHover.js";
export interface SafePolygonOptions {
  buffer?: number | undefined;
  blockPointerEvents?: boolean | undefined;
  requireIntent?: boolean | undefined;
}
/**
 * Generates a safe polygon area that the user can traverse without closing the
 * floating element once leaving the reference element.
 * @see https://floating-ui.com/docs/useHover#safepolygon
 */
export declare function safePolygon(options?: SafePolygonOptions): HandleClose;