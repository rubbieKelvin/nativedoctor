import * as React from 'react';
import type { BaseUIComponentProps } from "../../utils/types.js";
import { useAnchorPositioning, type Align, type Side } from "../../utils/useAnchorPositioning.js";
/**
 * Positions the navigation menu against the currently active trigger.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Navigation Menu](https://base-ui.com/react/components/navigation-menu)
 */
export declare const NavigationMenuPositioner: React.ForwardRefExoticComponent<Omit<NavigationMenuPositionerProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface NavigationMenuPositionerState {
  /**
   * Whether the navigation menu is currently open.
   */
  open: boolean;
  side: Side;
  align: Align;
  anchorHidden: boolean;
  /**
   * Whether CSS transitions should be disabled.
   */
  instant: boolean;
}
export interface NavigationMenuPositionerProps extends useAnchorPositioning.SharedParameters, BaseUIComponentProps<'div', NavigationMenuPositioner.State> {}
export declare namespace NavigationMenuPositioner {
  type State = NavigationMenuPositionerState;
  type Props = NavigationMenuPositionerProps;
}