import * as React from 'react';
import type { Align, Side } from "../../utils/useAnchorPositioning.js";
import type { BaseUIComponentProps } from "../../utils/types.js";
/**
 * Displays an element pointing toward the navigation menu's current anchor.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Navigation Menu](https://base-ui.com/react/components/navigation-menu)
 */
export declare const NavigationMenuArrow: React.ForwardRefExoticComponent<Omit<NavigationMenuArrowProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface NavigationMenuArrowState {
  /**
   * Whether the popup is currently open.
   */
  open: boolean;
  side: Side;
  align: Align;
  uncentered: boolean;
}
export interface NavigationMenuArrowProps extends BaseUIComponentProps<'div', NavigationMenuArrow.State> {}
export declare namespace NavigationMenuArrow {
  type State = NavigationMenuArrowState;
  type Props = NavigationMenuArrowProps;
}