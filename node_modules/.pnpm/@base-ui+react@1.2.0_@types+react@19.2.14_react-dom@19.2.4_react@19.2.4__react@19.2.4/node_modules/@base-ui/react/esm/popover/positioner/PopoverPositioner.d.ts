import * as React from 'react';
import { useAnchorPositioning, type Side, type Align } from "../../utils/useAnchorPositioning.js";
import type { BaseUIComponentProps } from "../../utils/types.js";
/**
 * Positions the popover against the trigger.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Popover](https://base-ui.com/react/components/popover)
 */
export declare const PopoverPositioner: React.ForwardRefExoticComponent<Omit<PopoverPositionerProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface PopoverPositionerState {
  /**
   * Whether the popover is currently open.
   */
  open: boolean;
  side: Side;
  align: Align;
  anchorHidden: boolean;
  /**
   * Whether CSS transitions should be disabled.
   */
  instant: string | undefined;
}
export interface PopoverPositionerProps extends useAnchorPositioning.SharedParameters, BaseUIComponentProps<'div', PopoverPositioner.State> {}
export declare namespace PopoverPositioner {
  type State = PopoverPositionerState;
  type Props = PopoverPositionerProps;
}