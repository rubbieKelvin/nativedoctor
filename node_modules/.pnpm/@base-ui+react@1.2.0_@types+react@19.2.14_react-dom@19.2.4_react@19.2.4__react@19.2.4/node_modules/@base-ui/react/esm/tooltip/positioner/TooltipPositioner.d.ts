import * as React from 'react';
import { useAnchorPositioning, type Side, type Align } from "../../utils/useAnchorPositioning.js";
import type { BaseUIComponentProps } from "../../utils/types.js";
/**
 * Positions the tooltip against the trigger.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Tooltip](https://base-ui.com/react/components/tooltip)
 */
export declare const TooltipPositioner: React.ForwardRefExoticComponent<Omit<TooltipPositionerProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface TooltipPositionerState {
  /**
   * Whether the tooltip is currently open.
   */
  open: boolean;
  side: Side;
  align: Align;
  anchorHidden: boolean;
  /**
   * Whether CSS transitions should be disabled.
   */
  instant: string | undefined;
}
export interface TooltipPositionerProps extends BaseUIComponentProps<'div', TooltipPositioner.State>, Omit<useAnchorPositioning.SharedParameters, 'side'> {
  /**
   * Which side of the anchor element to align the popup against.
   * May automatically change to avoid collisions.
   * @default 'top'
   */
  side?: Side | undefined;
}
export declare namespace TooltipPositioner {
  type State = TooltipPositionerState;
  type Props = TooltipPositionerProps;
}