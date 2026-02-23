import * as React from 'react';
import type { BaseUIComponentProps } from "../../utils/types.js";
import type { SliderRoot } from "../root/SliderRoot.js";
/**
 * The clickable, interactive part of the slider.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Slider](https://base-ui.com/react/components/slider)
 */
export declare const SliderControl: React.ForwardRefExoticComponent<Omit<SliderControlProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface SliderControlProps extends BaseUIComponentProps<'div', SliderRoot.State> {}
export declare namespace SliderControl {
  type State = SliderRoot.State;
  type Props = SliderControlProps;
}