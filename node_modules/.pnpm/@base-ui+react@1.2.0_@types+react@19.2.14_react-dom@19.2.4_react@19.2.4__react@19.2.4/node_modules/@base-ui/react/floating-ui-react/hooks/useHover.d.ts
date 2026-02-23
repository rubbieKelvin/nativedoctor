import type { Delay, ElementProps, FloatingContext, FloatingRootContext, FloatingTreeType, SafePolygonOptions } from "../types.js";
export interface HandleCloseContext extends FloatingContext {
  onClose: () => void;
  tree?: (FloatingTreeType | null) | undefined;
  leave?: boolean | undefined;
}
export interface HandleClose {
  (context: HandleCloseContext): (event: MouseEvent) => void;
  __options?: SafePolygonOptions | undefined;
}
export declare function getDelay(value: UseHoverProps['delay'], prop: 'open' | 'close', pointerType?: PointerEvent['pointerType']): number | undefined;
export interface UseHoverProps {
  /**
   * Accepts an event handler that runs on `mousemove` to control when the
   * floating element closes once the cursor leaves the reference element.
   * @default null
   */
  handleClose?: (HandleClose | null) | undefined;
  /**
   * Waits until the user’s cursor is at “rest” over the reference element
   * before changing the `open` state.
   * @default 0
   */
  restMs?: (number | (() => number)) | undefined;
  /**
   * Waits for the specified time when the event listener runs before changing
   * the `open` state.
   * @default 0
   */
  delay?: (Delay | (() => Delay)) | undefined;
  /**
   * Whether moving the cursor over the floating element will open it, without a
   * regular hover event required.
   * @default true
   */
  move?: boolean | undefined;
}
/**
 * Opens the floating element while hovering over the reference element, like
 * CSS `:hover`.
 * @see https://floating-ui.com/docs/useHover
 */
export declare function useHover(context: FloatingRootContext | FloatingContext, props?: UseHoverProps): ElementProps;