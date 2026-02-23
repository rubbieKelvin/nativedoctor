import * as React from 'react';
import { useAnchorPositioning, type Align, type Side } from "../../utils/useAnchorPositioning.js";
import { BaseUIComponentProps } from "../../utils/types.js";
/**
 * Positions the menu popup against the trigger.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Menu](https://base-ui.com/react/components/menu)
 */
export declare const MenuPositioner: React.ForwardRefExoticComponent<Omit<MenuPositionerProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface MenuPositionerState {
  /**
   * Whether the menu is currently open.
   */
  open: boolean;
  side: Side;
  align: Align;
  anchorHidden: boolean;
  nested: boolean;
}
export interface MenuPositionerProps extends useAnchorPositioning.SharedParameters, BaseUIComponentProps<'div', MenuPositioner.State> {}
export declare namespace MenuPositioner {
  type State = MenuPositionerState;
  type Props = MenuPositionerProps;
}