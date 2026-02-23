'use client';

import * as React from 'react';
import { isHTMLElement } from '@floating-ui/utils/dom';
import { useStableCallback } from '@base-ui/utils/useStableCallback';
import { error } from '@base-ui/utils/error';
import { SafeReact } from '@base-ui/utils/safeReact';
import { useIsoLayoutEffect } from '@base-ui/utils/useIsoLayoutEffect';
import { makeEventPreventable, mergeProps } from "../merge-props/index.js";
import { useCompositeRootContext } from "../composite/root/CompositeRootContext.js";
import { useFocusableWhenDisabled } from "../utils/useFocusableWhenDisabled.js";
export function useButton(parameters = {}) {
  const {
    disabled = false,
    focusableWhenDisabled,
    tabIndex = 0,
    native: isNativeButton = true
  } = parameters;
  const elementRef = React.useRef(null);
  const isCompositeItem = useCompositeRootContext(true) !== undefined;
  const isValidLink = useStableCallback(() => {
    const element = elementRef.current;
    return Boolean(element?.tagName === 'A' && element?.href);
  });
  const {
    props: focusableWhenDisabledProps
  } = useFocusableWhenDisabled({
    focusableWhenDisabled,
    disabled,
    composite: isCompositeItem,
    tabIndex,
    isNativeButton
  });
  if (process.env.NODE_ENV !== 'production') {
    // eslint-disable-next-line react-hooks/rules-of-hooks
    React.useEffect(() => {
      if (!elementRef.current) {
        return;
      }
      const isButtonTag = elementRef.current.tagName === 'BUTTON';
      if (isNativeButton) {
        if (!isButtonTag) {
          const ownerStackMessage = SafeReact.captureOwnerStack?.() || '';
          const message = 'A component that acts as a button expected a native <button> because the ' + '`nativeButton` prop is true. Rendering a non-<button> removes native button ' + 'semantics, which can impact forms and accessibility. Use a real <button> in the ' + '`render` prop, or set `nativeButton` to `false`.';
          error(`${message}${ownerStackMessage}`);
        }
      } else if (isButtonTag) {
        const ownerStackMessage = SafeReact.captureOwnerStack?.() || '';
        const message = 'A component that acts as a button expected a non-<button> because the `nativeButton` ' + 'prop is false. Rendering a <button> keeps native behavior while Base UI applies ' + 'non-native attributes and handlers, which can add unintended extra attributes (such ' + 'as `role` or `aria-disabled`). Use a non-<button> in the `render` prop, or set ' + '`nativeButton` to `true`.';
        error(`${message}${ownerStackMessage}`);
      }
    }, [isNativeButton]);
  }

  // handles a disabled composite button rendering another button, e.g.
  // <Toolbar.Button disabled render={<Menu.Trigger />} />
  // the `disabled` prop needs to pass through 2 `useButton`s then finally
  // delete the `disabled` attribute from DOM
  const updateDisabled = React.useCallback(() => {
    const element = elementRef.current;
    if (!isButtonElement(element)) {
      return;
    }
    if (isCompositeItem && disabled && focusableWhenDisabledProps.disabled === undefined && element.disabled) {
      element.disabled = false;
    }
  }, [disabled, focusableWhenDisabledProps.disabled, isCompositeItem]);
  useIsoLayoutEffect(updateDisabled, [updateDisabled]);
  const getButtonProps = React.useCallback((externalProps = {}) => {
    const {
      onClick: externalOnClick,
      onMouseDown: externalOnMouseDown,
      onKeyUp: externalOnKeyUp,
      onKeyDown: externalOnKeyDown,
      onPointerDown: externalOnPointerDown,
      ...otherExternalProps
    } = externalProps;
    const type = isNativeButton ? 'button' : undefined;
    return mergeProps({
      type,
      onClick(event) {
        if (disabled) {
          event.preventDefault();
          return;
        }
        externalOnClick?.(event);
      },
      onMouseDown(event) {
        if (!disabled) {
          externalOnMouseDown?.(event);
        }
      },
      onKeyDown(event) {
        if (!disabled) {
          makeEventPreventable(event);
          externalOnKeyDown?.(event);
        }
        if (event.baseUIHandlerPrevented) {
          return;
        }
        const shouldClick = event.target === event.currentTarget && !isNativeButton && !isValidLink() && !disabled;
        const isEnterKey = event.key === 'Enter';
        const isSpaceKey = event.key === ' ';

        // Keyboard accessibility for non interactive elements
        if (shouldClick) {
          if (isSpaceKey || isEnterKey) {
            event.preventDefault();
          }
          if (isEnterKey) {
            externalOnClick?.(event);
          }
        }
      },
      onKeyUp(event) {
        // calling preventDefault in keyUp on a <button> will not dispatch a click event if Space is pressed
        // https://codesandbox.io/p/sandbox/button-keyup-preventdefault-dn7f0
        // Keyboard accessibility for non interactive elements
        if (!disabled) {
          makeEventPreventable(event);
          externalOnKeyUp?.(event);
        }
        if (event.baseUIHandlerPrevented) {
          return;
        }
        if (event.target === event.currentTarget && !isNativeButton && !disabled && event.key === ' ') {
          externalOnClick?.(event);
        }
      },
      onPointerDown(event) {
        if (disabled) {
          event.preventDefault();
          return;
        }
        externalOnPointerDown?.(event);
      }
    }, !isNativeButton ? {
      role: 'button'
    } : undefined, focusableWhenDisabledProps, otherExternalProps);
  }, [disabled, focusableWhenDisabledProps, isNativeButton, isValidLink]);
  const buttonRef = useStableCallback(element => {
    elementRef.current = element;
    updateDisabled();
  });
  return {
    getButtonProps,
    buttonRef
  };
}
function isButtonElement(elem) {
  return isHTMLElement(elem) && elem.tagName === 'BUTTON';
}