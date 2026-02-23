"use strict";
'use client';

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard").default;
Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.getDelay = getDelay;
exports.useHoverFloatingInteraction = useHoverFloatingInteraction;
var React = _interopRequireWildcard(require("react"));
var _dom = require("@floating-ui/utils/dom");
var _useStableCallback = require("@base-ui/utils/useStableCallback");
var _useIsoLayoutEffect = require("@base-ui/utils/useIsoLayoutEffect");
var _owner = require("@base-ui/utils/owner");
var _utils = require("../utils");
var _createBaseUIEventDetails = require("../../utils/createBaseUIEventDetails");
var _reasons = require("../../utils/reasons");
var _FloatingTree = require("../components/FloatingTree");
var _useHoverInteractionSharedState = require("./useHoverInteractionSharedState");
const clickLikeEvents = new Set(['click', 'mousedown']);

/**
 * Provides hover interactions that should be attached to the floating element.
 */
function useHoverFloatingInteraction(context, parameters = {}) {
  const store = 'rootStore' in context ? context.rootStore : context;
  const open = store.useState('open');
  const floatingElement = store.useState('floatingElement');
  const domReferenceElement = store.useState('domReferenceElement');
  const {
    dataRef
  } = store.context;
  const {
    enabled = true,
    closeDelay: closeDelayProp = 0
  } = parameters;
  const instance = (0, _useHoverInteractionSharedState.useHoverInteractionSharedState)(store);
  const tree = (0, _FloatingTree.useFloatingTree)();
  const parentId = (0, _FloatingTree.useFloatingParentNodeId)();
  const isClickLikeOpenEvent = (0, _useStableCallback.useStableCallback)(() => {
    if (instance.interactedInside) {
      return true;
    }
    return dataRef.current.openEvent ? clickLikeEvents.has(dataRef.current.openEvent.type) : false;
  });
  const isHoverOpen = (0, _useStableCallback.useStableCallback)(() => {
    const type = dataRef.current.openEvent?.type;
    return type?.includes('mouse') && type !== 'mousedown';
  });
  const isRelatedTargetInsideEnabledTrigger = (0, _useStableCallback.useStableCallback)(target => {
    return (0, _utils.isTargetInsideEnabledTrigger)(target, store.context.triggerElements);
  });
  const closeWithDelay = React.useCallback((event, runElseBranch = true) => {
    const closeDelay = getDelay(closeDelayProp, instance.pointerType);
    if (closeDelay && !instance.handler) {
      instance.openChangeTimeout.start(closeDelay, () => store.setOpen(false, (0, _createBaseUIEventDetails.createChangeEventDetails)(_reasons.REASONS.triggerHover, event)));
    } else if (runElseBranch) {
      instance.openChangeTimeout.clear();
      store.setOpen(false, (0, _createBaseUIEventDetails.createChangeEventDetails)(_reasons.REASONS.triggerHover, event));
    }
  }, [closeDelayProp, store, instance]);
  const cleanupMouseMoveHandler = (0, _useStableCallback.useStableCallback)(() => {
    instance.unbindMouseMove();
    instance.handler = undefined;
  });
  const clearPointerEvents = (0, _useStableCallback.useStableCallback)(() => {
    if (instance.performedPointerEventsMutation) {
      const body = (0, _owner.ownerDocument)(floatingElement).body;
      body.style.pointerEvents = '';
      body.removeAttribute(_useHoverInteractionSharedState.safePolygonIdentifier);
      instance.performedPointerEventsMutation = false;
    }
  });
  const handleInteractInside = (0, _useStableCallback.useStableCallback)(event => {
    const target = (0, _utils.getTarget)(event);
    if (!(0, _useHoverInteractionSharedState.isInteractiveElement)(target)) {
      instance.interactedInside = false;
      return;
    }
    instance.interactedInside = true;
  });
  (0, _useIsoLayoutEffect.useIsoLayoutEffect)(() => {
    if (!open) {
      instance.pointerType = undefined;
      instance.restTimeoutPending = false;
      instance.interactedInside = false;
      cleanupMouseMoveHandler();
      clearPointerEvents();
    }
  }, [open, instance, cleanupMouseMoveHandler, clearPointerEvents]);
  React.useEffect(() => {
    return () => {
      cleanupMouseMoveHandler();
    };
  }, [cleanupMouseMoveHandler]);
  React.useEffect(() => {
    return clearPointerEvents;
  }, [clearPointerEvents]);
  (0, _useIsoLayoutEffect.useIsoLayoutEffect)(() => {
    if (!enabled) {
      return undefined;
    }
    if (open && instance.handleCloseOptions?.blockPointerEvents && isHoverOpen() && (0, _dom.isElement)(domReferenceElement) && floatingElement) {
      instance.performedPointerEventsMutation = true;
      const body = (0, _owner.ownerDocument)(floatingElement).body;
      body.setAttribute(_useHoverInteractionSharedState.safePolygonIdentifier, '');
      const ref = domReferenceElement;
      const floatingEl = floatingElement;
      const parentFloating = tree?.nodesRef.current.find(node => node.id === parentId)?.context?.elements.floating;
      if (parentFloating) {
        parentFloating.style.pointerEvents = '';
      }
      body.style.pointerEvents = 'none';
      ref.style.pointerEvents = 'auto';
      floatingEl.style.pointerEvents = 'auto';
      return () => {
        body.style.pointerEvents = '';
        ref.style.pointerEvents = '';
        floatingEl.style.pointerEvents = '';
      };
    }
    return undefined;
  }, [enabled, open, domReferenceElement, floatingElement, instance, isHoverOpen, tree, parentId]);
  React.useEffect(() => {
    if (!enabled) {
      return undefined;
    }

    // Ensure the floating element closes after scrolling even if the pointer
    // did not move.
    // https://github.com/floating-ui/floating-ui/discussions/1692
    function onScrollMouseLeave(event) {
      if (isClickLikeOpenEvent() || !dataRef.current.floatingContext || !store.select('open')) {
        return;
      }
      if (isRelatedTargetInsideEnabledTrigger(event.relatedTarget)) {
        // If the mouse is leaving the reference element to another trigger, don't explicitly close the popup
        // as it will be moved.
        return;
      }
      clearPointerEvents();
      cleanupMouseMoveHandler();
      if (!isClickLikeOpenEvent()) {
        closeWithDelay(event);
      }
    }
    function onFloatingMouseEnter(event) {
      instance.openChangeTimeout.clear();
      clearPointerEvents();
      instance.handler?.(event);
      cleanupMouseMoveHandler();
    }
    function onFloatingMouseLeave(event) {
      if (!isClickLikeOpenEvent()) {
        closeWithDelay(event, false);
      }
    }
    const floating = floatingElement;
    if (floating) {
      floating.addEventListener('mouseleave', onScrollMouseLeave);
      floating.addEventListener('mouseenter', onFloatingMouseEnter);
      floating.addEventListener('mouseleave', onFloatingMouseLeave);
      floating.addEventListener('pointerdown', handleInteractInside, true);
    }
    return () => {
      if (floating) {
        floating.removeEventListener('mouseleave', onScrollMouseLeave);
        floating.removeEventListener('mouseenter', onFloatingMouseEnter);
        floating.removeEventListener('mouseleave', onFloatingMouseLeave);
        floating.removeEventListener('pointerdown', handleInteractInside, true);
      }
    };
  }, [enabled, floatingElement, store, dataRef, isClickLikeOpenEvent, isRelatedTargetInsideEnabledTrigger, closeWithDelay, clearPointerEvents, cleanupMouseMoveHandler, handleInteractInside, instance]);
}
function getDelay(value, pointerType) {
  if (pointerType && !(0, _utils.isMouseLikePointerType)(pointerType)) {
    return 0;
  }
  if (typeof value === 'function') {
    return value();
  }
  return value;
}