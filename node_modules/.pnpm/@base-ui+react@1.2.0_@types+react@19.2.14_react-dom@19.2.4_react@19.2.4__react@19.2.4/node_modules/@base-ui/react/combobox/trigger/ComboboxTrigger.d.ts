import * as React from 'react';
import { BaseUIComponentProps, NativeButtonProps } from "../../utils/types.js";
import type { FieldRoot } from "../../field/root/FieldRoot.js";
import type { Side } from "../../utils/useAnchorPositioning.js";
/**
 * A button that opens the popup.
 * Renders a `<button>` element.
 */
export declare const ComboboxTrigger: React.ForwardRefExoticComponent<Omit<ComboboxTriggerProps, "ref"> & React.RefAttributes<HTMLButtonElement>>;
export interface ComboboxTriggerState extends FieldRoot.State {
  /**
   * Whether the popup is open.
   */
  open: boolean;
  /**
   * Whether the component should ignore user interaction.
   */
  disabled: boolean;
  /**
   * Indicates which side the corresponding popup is positioned relative to its anchor.
   */
  popupSide: Side | null;
  /**
   * Present when the corresponding items list is empty.
   */
  listEmpty: boolean;
  /**
   * Whether the combobox doesn't have a value.
   */
  placeholder: boolean;
}
export interface ComboboxTriggerProps extends NativeButtonProps, BaseUIComponentProps<'button', ComboboxTrigger.State> {
  /**
   * Whether the component should ignore user interaction.
   * @default false
   */
  disabled?: boolean | undefined;
}
export declare namespace ComboboxTrigger {
  type State = ComboboxTriggerState;
  type Props = ComboboxTriggerProps;
}