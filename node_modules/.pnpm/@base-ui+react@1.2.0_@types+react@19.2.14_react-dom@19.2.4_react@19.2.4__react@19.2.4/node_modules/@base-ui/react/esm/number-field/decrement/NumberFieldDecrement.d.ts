import * as React from 'react';
import { BaseUIComponentProps, NativeButtonProps } from "../../utils/types.js";
import type { NumberFieldRoot } from "../root/NumberFieldRoot.js";
/**
 * A stepper button that decreases the field value when clicked.
 * Renders an `<button>` element.
 *
 * Documentation: [Base UI Number Field](https://base-ui.com/react/components/number-field)
 */
export declare const NumberFieldDecrement: React.ForwardRefExoticComponent<Omit<NumberFieldDecrementProps, "ref"> & React.RefAttributes<HTMLButtonElement>>;
export interface NumberFieldDecrementState extends NumberFieldRoot.State {}
export interface NumberFieldDecrementProps extends NativeButtonProps, BaseUIComponentProps<'button', NumberFieldDecrement.State> {}
export declare namespace NumberFieldDecrement {
  type State = NumberFieldDecrementState;
  type Props = NumberFieldDecrementProps;
}