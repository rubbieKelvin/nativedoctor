import * as React from 'react';
import type { BaseUIComponentProps } from "../../utils/types.js";
import type { NumberFieldRoot } from "../root/NumberFieldRoot.js";
/**
 * The native input control in the number field.
 * Renders an `<input>` element.
 *
 * Documentation: [Base UI Number Field](https://base-ui.com/react/components/number-field)
 */
export declare const NumberFieldInput: React.ForwardRefExoticComponent<Omit<NumberFieldInputProps, "ref"> & React.RefAttributes<HTMLInputElement>>;
export interface NumberFieldInputState extends NumberFieldRoot.State {}
export interface NumberFieldInputProps extends BaseUIComponentProps<'input', NumberFieldInput.State> {
  /**
   * A string value that provides a user-friendly name for the role of the input.
   * @default 'Number field'
   */
  'aria-roledescription'?: React.AriaAttributes['aria-roledescription'] | undefined;
}
export declare namespace NumberFieldInput {
  type State = NumberFieldInputState;
  type Props = NumberFieldInputProps;
}