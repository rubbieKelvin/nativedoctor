import * as React from 'react';
import type { BaseUIComponentProps } from "../../utils/types.js";
import type { Align, Side } from "../../utils/useAnchorPositioning.js";
/**
 * Displays an element positioned against the select popup anchor.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Select](https://base-ui.com/react/components/select)
 */
export declare const SelectArrow: React.ForwardRefExoticComponent<Omit<SelectArrowProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface SelectArrowState {
  /**
   * Whether the select popup is currently open.
   */
  open: boolean;
  side: Side | 'none';
  align: Align;
  uncentered: boolean;
}
export interface SelectArrowProps extends BaseUIComponentProps<'div', SelectArrow.State> {}
export declare namespace SelectArrow {
  type State = SelectArrowState;
  type Props = SelectArrowProps;
}