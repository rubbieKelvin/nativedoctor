import * as React from 'react';
import { type Side, type Align, useAnchorPositioning } from "../../utils/useAnchorPositioning.js";
import type { BaseUIComponentProps } from "../../utils/types.js";
/**
 * Positions the popup against the trigger.
 * Renders a `<div>` element.
 */
export declare const ComboboxPositioner: React.ForwardRefExoticComponent<Omit<ComboboxPositionerProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface ComboboxPositionerState {
  /**
   * Whether the popup is currently open.
   */
  open: boolean;
  side: Side;
  align: Align;
  anchorHidden: boolean;
  empty: boolean;
}
export interface ComboboxPositionerProps extends useAnchorPositioning.SharedParameters, BaseUIComponentProps<'div', ComboboxPositioner.State> {}
export declare namespace ComboboxPositioner {
  type State = ComboboxPositionerState;
  type Props = ComboboxPositionerProps;
}