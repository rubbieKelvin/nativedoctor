import * as React from 'react';
import { BaseUIComponentProps } from "../../utils/types.js";
import type { AvatarRoot } from "../root/AvatarRoot.js";
import { type TransitionStatus } from "../../utils/useTransitionStatus.js";
/**
 * Rendered when the image fails to load or when no image is provided.
 * Renders a `<span>` element.
 *
 * Documentation: [Base UI Avatar](https://base-ui.com/react/components/avatar)
 */
export declare const AvatarFallback: React.ForwardRefExoticComponent<Omit<AvatarFallbackProps, "ref"> & React.RefAttributes<HTMLSpanElement>>;
export interface AvatarFallbackState extends AvatarRoot.State {
  transitionStatus: TransitionStatus;
}
export interface AvatarFallbackProps extends BaseUIComponentProps<'span', AvatarFallback.State> {
  /**
   * How long to wait before showing the fallback. Specified in milliseconds.
   */
  delay?: number | undefined;
}
export declare namespace AvatarFallback {
  type State = AvatarFallbackState;
  type Props = AvatarFallbackProps;
}