import * as React from 'react';
import type { BaseUIComponentProps } from "../../utils/types.js";
import type { Side, Align } from "../../utils/useAnchorPositioning.js";
/**
 * Displays an element positioned against the toast anchor.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Toast](https://base-ui.com/react/components/toast)
 */
export declare const ToastArrow: React.ForwardRefExoticComponent<Omit<ToastArrowProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface ToastArrowState {
  side: Side;
  align: Align;
  uncentered: boolean;
}
export interface ToastArrowProps extends BaseUIComponentProps<'div', ToastArrow.State> {}
export declare namespace ToastArrow {
  type State = ToastArrowState;
  type Props = ToastArrowProps;
}