import * as React from 'react';
import type { Side, Align } from "../../utils/useAnchorPositioning.js";
import type { BaseUIComponentProps } from "../../utils/types.js";
/**
 * Displays an element positioned against the menu anchor.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Menu](https://base-ui.com/react/components/menu)
 */
export declare const MenuArrow: React.ForwardRefExoticComponent<Omit<MenuArrowProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface MenuArrowState {
  /**
   * Whether the menu is currently open.
   */
  open: boolean;
  side: Side;
  align: Align;
  uncentered: boolean;
}
export interface MenuArrowProps extends BaseUIComponentProps<'div', MenuArrow.State> {}
export declare namespace MenuArrow {
  type State = MenuArrowState;
  type Props = MenuArrowProps;
}