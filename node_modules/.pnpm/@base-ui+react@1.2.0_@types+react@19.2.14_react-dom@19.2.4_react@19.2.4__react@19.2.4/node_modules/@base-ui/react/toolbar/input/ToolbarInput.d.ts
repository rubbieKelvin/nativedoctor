import * as React from 'react';
import { BaseUIComponentProps } from "../../utils/types.js";
import type { ToolbarRoot } from "../root/ToolbarRoot.js";
/**
 * A native input element that integrates with Toolbar keyboard navigation.
 * Renders an `<input>` element.
 *
 * Documentation: [Base UI Toolbar](https://base-ui.com/react/components/toolbar)
 */
export declare const ToolbarInput: React.ForwardRefExoticComponent<Omit<ToolbarInputProps, "ref"> & React.RefAttributes<HTMLInputElement>>;
export interface ToolbarInputState extends ToolbarRoot.State {
  disabled: boolean;
  focusable: boolean;
}
export interface ToolbarInputProps extends BaseUIComponentProps<'input', ToolbarInput.State> {
  /**
   * When `true` the item is disabled.
   * @default false
   */
  disabled?: boolean | undefined;
  /**
   * When `true` the item remains focuseable when disabled.
   * @default true
   */
  focusableWhenDisabled?: boolean | undefined;
  defaultValue?: React.ComponentProps<'input'>['defaultValue'] | undefined;
}
export declare namespace ToolbarInput {
  type State = ToolbarInputState;
  type Props = ToolbarInputProps;
}