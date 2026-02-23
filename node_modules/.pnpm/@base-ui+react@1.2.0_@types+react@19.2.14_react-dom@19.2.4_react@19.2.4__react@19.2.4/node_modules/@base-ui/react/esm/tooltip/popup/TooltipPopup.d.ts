import * as React from 'react';
import type { BaseUIComponentProps } from "../../utils/types.js";
import type { Align, Side } from "../../utils/useAnchorPositioning.js";
import type { TransitionStatus } from "../../utils/useTransitionStatus.js";
/**
 * A container for the tooltip contents.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Tooltip](https://base-ui.com/react/components/tooltip)
 */
export declare const TooltipPopup: React.ForwardRefExoticComponent<Omit<TooltipPopupProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface TooltipPopupState {
  /**
   * Whether the tooltip is currently open.
   */
  open: boolean;
  side: Side;
  align: Align;
  instant: 'delay' | 'focus' | 'dismiss' | undefined;
  transitionStatus: TransitionStatus;
}
export interface TooltipPopupProps extends BaseUIComponentProps<'div', TooltipPopup.State> {}
export declare namespace TooltipPopup {
  type State = TooltipPopupState;
  type Props = TooltipPopupProps;
}