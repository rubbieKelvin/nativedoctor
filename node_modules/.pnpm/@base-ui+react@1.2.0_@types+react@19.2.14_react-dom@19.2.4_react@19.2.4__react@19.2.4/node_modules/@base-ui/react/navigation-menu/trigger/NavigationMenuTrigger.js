"use strict";
'use client';

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard").default;
Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.NavigationMenuTrigger = void 0;
var React = _interopRequireWildcard(require("react"));
var ReactDOM = _interopRequireWildcard(require("react-dom"));
var _tabbable = require("tabbable");
var _useIsoLayoutEffect = require("@base-ui/utils/useIsoLayoutEffect");
var _useTimeout = require("@base-ui/utils/useTimeout");
var _useAnimationFrame = require("@base-ui/utils/useAnimationFrame");
var _floatingUiReact = require("../../floating-ui-react");
var _utils = require("../../floating-ui-react/utils");
var _NavigationMenuItemContext = require("../item/NavigationMenuItemContext");
var _NavigationMenuRootContext = require("../root/NavigationMenuRootContext");
var _createBaseUIEventDetails = require("../../utils/createBaseUIEventDetails");
var _reasons = require("../../utils/reasons");
var _constants = require("../../utils/constants");
var _FocusGuard = require("../../utils/FocusGuard");
var _popupStateMapping = require("../../utils/popupStateMapping");
var _isOutsideMenuEvent = require("../utils/isOutsideMenuEvent");
var _CompositeItem = require("../../composite/item/CompositeItem");
var _useButton = require("../../use-button");
var _constants2 = require("../utils/constants");
var _NavigationMenuDismissContext = require("../list/NavigationMenuDismissContext");
var _jsxRuntime = require("react/jsx-runtime");
/**
 * Opens the navigation menu popup when hovered or clicked, revealing the
 * associated content.
 * Renders a `<button>` element.
 *
 * Documentation: [Base UI Navigation Menu](https://base-ui.com/react/components/navigation-menu)
 */
