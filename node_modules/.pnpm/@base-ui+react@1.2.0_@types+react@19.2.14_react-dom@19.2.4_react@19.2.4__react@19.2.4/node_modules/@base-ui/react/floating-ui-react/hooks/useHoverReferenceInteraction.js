"use strict";
'use client';

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard").default;
Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.useHoverReferenceInteraction = useHoverReferenceInteraction;
var React = _interopRequireWildcard(require("react"));
var ReactDOM = _interopRequireWildcard(require("react-dom"));
var _dom = require("@floating-ui/utils/dom");
var _useValueAsRef = require("@base-ui/utils/useValueAsRef");
var _useStableCallback = require("@base-ui/utils/useStableCallback");
var _owner = require("@base-ui/utils/owner");
var _utils = require("../utils");
var _createBaseUIEventDetails = require("../../utils/createBaseUIEventDetails");
var _reasons = require("../../utils/reasons");
var _useHover = require("./useHover");
var _FloatingTree = require("../components/FloatingTree");
var _useHoverInteractionSharedState = require("./useHoverInteractionSharedState");
function getRestMs(value) {
  if (typeof value === 'function') {
    return value();
  }
  return value;
}
const EMPTY_REF = {
  current: null
};

/**
 * Provides hover interactions that should be attached to reference or trigger
 * elements.
 */
