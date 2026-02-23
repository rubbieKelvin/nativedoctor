import * as React from 'react';
export declare function useButton(parameters?: useButton.Parameters): useButton.ReturnValue;
export interface UseButtonParameters {
  /**
   * Whether the component should ignore user interaction.
   * @default false
   */
  disabled?: boolean | undefined;
  /**
   * Whether the button may receive focus even if it is disabled.
   * @default false
   */
  focusableWhenDisabled?: boolean | undefined;
  tabIndex?: NonNullable<React.HTMLAttributes<any>['tabIndex']> | undefined;
  /**
   * Whether the component is being rendered as a native button.
   * @default true
   */
  native?: boolean | undefined;
}
export interface UseButtonReturnValue {
  /**
   * Resolver for the button props.
   * @param externalProps additional props for the button
   * @returns props that should be spread on the button
   */
  getButtonProps: (externalProps?: React.ComponentPropsWithRef<any>) => React.ComponentPropsWithRef<any>;
  /**
   * A ref to the button DOM element. This ref should be passed to the rendered element.
   * It is not a part of the props returned by `getButtonProps`.
   */
  buttonRef: React.Ref<HTMLElement>;
}
export declare namespace useButton {
  type Parameters = UseButtonParameters;
  type ReturnValue = UseButtonReturnValue;
}