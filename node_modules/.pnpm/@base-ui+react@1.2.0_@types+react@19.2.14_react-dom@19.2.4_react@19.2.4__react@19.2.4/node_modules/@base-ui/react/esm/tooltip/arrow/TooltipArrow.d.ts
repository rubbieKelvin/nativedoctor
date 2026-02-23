import * as React from 'react';
import type { BaseUIComponentProps } from "../../utils/types.js";
import type { Side, Align } from "../../utils/useAnchorPositioning.js";
/**
 * Displays an element positioned against the tooltip anchor.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Tooltip](https://base-ui.com/react/components/tooltip)
 */
export declare const TooltipArrow: React.ForwardRefExoticComponent<Omit<TooltipArrowProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface TooltipArrowState {
  /**
   * Whether the tooltip is currently open.
   */
  open: boolean;
  side: Side;
  align: Align;
  uncentered: boolean;
  instant: 'delay' | 'dismiss' | 'focus' | undefined;
}
export interface TooltipArrowProps extends BaseUIComponentProps<'div', TooltipArrow.State> {}
export declare namespace TooltipArrow {
  type State = TooltipArrowState;
  type Props = TooltipArrowProps;
}