function useHoverReferenceInteraction(context, props = {}) {
  const store = 'rootStore' in context ? context.rootStore : context;
  const {
    dataRef,
    events
  } = store.context;
  const {
    enabled = true,
    delay = 0,
    handleClose = null,
    mouseOnly = false,
    restMs = 0,
    move = true,
    triggerElementRef = EMPTY_REF,
    externalTree,
    isActiveTrigger = true
  } = props;
  const tree = (0, _FloatingTree.useFloatingTree)(externalTree);
  const instance = (0, _useHoverInteractionSharedState.useHoverInteractionSharedState)(store);
  const handleCloseRef = (0, _useValueAsRef.useValueAsRef)(handleClose);
  const delayRef = (0, _useValueAsRef.useValueAsRef)(delay);
  const restMsRef = (0, _useValueAsRef.useValueAsRef)(restMs);
  const enabledRef = (0, _useValueAsRef.useValueAsRef)(enabled);
  if (isActiveTrigger) {
    // eslint-disable-next-line no-underscore-dangle
    instance.handleCloseOptions = handleCloseRef.current?.__options;
  }
  const isClickLikeOpenEvent = (0, _useStableCallback.useStableCallback)(() => {
    if (instance.interactedInside) {
      return true;
    }
    return dataRef.current.openEvent ? ['click', 'mousedown'].includes(dataRef.current.openEvent.type) : false;
  });
  const isRelatedTargetInsideEnabledTrigger = (0, _useStableCallback.useStableCallback)(target => {
    return (0, _utils.isTargetInsideEnabledTrigger)(target, store.context.triggerElements);
  });
  const closeWithDelay = React.useCallback((event, runElseBranch = true) => {
    const closeDelay = (0, _useHover.getDelay)(delayRef.current, 'close', instance.pointerType);
    if (closeDelay && !instance.handler) {
      instance.openChangeTimeout.start(closeDelay, () => store.setOpen(false, (0, _createBaseUIEventDetails.createChangeEventDetails)(_reasons.REASONS.triggerHover, event)));
    } else if (runElseBranch) {
      instance.openChangeTimeout.clear();
      store.setOpen(false, (0, _createBaseUIEventDetails.createChangeEventDetails)(_reasons.REASONS.triggerHover, event));
    }
  }, [delayRef, store, instance]);
  const cleanupMouseMoveHandler = (0, _useStableCallback.useStableCallback)(() => {
    instance.unbindMouseMove();
    instance.handler = undefined;
  });
  const clearPointerEvents = (0, _useStableCallback.useStableCallback)(() => {
    if (instance.performedPointerEventsMutation) {
      const body = (0, _owner.ownerDocument)(store.select('domReferenceElement')).body;
      body.style.pointerEvents = '';
      body.removeAttribute(_useHoverInteractionSharedState.safePolygonIdentifier);
      instance.performedPointerEventsMutation = false;
    }
  });

  // When closing before opening, clear the delay timeouts to cancel it
  // from showing.
  React.useEffect(() => {
    if (!enabled) {
      return undefined;
    }
    function onOpenChangeLocal(details) {
      if (!details.open) {
        instance.openChangeTimeout.clear();
        instance.restTimeout.clear();
        instance.blockMouseMove = true;
        instance.restTimeoutPending = false;
      }
    }
    events.on('openchange', onOpenChangeLocal);
    return () => {
      events.off('openchange', onOpenChangeLocal);
    };
  }, [enabled, events, instance]);
  const handleScrollMouseLeave = (0, _useStableCallback.useStableCallback)(event => {
    if (isClickLikeOpenEvent()) {
      return;
    }
    if (!dataRef.current.floatingContext) {
      return;
    }
    if (isRelatedTargetInsideEnabledTrigger(event.relatedTarget)) {
      return;
    }
    const currentTrigger = triggerElementRef.current;
    handleCloseRef.current?.({
      ...dataRef.current.floatingContext,
      tree,
      x: event.clientX,
      y: event.clientY,
      onClose() {
        clearPointerEvents();
        cleanupMouseMoveHandler();
        if (!isClickLikeOpenEvent() && currentTrigger === store.select('domReferenceElement')) {
          closeWithDelay(event);
        }
      }
    })(event);
  });
  React.useEffect(() => {
    if (!enabled) {
      return undefined;
    }
    const trigger = triggerElementRef.current ?? (isActiveTrigger ? store.select('domReferenceElement') : null);
    if (!(0, _dom.isElement)(trigger)) {
      return undefined;
    }
    function onMouseEnter(event) {
      instance.openChangeTimeout.clear();
      instance.blockMouseMove = false;
      if (mouseOnly && !(0, _utils.isMouseLikePointerType)(instance.pointerType)) {
        return;
      }

      // Only rest delay is set; there's no fallback delay.
      // This will be handled by `onMouseMove`.
      if (getRestMs(restMsRef.current) > 0 && !(0, _useHover.getDelay)(delayRef.current, 'open')) {
        return;
      }
      const openDelay = (0, _useHover.getDelay)(delayRef.current, 'open', instance.pointerType);
      const currentDomReference = store.select('domReferenceElement');
      const allTriggers = store.context.triggerElements;
      const isOverInactiveTrigger = (allTriggers.hasElement(event.target) || allTriggers.hasMatchingElement(t => (0, _utils.contains)(t, event.target))) && (!currentDomReference || !(0, _utils.contains)(currentDomReference, event.target));
      const triggerNode = event.currentTarget ?? null;
      const isOpen = store.select('open');
      const shouldOpen = !isOpen || isOverInactiveTrigger;

      // When moving between triggers while already open, open immediately without delay
      if (isOverInactiveTrigger && isOpen) {
        store.setOpen(true, (0, _createBaseUIEventDetails.createChangeEventDetails)(_reasons.REASONS.triggerHover, event, triggerNode));
      } else if (openDelay) {
        instance.openChangeTimeout.start(openDelay, () => {
          if (shouldOpen) {
            store.setOpen(true, (0, _createBaseUIEventDetails.createChangeEventDetails)(_reasons.REASONS.triggerHover, event, triggerNode));
          }
        });
      } else if (shouldOpen) {
        store.setOpen(true, (0, _createBaseUIEventDetails.createChangeEventDetails)(_reasons.REASONS.triggerHover, event, triggerNode));
      }
    }
    function onMouseLeave(event) {
      if (isClickLikeOpenEvent()) {
        clearPointerEvents();
        return;
      }
      instance.unbindMouseMove();
      const domReferenceElement = store.select('domReferenceElement');
      const doc = (0, _owner.ownerDocument)(domReferenceElement);
      instance.restTimeout.clear();
      instance.restTimeoutPending = false;
      if (isRelatedTargetInsideEnabledTrigger(event.relatedTarget)) {
        return;
      }
      if (handleCloseRef.current && dataRef.current.floatingContext) {
        if (!store.select('open')) {
          instance.openChangeTimeout.clear();
        }
        const currentTrigger = triggerElementRef.current;
        instance.handler = handleCloseRef.current({
          ...dataRef.current.floatingContext,
          tree,
          x: event.clientX,
          y: event.clientY,
          onClose() {
            clearPointerEvents();
            cleanupMouseMoveHandler();
            if (enabledRef.current && !isClickLikeOpenEvent() && currentTrigger === store.select('domReferenceElement')) {
              closeWithDelay(event, true);
            }
          }
        });
        const handler = instance.handler;
        handler(event);
        doc.addEventListener('mousemove', handler);
        instance.unbindMouseMove = () => {
          doc.removeEventListener('mousemove', handler);
        };
        return;
      }
      const shouldClose = instance.pointerType === 'touch' ? !(0, _utils.contains)(store.select('floatingElement'), event.relatedTarget) : true;
      if (shouldClose) {
        closeWithDelay(event);
      }
    }
    function onScrollMouseLeave(event) {
      handleScrollMouseLeave(event);
    }
    if (store.select('open')) {
      trigger.addEventListener('mouseleave', onScrollMouseLeave);
    }
    if (move) {
      trigger.addEventListener('mousemove', onMouseEnter, {
        once: true
      });
    }
    trigger.addEventListener('mouseenter', onMouseEnter);
    trigger.addEventListener('mouseleave', onMouseLeave);
    return () => {
      trigger.removeEventListener('mouseleave', onScrollMouseLeave);
      if (move) {
        trigger.removeEventListener('mousemove', onMouseEnter);
      }
      trigger.removeEventListener('mouseenter', onMouseEnter);
      trigger.removeEventListener('mouseleave', onMouseLeave);
    };
  }, [cleanupMouseMoveHandler, clearPointerEvents, dataRef, delayRef, closeWithDelay, store, enabled, handleCloseRef, handleScrollMouseLeave, instance, isActiveTrigger, isClickLikeOpenEvent, isRelatedTargetInsideEnabledTrigger, mouseOnly, move, restMsRef, triggerElementRef, tree, enabledRef]);
  return React.useMemo(() => {
    if (!enabled) {
      return undefined;
    }
    function setPointerRef(event) {
      instance.pointerType = event.pointerType;
    }
    return {
      onPointerDown: setPointerRef,
      onPointerEnter: setPointerRef,
      onMouseMove(event) {
        const {
          nativeEvent
        } = event;
        const trigger = event.currentTarget;
        const currentDomReference = store.select('domReferenceElement');
        const allTriggers = store.context.triggerElements;
        const currentOpen = store.select('open');
        const isOverInactiveTrigger = (allTriggers.hasElement(event.target) || allTriggers.hasMatchingElement(t => (0, _utils.contains)(t, event.target))) && (!currentDomReference || !(0, _utils.contains)(currentDomReference, event.target));
        if (mouseOnly && !(0, _utils.isMouseLikePointerType)(instance.pointerType)) {
          return;
        }
        if (currentOpen && !isOverInactiveTrigger || getRestMs(restMsRef.current) === 0) {
          return;
        }
        if (!isOverInactiveTrigger && instance.restTimeoutPending && event.movementX ** 2 + event.movementY ** 2 < 2) {
          return;
        }
        instance.restTimeout.clear();
        function handleMouseMove() {
          instance.restTimeoutPending = false;

          // A delayed hover open should not override a click-like open that happened
          // while the hover delay was pending.
          if (isClickLikeOpenEvent()) {
            return;
          }
          const latestOpen = store.select('open');
          if (!instance.blockMouseMove && (!latestOpen || isOverInactiveTrigger)) {
            store.setOpen(true, (0, _createBaseUIEventDetails.createChangeEventDetails)(_reasons.REASONS.triggerHover, nativeEvent, trigger));
          }
        }
        if (instance.pointerType === 'touch') {
          ReactDOM.flushSync(() => {
            handleMouseMove();
          });
        } else if (isOverInactiveTrigger && currentOpen) {
          handleMouseMove();
        } else {
          instance.restTimeoutPending = true;
          instance.restTimeout.start(getRestMs(restMsRef.current), handleMouseMove);
        }
      }
    };
  }, [enabled, instance, isClickLikeOpenEvent, mouseOnly, store, restMsRef]);
}