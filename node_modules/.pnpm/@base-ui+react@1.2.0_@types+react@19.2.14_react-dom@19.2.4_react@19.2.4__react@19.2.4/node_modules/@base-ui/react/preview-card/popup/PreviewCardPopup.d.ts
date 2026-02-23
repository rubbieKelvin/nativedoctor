import * as React from 'react';
import type { Align, Side } from "../../utils/useAnchorPositioning.js";
import type { BaseUIComponentProps } from "../../utils/types.js";
import type { TransitionStatus } from "../../utils/useTransitionStatus.js";
/**
 * A container for the preview card contents.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Preview Card](https://base-ui.com/react/components/preview-card)
 */
export declare const PreviewCardPopup: React.ForwardRefExoticComponent<Omit<PreviewCardPopupProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface PreviewCardPopupState {
  /**
   * Whether the preview card is currently open.
   */
  open: boolean;
  side: Side;
  align: Align;
  instant: 'dismiss' | 'focus' | undefined;
  transitionStatus: TransitionStatus;
}
export interface PreviewCardPopupProps extends BaseUIComponentProps<'div', PreviewCardPopup.State> {}
export declare namespace PreviewCardPopup {
  type State = PreviewCardPopupState;
  type Props = PreviewCardPopupProps;
}