import * as React from 'react';
import { BaseUIComponentProps } from "../../utils/types.js";
/**
 * A viewport for displaying content transitions.
 * This component is only required if one popup can be opened by multiple triggers, its content change based on the trigger
 * and switching between them is animated.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Preview Card](https://base-ui.com/react/components/preview-card)
 */
export declare const PreviewCardViewport: React.ForwardRefExoticComponent<Omit<PreviewCardViewport.Props, "ref"> & React.RefAttributes<HTMLDivElement>>;
export declare namespace PreviewCardViewport {
  interface Props extends BaseUIComponentProps<'div', State> {
    /**
     * The content to render inside the transition container.
     */
    children?: React.ReactNode;
  }
  interface State {
    activationDirection: string | undefined;
    /**
     * Whether the viewport is currently transitioning between contents.
     */
    transitioning: boolean;
    instant: 'dismiss' | 'focus' | undefined;
  }
}