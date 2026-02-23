import * as React from 'react';
import type { BaseUIComponentProps } from "../../utils/types.js";
import { Side } from "../../utils/useAnchorPositioning.js";
import { type TransitionStatus } from "../../utils/useTransitionStatus.js";
/**
 * @internal
 */
export declare const SelectScrollArrow: React.ForwardRefExoticComponent<Omit<SelectScrollArrowProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface SelectScrollArrowState {
  direction: 'up' | 'down';
  visible: boolean;
  side: Side | 'none';
  transitionStatus: TransitionStatus;
}
export interface SelectScrollArrowProps extends BaseUIComponentProps<'div', SelectScrollArrow.State> {
  direction: 'up' | 'down';
  /**
   * Whether to keep the HTML element in the DOM while the select popup is not scrollable.
   * @default false
   */
  keepMounted?: boolean | undefined;
}
export declare namespace SelectScrollArrow {
  type State = SelectScrollArrowState;
  type Props = SelectScrollArrowProps;
}