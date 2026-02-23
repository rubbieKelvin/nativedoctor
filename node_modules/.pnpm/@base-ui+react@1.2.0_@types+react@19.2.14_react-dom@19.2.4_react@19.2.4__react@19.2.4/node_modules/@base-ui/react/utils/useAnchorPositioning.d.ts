import * as React from 'react';
import { type Rect } from '@floating-ui/utils';
import { useFloating, type FloatingRootContext, type VirtualElement, type Padding, type FloatingContext, type Side as PhysicalSide, type Middleware, type FloatingTreeStore } from "../floating-ui-react/index.js";
export type Side = 'top' | 'bottom' | 'left' | 'right' | 'inline-end' | 'inline-start';
export type Align = 'start' | 'center' | 'end';
export type Boundary = 'clipping-ancestors' | Element | Element[] | Rect;
export type OffsetFunction = (data: {
  side: Side;
  align: Align;
  anchor: {
    width: number;
    height: number;
  };
  positioner: {
    width: number;
    height: number;
  };
}) => number;
interface SideFlipMode {
  /**
   * How to avoid collisions on the side axis.
   */
  side?: ('flip' | 'none') | undefined;
  /**
   * How to avoid collisions on the align axis.
   */
  align?: ('flip' | 'shift' | 'none') | undefined;
  /**
   * If both sides on the preferred axis do not fit, determines whether to fallback
   * to a side on the perpendicular axis and which logical side to prefer.
   */
  fallbackAxisSide?: ('start' | 'end' | 'none') | undefined;
}
interface SideShiftMode {
  /**
   * How to avoid collisions on the side axis.
   */
  side?: ('shift' | 'none') | undefined;
  /**
   * How to avoid collisions on the align axis.
   */
  align?: ('shift' | 'none') | undefined;
  /**
   * If both sides on the preferred axis do not fit, determines whether to fallback
   * to a side on the perpendicular axis and which logical side to prefer.
   */
  fallbackAxisSide?: ('start' | 'end' | 'none') | undefined;
}
export type CollisionAvoidance = SideFlipMode | SideShiftMode;
/**
 * Provides standardized anchor positioning behavior for floating elements. Wraps Floating UI's
 * `useFloating` hook.
 */
export declare function useAnchorPositioning(params: useAnchorPositioning.Parameters): useAnchorPositioning.ReturnValue;
export interface UseAnchorPositioningSharedParameters {
  /**
   * An element to position the popup against.
   * By default, the popup will be positioned against the trigger.
   */
  anchor?: (Element | null | VirtualElement | React.RefObject<Element | null> | (() => Element | VirtualElement | null)) | undefined;
  /**
   * Determines which CSS `position` property to use.
   * @default 'absolute'
   */
  positionMethod?: ('absolute' | 'fixed') | undefined;
  /**
   * Which side of the anchor element to align the popup against.
   * May automatically change to avoid collisions.
   * @default 'bottom'
   */
  side?: Side | undefined;
  /**
   * Distance between the anchor and the popup in pixels.
   * Also accepts a function that returns the distance to read the dimensions of the anchor
   * and positioner elements, along with its side and alignment.
   *
   * The function takes a `data` object parameter with the following properties:
   * - `data.anchor`: the dimensions of the anchor element with properties `width` and `height`.
   * - `data.positioner`: the dimensions of the positioner element with properties `width` and `height`.
   * - `data.side`: which side of the anchor element the positioner is aligned against.
   * - `data.align`: how the positioner is aligned relative to the specified side.
   *
   * @example
   * ```jsx
   * <Positioner
   *   sideOffset={({ side, align, anchor, positioner }) => {
   *     return side === 'top' || side === 'bottom'
   *       ? anchor.height
   *       : anchor.width;
   *   }}
   * />
   * ```
   *
   * @default 0
   */
  sideOffset?: (number | OffsetFunction) | undefined;
  /**
   * How to align the popup relative to the specified side.
   * @default 'center'
   */
  align?: Align | undefined;
  /**
   * Additional offset along the alignment axis in pixels.
   * Also accepts a function that returns the offset to read the dimensions of the anchor
   * and positioner elements, along with its side and alignment.
   *
   * The function takes a `data` object parameter with the following properties:
   * - `data.anchor`: the dimensions of the anchor element with properties `width` and `height`.
   * - `data.positioner`: the dimensions of the positioner element with properties `width` and `height`.
   * - `data.side`: which side of the anchor element the positioner is aligned against.
   * - `data.align`: how the positioner is aligned relative to the specified side.
   *
   * @example
   * ```jsx
   * <Positioner
   *   alignOffset={({ side, align, anchor, positioner }) => {
   *     return side === 'top' || side === 'bottom'
   *       ? anchor.width
   *       : anchor.height;
   *   }}
   * />
   * ```
   *
   * @default 0
   */
  alignOffset?: (number | OffsetFunction) | undefined;
  /**
   * An element or a rectangle that delimits the area that the popup is confined to.
   * @default 'clipping-ancestors'
   */
  collisionBoundary?: Boundary | undefined;
  /**
   * Additional space to maintain from the edge of the collision boundary.
   * @default 5
   */
  collisionPadding?: Padding | undefined;
  /**
   * Whether to maintain the popup in the viewport after
   * the anchor element was scrolled out of view.
   * @default false
   */
  sticky?: boolean | undefined;
  /**
   * Minimum distance to maintain between the arrow and the edges of the popup.
   *
   * Use it to prevent the arrow element from hanging out of the rounded corners of a popup.
   * @default 5
   */
  arrowPadding?: number | undefined;
  /**
   * Whether to disable the popup from tracking any layout shift of its positioning anchor.
   * @default false
   */
  disableAnchorTracking?: boolean | undefined;
  /**
   * Determines how to handle collisions when positioning the popup.
   *
   * @example
   * ```jsx
   * <Positioner
   *   collisionAvoidance={{
   *     side: 'shift',
   *     align: 'shift',
   *     fallbackAxisSide: 'none',
   *   }}
   * />
   * ```
   *
   */
  collisionAvoidance?: CollisionAvoidance | undefined;
}
export interface UseAnchorPositioningParameters extends useAnchorPositioning.SharedParameters {
  keepMounted?: boolean | undefined;
  trackCursorAxis?: ('none' | 'x' | 'y' | 'both') | undefined;
  floatingRootContext?: FloatingRootContext | undefined;
  mounted: boolean;
  disableAnchorTracking: boolean;
  nodeId?: string | undefined;
  adaptiveOrigin?: Middleware | undefined;
  collisionAvoidance: CollisionAvoidance;
  shiftCrossAxis?: boolean | undefined;
  lazyFlip?: boolean | undefined;
  externalTree?: FloatingTreeStore | undefined;
}
export interface UseAnchorPositioningReturnValue {
  positionerStyles: React.CSSProperties;
  arrowStyles: React.CSSProperties;
  arrowRef: React.RefObject<Element | null>;
  arrowUncentered: boolean;
  side: Side;
  align: Align;
  physicalSide: PhysicalSide;
  anchorHidden: boolean;
  refs: ReturnType<typeof useFloating>['refs'];
  context: FloatingContext;
  isPositioned: boolean;
  update: () => void;
}
export declare namespace useAnchorPositioning {
  type SharedParameters = UseAnchorPositioningSharedParameters;
  type Parameters = UseAnchorPositioningParameters;
  type ReturnValue = UseAnchorPositioningReturnValue;
}
export {};