import * as React from 'react';
import type { BaseUIComponentProps } from "../../utils/types.js";
import type { Align, Side } from "../../utils/useAnchorPositioning.js";
/**
 * Displays an element positioned against the preview card anchor.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Preview Card](https://base-ui.com/react/components/preview-card)
 */
export declare const PreviewCardArrow: React.ForwardRefExoticComponent<Omit<PreviewCardArrowProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface PreviewCardArrowState {
  /**
   * Whether the preview card is currently open.
   */
  open: boolean;
  side: Side;
  align: Align;
  uncentered: boolean;
}
export interface PreviewCardArrowProps extends BaseUIComponentProps<'div', PreviewCardArrow.State> {}
export declare namespace PreviewCardArrow {
  type State = PreviewCardArrowState;
  type Props = PreviewCardArrowProps;
}