import * as React from 'react';
import type { BaseUIComponentProps } from "../../utils/types.js";
import type { SliderRoot } from "../root/SliderRoot.js";
/**
 * Displays the current value of the slider as text.
 * Renders an `<output>` element.
 *
 * Documentation: [Base UI Slider](https://base-ui.com/react/components/slider)
 */
export declare const SliderValue: React.ForwardRefExoticComponent<Omit<SliderValueProps, "ref"> & React.RefAttributes<HTMLOutputElement>>;
export interface SliderValueProps extends Omit<BaseUIComponentProps<'output', SliderRoot.State>, 'children'> {
  children?: (null | ((formattedValues: readonly string[], values: readonly number[]) => React.ReactNode)) | undefined;
}
export declare namespace SliderValue {
  type Props = SliderValueProps;
}