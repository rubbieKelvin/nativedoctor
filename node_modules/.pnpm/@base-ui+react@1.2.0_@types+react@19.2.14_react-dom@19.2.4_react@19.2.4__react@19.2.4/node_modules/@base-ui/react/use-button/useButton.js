"use strict";
'use client';

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard").default;
Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.useButton = useButton;
var React = _interopRequireWildcard(require("react"));
var _dom = require("@floating-ui/utils/dom");
var _useStableCallback = require("@base-ui/utils/useStableCallback");
var _error = require("@base-ui/utils/error");
var _safeReact = require("@base-ui/utils/safeReact");
var _useIsoLayoutEffect = require("@base-ui/utils/useIsoLayoutEffect");
var _mergeProps = require("../merge-props");
var _CompositeRootContext = require("../composite/root/CompositeRootContext");
var _useFocusableWhenDisabled = require("../utils/useFocusableWhenDisabled");
function useButton(parameters = {}) {
  const {
    disabled = false,
    focusableWhenDisabled,
    tabIndex = 0,
    native: isNativeButton = true
  } = parameters;
  const elementRef = React.useRef(null);
  const isCompositeItem = (0, _CompositeRootContext.useCompositeRootContext)(true) !== undefined;
  const isValidLink = (0, _useStableCallback.useStableCallback)(() => {
    const element = elementRef.current;
    return Boolean(element?.tagName === 'A' && element?.href);
  });
  const {
    props: focusableWhenDisabledProps
  } = (0, _useFocusableWhenDisabled.useFocusableWhenDisabled)({
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
          const ownerStackMessage = _safeReact.SafeReact.captureOwnerStack?.() || '';
          const message = 'A component that acts as a button expected a native <button> because the ' + '`nativeButton` prop is true. Rendering a non-<button> removes native button ' + 'semantics, which can impact forms and accessibility. Use a real <button> in the ' + '`render` prop, or set `nativeButton` to `false`.';
          (0, _error.error)(`${message}${ownerStackMessage}`);
        }
      } else if (isButtonTag) {
        const ownerStackMessage = _safeReact.SafeReact.captureOwnerStack?.() || '';
        const message = 'A component that acts as a button expected a non-<button> because the `nativeButton` ' + 'prop is false. Rendering a <button> keeps native behavior while Base UI applies ' + 'non-native attributes and handlers, which can add unintended extra attributes (such ' + 'as `role` or `aria-disabled`). Use a non-<button> in the `render` prop, or set ' + '`nativeButton` to `true`.';
        (0, _error.error)(`${message}${ownerStackMessage}`);
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
  (0, _useIsoLayoutEffect.useIsoLayoutEffect)(updateDisabled, [updateDisabled]);
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
    return (0, _mergeProps.mergeProps)({
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
          (0, _mergeProps.makeEventPreventable)(event);
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
          (0, _mergeProps.makeEventPreventable)(event);
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
  const buttonRef = (0, _useStableCallback.useStableCallback)(element => {
    elementRef.current = element;
    updateDisabled();
  });
  return {
    getButtonProps,
    buttonRef
  };
}
function isButtonElement(elem) {
  return (0, _dom.isHTMLElement)(elem) && elem.tagName === 'BUTTON';
}