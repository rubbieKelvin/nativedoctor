import * as React from 'react';
import type { BaseUIComponentProps } from "../../utils/types.js";
import type { TransitionStatus } from "../../utils/useTransitionStatus.js";
/**
 * An overlay displayed beneath the popup.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Preview Card](https://base-ui.com/react/components/preview-card)
 */
export declare const PreviewCardBackdrop: React.ForwardRefExoticComponent<Omit<PreviewCardBackdropProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface PreviewCardBackdropState {
  /**
   * Whether the preview card is currently open.
   */
  open: boolean;
  transitionStatus: TransitionStatus;
}
export interface PreviewCardBackdropProps extends BaseUIComponentProps<'div', PreviewCardBackdrop.State> {}
export declare namespace PreviewCardBackdrop {
  type State = PreviewCardBackdropState;
  type Props = PreviewCardBackdropProps;
}