import * as React from 'react';
import type { BaseUIComponentProps } from "../../utils/types.js";
import type { TransitionStatus } from "../../utils/useTransitionStatus.js";
/**
 * An overlay displayed beneath the menu popup.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Menu](https://base-ui.com/react/components/menu)
 */
export declare const MenuBackdrop: React.ForwardRefExoticComponent<Omit<MenuBackdropProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface MenuBackdropState {
  /**
   * Whether the menu is currently open.
   */
  open: boolean;
  transitionStatus: TransitionStatus;
}
export interface MenuBackdropProps extends BaseUIComponentProps<'div', MenuBackdrop.State> {}
export declare namespace MenuBackdrop {
  type State = MenuBackdropState;
  type Props = MenuBackdropProps;
}