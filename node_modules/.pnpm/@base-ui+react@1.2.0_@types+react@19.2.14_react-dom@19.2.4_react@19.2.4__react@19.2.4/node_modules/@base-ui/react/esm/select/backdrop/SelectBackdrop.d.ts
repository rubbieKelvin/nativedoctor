import * as React from 'react';
import type { BaseUIComponentProps } from "../../utils/types.js";
import type { TransitionStatus } from "../../utils/useTransitionStatus.js";
/**
 * An overlay displayed beneath the menu popup.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Select](https://base-ui.com/react/components/select)
 */
export declare const SelectBackdrop: React.ForwardRefExoticComponent<Omit<SelectBackdropProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface SelectBackdropState {
  open: boolean;
  transitionStatus: TransitionStatus;
}
export interface SelectBackdropProps extends BaseUIComponentProps<'div', SelectBackdrop.State> {}
export declare namespace SelectBackdrop {
  type State = SelectBackdropState;
  type Props = SelectBackdropProps;
}