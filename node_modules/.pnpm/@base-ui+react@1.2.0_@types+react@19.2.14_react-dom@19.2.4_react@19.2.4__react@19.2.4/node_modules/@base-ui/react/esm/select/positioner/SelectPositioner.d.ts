import * as React from 'react';
import type { BaseUIComponentProps } from "../../utils/types.js";
import { useAnchorPositioning, type Align, type Side } from "../../utils/useAnchorPositioning.js";
/**
 * Positions the select popup.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Select](https://base-ui.com/react/components/select)
 */
export declare const SelectPositioner: React.ForwardRefExoticComponent<Omit<SelectPositionerProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface SelectPositionerState {
  open: boolean;
  side: Side | 'none';
  align: Align;
  anchorHidden: boolean;
}
export interface SelectPositionerProps extends useAnchorPositioning.SharedParameters, BaseUIComponentProps<'div', SelectPositioner.State> {
  /**
   * Whether the positioner overlaps the trigger so the selected item's text is aligned with the trigger's value text. This only applies to mouse input and is automatically disabled if there is not enough space.
   * @default true
   */
  alignItemWithTrigger?: boolean | undefined;
}
export declare namespace SelectPositioner {
  type State = SelectPositionerState;
  type Props = SelectPositionerProps;
}