import * as React from 'react';
import { useAnchorPositioning, type Side, type Align } from "../../utils/useAnchorPositioning.js";
import type { BaseUIComponentProps } from "../../utils/types.js";
import type { ToastObject } from "../useToastManager.js";
/**
 * Positions the toast against the anchor.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Toast](https://base-ui.com/react/components/toast)
 */
export declare const ToastPositioner: React.ForwardRefExoticComponent<Omit<ToastPositionerProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface ToastPositionerState {
  side: Side;
  align: Align;
  anchorHidden: boolean;
}
export interface ToastPositionerProps extends BaseUIComponentProps<'div', ToastPositioner.State>, Omit<useAnchorPositioning.SharedParameters, 'side' | 'anchor'> {
  /**
   * An element to position the toast against.
   */
  anchor?: (Element | null) | undefined;
  /**
   * Which side of the anchor element to align the toast against.
   * May automatically change to avoid collisions.
   * @default 'top'
   */
  side?: Side | undefined;
  /**
   * The toast object associated with the positioner.
   */
  toast: ToastObject<any>;
}
export declare namespace ToastPositioner {
  type State = ToastPositionerState;
  type Props = ToastPositionerProps;
}