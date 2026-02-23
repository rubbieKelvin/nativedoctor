import * as React from 'react';
import type { BaseUIComponentProps } from "../../utils/types.js";
import type { SliderRoot } from "../root/SliderRoot.js";
/**
 * Visualizes the current value of the slider.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Slider](https://base-ui.com/react/components/slider)
 */
export declare const SliderIndicator: React.ForwardRefExoticComponent<Omit<SliderIndicatorProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface SliderIndicatorProps extends BaseUIComponentProps<'div', SliderRoot.State> {}
export declare namespace SliderIndicator {
  type Props = SliderIndicatorProps;
}