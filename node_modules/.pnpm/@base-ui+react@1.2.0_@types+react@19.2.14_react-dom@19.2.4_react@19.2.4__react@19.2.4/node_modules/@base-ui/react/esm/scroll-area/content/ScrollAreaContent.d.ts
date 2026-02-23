import * as React from 'react';
import type { BaseUIComponentProps } from "../../utils/types.js";
import type { ScrollAreaRoot } from "../root/ScrollAreaRoot.js";
/**
 * A container for the content of the scroll area.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Scroll Area](https://base-ui.com/react/components/scroll-area)
 */
export declare const ScrollAreaContent: React.ForwardRefExoticComponent<Omit<ScrollAreaContentProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface ScrollAreaContentState extends ScrollAreaRoot.State {}
export interface ScrollAreaContentProps extends BaseUIComponentProps<'div', ScrollAreaContent.State> {}
export declare namespace ScrollAreaContent {
  type State = ScrollAreaContentState;
  type Props = ScrollAreaContentProps;
}