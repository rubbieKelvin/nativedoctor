import * as React from 'react';
import { BaseUIComponentProps, NativeButtonProps } from "../../utils/types.js";
import type { ToolbarRoot } from "../root/ToolbarRoot.js";
/**
 * A button that can be used as-is or as a trigger for other components.
 * Renders a `<button>` element.
 *
 * Documentation: [Base UI Toolbar](https://base-ui.com/react/components/toolbar)
 */
export declare const ToolbarButton: React.ForwardRefExoticComponent<Omit<ToolbarButtonProps, "ref"> & React.RefAttributes<HTMLButtonElement>>;
export interface ToolbarButtonState extends ToolbarRoot.State {
  disabled: boolean;
  focusable: boolean;
}
export interface ToolbarButtonProps extends NativeButtonProps, BaseUIComponentProps<'button', ToolbarButton.State> {
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
}
export declare namespace ToolbarButton {
  type State = ToolbarButtonState;
  type Props = ToolbarButtonProps;
}