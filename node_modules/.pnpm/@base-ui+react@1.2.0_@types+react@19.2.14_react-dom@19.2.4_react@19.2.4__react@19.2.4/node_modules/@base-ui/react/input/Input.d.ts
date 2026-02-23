import * as React from 'react';
import type { BaseUIComponentProps } from "../utils/types.js";
import { Field } from "../field/index.js";
/**
 * A native input element that automatically works with [Field](https://base-ui.com/react/components/field).
 * Renders an `<input>` element.
 *
 * Documentation: [Base UI Input](https://base-ui.com/react/components/input)
 */
export declare const Input: React.ForwardRefExoticComponent<Omit<InputProps, "ref"> & React.RefAttributes<HTMLElement>>;
export interface InputProps extends BaseUIComponentProps<'input', Input.State> {
  /**
   * Callback fired when the `value` changes. Use when controlled.
   */
  onValueChange?: Field.Control.Props['onValueChange'] | undefined;
  defaultValue?: Field.Control.Props['defaultValue'] | undefined;
}
export interface InputState extends Field.Control.State {}
export type InputChangeEventReason = Field.Control.ChangeEventReason;
export type InputChangeEventDetails = Field.Control.ChangeEventDetails;
export declare namespace Input {
  type Props = InputProps;
  type State = InputState;
  type ChangeEventReason = InputChangeEventReason;
  type ChangeEventDetails = InputChangeEventDetails;
}