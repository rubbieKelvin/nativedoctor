import * as React from 'react';
import { type Side, type Align, useAnchorPositioning } from "../../utils/useAnchorPositioning.js";
import type { BaseUIComponentProps } from "../../utils/types.js";
/**
 * Positions the popup against the trigger.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Preview Card](https://base-ui.com/react/components/preview-card)
 */
export declare const PreviewCardPositioner: React.ForwardRefExoticComponent<Omit<PreviewCardPositionerProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface PreviewCardPositionerState {
  /**
   * Whether the preview card is currently open.
   */
  open: boolean;
  side: Side;
  align: Align;
  anchorHidden: boolean;
  instant: 'dismiss' | 'focus' | undefined;
}
export interface PreviewCardPositionerProps extends useAnchorPositioning.SharedParameters, BaseUIComponentProps<'div', PreviewCardPositioner.State> {}
export declare namespace PreviewCardPositioner {
  type State = PreviewCardPositionerState;
  type Props = PreviewCardPositionerProps;
}