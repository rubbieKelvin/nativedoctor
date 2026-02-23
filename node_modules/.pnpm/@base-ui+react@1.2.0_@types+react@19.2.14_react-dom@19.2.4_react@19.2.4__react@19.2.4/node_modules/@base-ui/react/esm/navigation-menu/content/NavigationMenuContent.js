'use client';

import * as React from 'react';
import * as ReactDOM from 'react-dom';
import { inertValue } from '@base-ui/utils/inertValue';
import { FloatingNode } from "../../floating-ui-react/index.js";
import { contains, getTarget } from "../../floating-ui-react/utils.js";
import { useNavigationMenuRootContext, useNavigationMenuTreeContext } from "../root/NavigationMenuRootContext.js";
import { useNavigationMenuItemContext } from "../item/NavigationMenuItemContext.js";
import { useTransitionStatus } from "../../utils/useTransitionStatus.js";
import { useOpenChangeComplete } from "../../utils/useOpenChangeComplete.js";
import { transitionStatusMapping } from "../../utils/stateAttributesMapping.js";
import { CompositeRoot } from "../../composite/root/CompositeRoot.js";
import { popupStateMapping } from "../../utils/popupStateMapping.js";
import { EMPTY_OBJECT } from "../../utils/constants.js";
import { jsx as _jsx } from "react/jsx-runtime";
const stateAttributesMapping = {
  ...popupStateMapping,
  ...transitionStatusMapping,
  activationDirection(value) {
    if (!value) {
      return null;
    }
    return {
      'data-activation-direction': value
    };
  }
};

/**
 * A container for the content of the navigation menu item that is moved into the popup
 * when the item is active.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Navigation Menu](https://base-ui.com/react/components/navigation-menu)
 */
export const NavigationMenuContent = /*#__PURE__*/React.forwardRef(function NavigationMenuContent(componentProps, forwardedRef) {
  const {
    className,
    render,
    keepMounted = false,
    ...elementProps
  } = componentProps;
  const {
    mounted: popupMounted,
    viewportElement,
    value,
    activationDirection,
    currentContentRef,
    viewportTargetElement
  } = useNavigationMenuRootContext();
  const {
    value: itemValue
  } = useNavigationMenuItemContext();
  const nodeId = useNavigationMenuTreeContext();
  const open = popupMounted && value === itemValue;
  const ref = React.useRef(null);
  const [hasMountedInPortal, setHasMountedInPortal] = React.useState(false);
  const [focusInside, setFocusInside] = React.useState(false);
  const {
    mounted,
    setMounted,
    transitionStatus
  } = useTransitionStatus(open);

  // If the popup unmounts before the content's exit animation completes, reset the internal
  // mounted state so the next open can re-enter via `transitionStatus="starting"`.
  if (mounted && !popupMounted) {
    setMounted(false);
  }
  useOpenChangeComplete({
    ref,
    open,
    onComplete() {
      if (!open) {
        setMounted(false);
      }
    }
  });
  const state = {
    open,
    transitionStatus,
    activationDirection
  };
  const handleCurrentContentRef = React.useCallback(node => {
    if (node) {
      currentContentRef.current = node;
    }
  }, [currentContentRef]);
  const commonProps = {
    onFocus(event) {
      const target = getTarget(event.nativeEvent);
      if (target?.hasAttribute('data-base-ui-focus-guard')) {
        return;
      }
      setFocusInside(true);
    },
    onBlur(event) {
      if (!contains(event.currentTarget, event.relatedTarget)) {
        setFocusInside(false);
      }
    }
  };
  const defaultProps = !open && mounted ? {
    style: {
      position: 'absolute',
      top: 0,
      left: 0
    },
    inert: inertValue(!focusInside),
    ...commonProps
  } : commonProps;
  const portalContainer = viewportTargetElement || viewportElement;
  const hidden = keepMounted && !mounted;
  const shouldRenderInline = keepMounted && !portalContainer && !hasMountedInPortal;
  if (keepMounted && portalContainer && !hasMountedInPortal) {
    setHasMountedInPortal(true);
  }
  if (shouldRenderInline) {
    return /*#__PURE__*/_jsx(CompositeRoot, {
      render: render,
      className: className,
      state: state,
      refs: [forwardedRef],
      props: [defaultProps, {
        hidden: true
      }, elementProps],
      stateAttributesMapping: stateAttributesMapping
    });
  }
  if (!portalContainer || !mounted && !keepMounted) {
    return null;
  }
  return /*#__PURE__*/ReactDOM.createPortal(/*#__PURE__*/_jsx(FloatingNode, {
    id: nodeId,
    children: /*#__PURE__*/_jsx(CompositeRoot, {
      render: render,
      className: className,
      state: state,
      refs: [forwardedRef, ref, handleCurrentContentRef],
      props: [defaultProps, hidden ? {
        hidden: true
      } : EMPTY_OBJECT, elementProps],
      stateAttributesMapping: stateAttributesMapping
    })
  }), portalContainer);
});
if (process.env.NODE_ENV !== "production") NavigationMenuContent.displayName = "NavigationMenuContent";