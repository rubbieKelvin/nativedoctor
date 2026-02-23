import { Timeout } from '@base-ui/utils/useTimeout';
import type { FloatingRootContext, SafePolygonOptions } from "../types.js";
export declare const safePolygonIdentifier: string;
export declare function isInteractiveElement(element: Element | null): boolean;
export declare class HoverInteraction {
  pointerType: string | undefined;
  interactedInside: boolean;
  handler: ((event: MouseEvent) => void) | undefined;
  blockMouseMove: boolean;
  performedPointerEventsMutation: boolean;
  unbindMouseMove: () => void;
  restTimeoutPending: boolean;
  openChangeTimeout: Timeout;
  restTimeout: Timeout;
  handleCloseOptions: SafePolygonOptions | undefined;
  constructor();
  static create(): HoverInteraction;
  dispose: () => void;
  disposeEffect: () => () => void;
}
export declare function useHoverInteractionSharedState(store: FloatingRootContext): HoverInteraction;