import * as React from 'react';
import type { Side, Align } from "../../utils/useAnchorPositioning.js";
import type { BaseUIComponentProps } from "../../utils/types.js";
/**
 * Displays an element positioned against the anchor.
 * Renders a `<div>` element.
 */
export declare const ComboboxArrow: React.ForwardRefExoticComponent<Omit<ComboboxArrowProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface ComboboxArrowState {
  /**
   * Whether the popup is currently open.
   */
  open: boolean;
  side: Side;
  align: Align;
  uncentered: boolean;
}
export interface ComboboxArrowProps extends BaseUIComponentProps<'div', ComboboxArrow.State> {}
export declare namespace ComboboxArrow {
  type State = ComboboxArrowState;
  type Props = ComboboxArrowProps;
}