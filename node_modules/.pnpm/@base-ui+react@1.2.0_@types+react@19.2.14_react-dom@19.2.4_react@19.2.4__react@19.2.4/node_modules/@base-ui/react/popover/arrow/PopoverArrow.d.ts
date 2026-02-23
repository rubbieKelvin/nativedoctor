import * as React from 'react';
import type { Align, Side } from "../../utils/useAnchorPositioning.js";
import type { BaseUIComponentProps } from "../../utils/types.js";
/**
 * Displays an element positioned against the popover anchor.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Popover](https://base-ui.com/react/components/popover)
 */
export declare const PopoverArrow: React.ForwardRefExoticComponent<Omit<PopoverArrowProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface PopoverArrowState {
  /**
   * Whether the popover is currently open.
   */
  open: boolean;
  side: Side;
  align: Align;
  uncentered: boolean;
}
export interface PopoverArrowProps extends BaseUIComponentProps<'div', PopoverArrow.State> {}
export declare namespace PopoverArrow {
  type State = PopoverArrowState;
  type Props = PopoverArrowProps;
}