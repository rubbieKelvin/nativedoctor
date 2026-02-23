import * as React from 'react';
export declare function useFocusableWhenDisabled(parameters: useFocusableWhenDisabled.Parameters): useFocusableWhenDisabled.ReturnValue;
interface FocusableWhenDisabledProps {
  'aria-disabled'?: boolean | undefined;
  disabled?: boolean | undefined;
  onKeyDown: (event: React.KeyboardEvent) => void;
  tabIndex: number;
}
export interface UseFocusableWhenDisabledParameters {
  /**
   * Whether the component should be focusable when disabled.
   * When `undefined`, composite items are focusable when disabled by default.
   */
  focusableWhenDisabled?: boolean | undefined;
  /**
   * The disabled state of the component.
   */
  disabled: boolean;
  /**
   * Whether this is a composite item or not.
   * @default false
   */
  composite?: boolean | undefined;
  /**
   * @default 0
   */
  tabIndex?: number | undefined;
  /**
   * @default true
   */
  isNativeButton: boolean;
}
export interface UseFocusableWhenDisabledReturnValue {
  props: FocusableWhenDisabledProps;
}
export declare namespace useFocusableWhenDisabled {
  type Parameters = UseFocusableWhenDisabledParameters;
  type ReturnValue = UseFocusableWhenDisabledReturnValue;
}
export {};