const NavigationMenuTrigger = exports.NavigationMenuTrigger = /*#__PURE__*/React.forwardRef(function NavigationMenuTrigger(componentProps, forwardedRef) {
  const {
    className,
    render,
    nativeButton = true,
    disabled,
    ...elementProps
  } = componentProps;
  const {
    value,
    setValue,
    mounted,
    open,
    positionerElement,
    setActivationDirection,
    setFloatingRootContext,
    popupElement,
    viewportElement,
    rootRef,
    beforeOutsideRef,
    afterOutsideRef,
    afterInsideRef,
    beforeInsideRef,
    prevTriggerElementRef,
    delay,
    closeDelay,
    orientation,
    setViewportInert,
    nested
  } = (0, _NavigationMenuRootContext.useNavigationMenuRootContext)();
  const {
    value: itemValue
  } = (0, _NavigationMenuItemContext.useNavigationMenuItemContext)();
  const nodeId = (0, _NavigationMenuRootContext.useNavigationMenuTreeContext)();
  const tree = (0, _floatingUiReact.useFloatingTree)();
  const dismissProps = (0, _NavigationMenuDismissContext.useNavigationMenuDismissContext)();
  const stickIfOpenTimeout = (0, _useTimeout.useTimeout)();
  const focusFrame = (0, _useAnimationFrame.useAnimationFrame)();
  const [triggerElement, setTriggerElement] = React.useState(null);
  const [stickIfOpen, setStickIfOpen] = React.useState(true);
  const [pointerType, setPointerType] = React.useState('');
  const allowFocusRef = React.useRef(false);
  const isActiveItem = open && value === itemValue;
  const interactionsEnabled = positionerElement ? true : !value;
  React.useEffect(() => {
    if (!open) {
      stickIfOpenTimeout.clear();
    }
  }, [stickIfOpenTimeout, open]);
  React.useEffect(() => {
    if (isActiveItem && open && popupElement && allowFocusRef.current) {
      allowFocusRef.current = false;
      focusFrame.request(() => {
        beforeOutsideRef.current?.focus();
      });
    }
    return () => {
      focusFrame.cancel();
    };
  }, [beforeOutsideRef, focusFrame, isActiveItem, open, popupElement]);
  function handleOpenChange(nextOpen, eventDetails) {
    const isHover = eventDetails.reason === _reasons.REASONS.triggerHover;
    if (!interactionsEnabled) {
      return;
    }
    if (pointerType === 'touch' && isHover) {
      return;
    }
    if (!nextOpen && value !== itemValue) {
      return;
    }
    function changeState() {
      if (isHover) {
        // Only allow "patient" clicks to close the popup if it's open.
        // If they clicked within 500ms of the popup opening, keep it open.
        setStickIfOpen(true);
        stickIfOpenTimeout.clear();
        stickIfOpenTimeout.start(_constants.PATIENT_CLICK_THRESHOLD, () => {
          setStickIfOpen(false);
        });
      }
      if (nextOpen) {
        setValue(itemValue, eventDetails);
      } else {
        setValue(null, eventDetails);
        setPointerType('');
      }
    }
    if (isHover) {
      ReactDOM.flushSync(changeState);
    } else {
      changeState();
    }
  }
  const context = (0, _floatingUiReact.useFloatingRootContext)({
    open,
    onOpenChange: handleOpenChange,
    elements: {
      reference: triggerElement,
      floating: positionerElement || viewportElement
    }
  });
  const hover = (0, _floatingUiReact.useHover)(context, {
    move: false,
    handleClose: (0, _floatingUiReact.safePolygon)({
      blockPointerEvents: pointerType !== 'touch'
    }),
    restMs: mounted && positionerElement ? 0 : delay,
    delay: {
      close: closeDelay
    }
  });
  const click = (0, _floatingUiReact.useClick)(context, {
    enabled: interactionsEnabled,
    stickIfOpen,
    toggle: isActiveItem
  });
  (0, _useIsoLayoutEffect.useIsoLayoutEffect)(() => {
    if (isActiveItem) {
      setFloatingRootContext(context);
      prevTriggerElementRef.current = triggerElement;
    }
  }, [isActiveItem, context, setFloatingRootContext, prevTriggerElementRef, triggerElement]);
  const {
    getReferenceProps
  } = (0, _floatingUiReact.useInteractions)([hover, click]);
  function handleActivation(event) {
    ReactDOM.flushSync(() => {
      const prevTriggerRect = prevTriggerElementRef.current?.getBoundingClientRect();
      if (mounted && prevTriggerRect && triggerElement) {
        const nextTriggerRect = triggerElement.getBoundingClientRect();
        const isMovingRight = nextTriggerRect.left > prevTriggerRect.left;
        const isMovingDown = nextTriggerRect.top > prevTriggerRect.top;
        if (orientation === 'horizontal' && nextTriggerRect.left !== prevTriggerRect.left) {
          setActivationDirection(isMovingRight ? 'right' : 'left');
        } else if (orientation === 'vertical' && nextTriggerRect.top !== prevTriggerRect.top) {
          setActivationDirection(isMovingDown ? 'down' : 'up');
        }
      }

      // Reset the `openEvent` to `undefined` when the active item changes so that a
      // `click` -> `hover` on new trigger -> `hover` back to old trigger doesn't unexpectedly
      // cause the popup to remain stuck open when leaving the old trigger.
      if (event.type !== 'click') {
        context.context.dataRef.current.openEvent = undefined;
      }
      if (pointerType === 'touch' && event.type !== 'click') {
        return;
      }
      if (value != null) {
        setValue(itemValue, (0, _createBaseUIEventDetails.createChangeEventDetails)(event.type === 'mouseenter' ? _reasons.REASONS.triggerHover : _reasons.REASONS.triggerPress, event.nativeEvent));
      }
    });
  }
  const state = {
    open: isActiveItem
  };
  function handleSetPointerType(event) {
    setPointerType(event.pointerType);
  }
  const defaultProps = {
    tabIndex: 0,
    onMouseEnter: handleActivation,
    onClick: handleActivation,
    onPointerEnter: handleSetPointerType,
    onPointerDown: handleSetPointerType,
    'aria-expanded': isActiveItem,
    'aria-controls': isActiveItem ? popupElement?.id : undefined,
    [_constants2.NAVIGATION_MENU_TRIGGER_IDENTIFIER]: '',
    onFocus() {
      if (!isActiveItem) {
        return;
      }
      setViewportInert(false);
    },
    onMouseMove() {
      allowFocusRef.current = false;
    },
    onKeyDown(event) {
      allowFocusRef.current = true;

      // For nested (submenu) triggers, don't intercept arrow keys that are used for
      // navigation in the parent content. The arrow keys should be handled by the
      // parent's CompositeRoot for navigating between items.
      if (nested) {
        return;
      }
      const openHorizontal = orientation === 'horizontal' && event.key === 'ArrowDown';
      const openVertical = orientation === 'vertical' && event.key === 'ArrowRight';
      if (openHorizontal || openVertical) {
        setValue(itemValue, (0, _createBaseUIEventDetails.createChangeEventDetails)(_reasons.REASONS.listNavigation, event.nativeEvent));
        handleActivation(event);
        (0, _utils.stopEvent)(event);
      }
    },
    onBlur(event) {
      if (positionerElement && popupElement && (0, _isOutsideMenuEvent.isOutsideMenuEvent)({
        currentTarget: event.currentTarget,
        relatedTarget: event.relatedTarget
      }, {
        popupElement,
        rootRef,
        tree,
        nodeId
      })) {
        setValue(null, (0, _createBaseUIEventDetails.createChangeEventDetails)(_reasons.REASONS.focusOut, event.nativeEvent));
      }
    }
  };
  const {
    getButtonProps,
    buttonRef
  } = (0, _useButton.useButton)({
    disabled,
    focusableWhenDisabled: true,
    native: nativeButton
  });
  const referenceElement = positionerElement || viewportElement;
  return /*#__PURE__*/(0, _jsxRuntime.jsxs)(React.Fragment, {
    children: [/*#__PURE__*/(0, _jsxRuntime.jsx)(_CompositeItem.CompositeItem, {
      tag: "button",
      render: render,
      className: className,
      state: state,
      stateAttributesMapping: _popupStateMapping.pressableTriggerOpenStateMapping,
      refs: [forwardedRef, setTriggerElement, buttonRef],
      props: [getReferenceProps, dismissProps?.reference || _constants.EMPTY_ARRAY, defaultProps, elementProps, getButtonProps]
    }), isActiveItem && /*#__PURE__*/(0, _jsxRuntime.jsxs)(React.Fragment, {
      children: [/*#__PURE__*/(0, _jsxRuntime.jsx)(_FocusGuard.FocusGuard, {
        ref: beforeOutsideRef,
        onFocus: event => {
          if (referenceElement && (0, _utils.isOutsideEvent)(event, referenceElement)) {
            beforeInsideRef.current?.focus();
          } else {
            const prevTabbable = (0, _utils.getPreviousTabbable)(triggerElement);
            prevTabbable?.focus();
          }
        }
      }), /*#__PURE__*/(0, _jsxRuntime.jsx)("span", {
        "aria-owns": viewportElement?.id,
        style: _constants.ownerVisuallyHidden
      }), /*#__PURE__*/(0, _jsxRuntime.jsx)(_FocusGuard.FocusGuard, {
        ref: afterOutsideRef,
        onFocus: event => {
          if (referenceElement && (0, _utils.isOutsideEvent)(event, referenceElement)) {
            const elementToFocus = afterInsideRef.current && (0, _tabbable.isTabbable)(afterInsideRef.current) ? afterInsideRef.current : triggerElement;
            elementToFocus?.focus();
          } else {
            const nextTabbable = (0, _utils.getNextTabbable)(triggerElement);
            nextTabbable?.focus();
            if (!(0, _utils.contains)(rootRef.current, nextTabbable)) {
              setValue(null, (0, _createBaseUIEventDetails.createChangeEventDetails)(_reasons.REASONS.focusOut, event.nativeEvent));
            }
          }
        }
      })]
    })]
  });
});
if (process.env.NODE_ENV !== "production") NavigationMenuTrigger.displayName = "NavigationMenuTrigger";