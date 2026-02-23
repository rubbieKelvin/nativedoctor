"use strict";
'use client';

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard").default;
Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.normalizeProp = normalizeProp;
exports.useDismiss = useDismiss;
var React = _interopRequireWildcard(require("react"));
var _reactDom = require("@floating-ui/react-dom");
var _dom = require("@floating-ui/utils/dom");
var _useTimeout = require("@base-ui/utils/useTimeout");
var _useStableCallback = require("@base-ui/utils/useStableCallback");
var _owner = require("@base-ui/utils/owner");
var _utils = require("../utils");
var _FloatingTree = require("../components/FloatingTree");
var _createBaseUIEventDetails = require("../../utils/createBaseUIEventDetails");
var _reasons = require("../../utils/reasons");
var _createAttribute = require("../utils/createAttribute");
/* eslint-disable no-underscore-dangle */

const bubbleHandlerKeys = {
  intentional: 'onClick',
  sloppy: 'onPointerDown'
};
function normalizeProp(normalizable) {
  return {
    escapeKey: typeof normalizable === 'boolean' ? normalizable : normalizable?.escapeKey ?? false,
    outsidePress: typeof normalizable === 'boolean' ? normalizable : normalizable?.outsidePress ?? true
  };
}
/**
 * Closes the floating element when a dismissal is requested — by default, when
 * the user presses the `escape` key or outside of the floating element.
 * @see https://floating-ui.com/docs/useDismiss
 */
function useDismiss(context, props = {}) {
  const store = 'rootStore' in context ? context.rootStore : context;
  const open = store.useState('open');
  const floatingElement = store.useState('floatingElement');
  const referenceElement = store.useState('referenceElement');
  const domReferenceElement = store.useState('domReferenceElement');
  const {
    onOpenChange,
    dataRef
  } = store.context;
  const {
    enabled = true,
    escapeKey = true,
    outsidePress: outsidePressProp = true,
    outsidePressEvent = 'sloppy',
    referencePress = false,
    referencePressEvent = 'sloppy',
    ancestorScroll = false,
    bubbles,
    externalTree
  } = props;
  const tree = (0, _FloatingTree.useFloatingTree)(externalTree);
  const outsidePressFn = (0, _useStableCallback.useStableCallback)(typeof outsidePressProp === 'function' ? outsidePressProp : () => false);
  const outsidePress = typeof outsidePressProp === 'function' ? outsidePressFn : outsidePressProp;
  const endedOrStartedInsideRef = React.useRef(false);
  const {
    escapeKey: escapeKeyBubbles,
    outsidePress: outsidePressBubbles
  } = normalizeProp(bubbles);
  const touchStateRef = React.useRef(null);
  const cancelDismissOnEndTimeout = (0, _useTimeout.useTimeout)();
  const clearInsideReactTreeTimeout = (0, _useTimeout.useTimeout)();
  const clearInsideReactTree = (0, _useStableCallback.useStableCallback)(() => {
    clearInsideReactTreeTimeout.clear();
    dataRef.current.insideReactTree = false;
  });
  const isComposingRef = React.useRef(false);
  const currentPointerTypeRef = React.useRef('');
  const trackPointerType = (0, _useStableCallback.useStableCallback)(event => {
    currentPointerTypeRef.current = event.pointerType;
  });
  const getOutsidePressEvent = (0, _useStableCallback.useStableCallback)(() => {
    const type = currentPointerTypeRef.current;
    const computedType = type === 'pen' || !type ? 'mouse' : type;
    const resolved = typeof outsidePressEvent === 'function' ? outsidePressEvent() : outsidePressEvent;
    if (typeof resolved === 'string') {
      return resolved;
    }
    return resolved[computedType];
  });
  const closeOnEscapeKeyDown = (0, _useStableCallback.useStableCallback)(event => {
    if (!open || !enabled || !escapeKey || event.key !== 'Escape') {
      return;
    }

    // Wait until IME is settled. Pressing `Escape` while composing should
    // close the compose menu, but not the floating element.
    if (isComposingRef.current) {
      return;
    }
    const nodeId = dataRef.current.floatingContext?.nodeId;
    const children = tree ? (0, _utils.getNodeChildren)(tree.nodesRef.current, nodeId) : [];
    if (!escapeKeyBubbles) {
      if (children.length > 0) {
        let shouldDismiss = true;
        children.forEach(child => {
          if (child.context?.open && !child.context.dataRef.current.__escapeKeyBubbles) {
            shouldDismiss = false;
          }
        });
        if (!shouldDismiss) {
          return;
        }
      }
    }
    const native = (0, _utils.isReactEvent)(event) ? event.nativeEvent : event;
    const eventDetails = (0, _createBaseUIEventDetails.createChangeEventDetails)(_reasons.REASONS.escapeKey, native);
    store.setOpen(false, eventDetails);
    if (!escapeKeyBubbles && !eventDetails.isPropagationAllowed) {
      event.stopPropagation();
    }
  });
  const shouldIgnoreEvent = (0, _useStableCallback.useStableCallback)(event => {
    const computedOutsidePressEvent = getOutsidePressEvent();
    return computedOutsidePressEvent === 'intentional' && event.type !== 'click' || computedOutsidePressEvent === 'sloppy' && event.type === 'click';
  });
  const markInsideReactTree = (0, _useStableCallback.useStableCallback)(() => {
    dataRef.current.insideReactTree = true;
    clearInsideReactTreeTimeout.start(0, clearInsideReactTree);
  });
  const closeOnPressOutside = (0, _useStableCallback.useStableCallback)((event, endedOrStartedInside = false) => {
    if (shouldIgnoreEvent(event)) {
      clearInsideReactTree();
      return;
    }
    if (dataRef.current.insideReactTree) {
      clearInsideReactTree();
      return;
    }
    if (getOutsidePressEvent() === 'intentional' && endedOrStartedInside) {
      return;
    }
    if (typeof outsidePress === 'function' && !outsidePress(event)) {
      return;
    }
    const target = (0, _utils.getTarget)(event);
    const inertSelector = `[${(0, _createAttribute.createAttribute)('inert')}]`;
    const markers = (0, _owner.ownerDocument)(store.select('floatingElement')).querySelectorAll(inertSelector);
    const triggers = store.context.triggerElements;

    // If another trigger is clicked, don't close the floating element.
    if (target && (triggers.hasElement(target) || triggers.hasMatchingElement(trigger => (0, _utils.contains)(trigger, target)))) {
      return;
    }
    let targetRootAncestor = (0, _dom.isElement)(target) ? target : null;
    while (targetRootAncestor && !(0, _dom.isLastTraversableNode)(targetRootAncestor)) {
      const nextParent = (0, _dom.getParentNode)(targetRootAncestor);
      if ((0, _dom.isLastTraversableNode)(nextParent) || !(0, _dom.isElement)(nextParent)) {
        break;
      }
      targetRootAncestor = nextParent;
    }

    // Check if the click occurred on a third-party element injected after the
    // floating element rendered.
    if (markers.length && (0, _dom.isElement)(target) && !(0, _utils.isRootElement)(target) &&
    // Clicked on a direct ancestor (e.g. FloatingOverlay).
    !(0, _utils.contains)(target, store.select('floatingElement')) &&
    // If the target root element contains none of the markers, then the
    // element was injected after the floating element rendered.
    Array.from(markers).every(marker => !(0, _utils.contains)(targetRootAncestor, marker))) {
      return;
    }

    // Check if the click occurred on the scrollbar
    // Skip for touch events: scrollbars don't receive touch events on most platforms
    if ((0, _dom.isHTMLElement)(target) && !('touches' in event)) {
      const lastTraversableNode = (0, _dom.isLastTraversableNode)(target);
      const style = (0, _dom.getComputedStyle)(target);
      const scrollRe = /auto|scroll/;
      const isScrollableX = lastTraversableNode || scrollRe.test(style.overflowX);
      const isScrollableY = lastTraversableNode || scrollRe.test(style.overflowY);
      const canScrollX = isScrollableX && target.clientWidth > 0 && target.scrollWidth > target.clientWidth;
      const canScrollY = isScrollableY && target.clientHeight > 0 && target.scrollHeight > target.clientHeight;
      const isRTL = style.direction === 'rtl';

      // Check click position relative to scrollbar.
      // In some browsers it is possible to change the <body> (or window)
      // scrollbar to the left side, but is very rare and is difficult to
      // check for. Plus, for modal dialogs with backdrops, it is more
      // important that the backdrop is checked but not so much the window.
      const pressedVerticalScrollbar = canScrollY && (isRTL ? event.offsetX <= target.offsetWidth - target.clientWidth : event.offsetX > target.clientWidth);
      const pressedHorizontalScrollbar = canScrollX && event.offsetY > target.clientHeight;
      if (pressedVerticalScrollbar || pressedHorizontalScrollbar) {
        return;
      }
    }
    const nodeId = dataRef.current.floatingContext?.nodeId;
    const targetIsInsideChildren = tree && (0, _utils.getNodeChildren)(tree.nodesRef.current, nodeId).some(node => (0, _utils.isEventTargetWithin)(event, node.context?.elements.floating));
    if ((0, _utils.isEventTargetWithin)(event, store.select('floatingElement')) || (0, _utils.isEventTargetWithin)(event, store.select('domReferenceElement')) || targetIsInsideChildren) {
      return;
    }
    const children = tree ? (0, _utils.getNodeChildren)(tree.nodesRef.current, nodeId) : [];
    if (children.length > 0) {
      let shouldDismiss = true;
      children.forEach(child => {
        if (child.context?.open && !child.context.dataRef.current.__outsidePressBubbles) {
          shouldDismiss = false;
        }
      });
      if (!shouldDismiss) {
        return;
      }
    }
    store.setOpen(false, (0, _createBaseUIEventDetails.createChangeEventDetails)(_reasons.REASONS.outsidePress, event));
    clearInsideReactTree();
  });
  const handlePointerDown = (0, _useStableCallback.useStableCallback)(event => {
    if (getOutsidePressEvent() !== 'sloppy' || event.pointerType === 'touch' || !store.select('open') || !enabled || (0, _utils.isEventTargetWithin)(event, store.select('floatingElement')) || (0, _utils.isEventTargetWithin)(event, store.select('domReferenceElement'))) {
      return;
    }
    closeOnPressOutside(event);
  });
  const handleTouchStart = (0, _useStableCallback.useStableCallback)(event => {
    if (getOutsidePressEvent() !== 'sloppy' || !store.select('open') || !enabled || (0, _utils.isEventTargetWithin)(event, store.select('floatingElement')) || (0, _utils.isEventTargetWithin)(event, store.select('domReferenceElement'))) {
      return;
    }
    const touch = event.touches[0];
    if (touch) {
      touchStateRef.current = {
        startTime: Date.now(),
        startX: touch.clientX,
        startY: touch.clientY,
        dismissOnTouchEnd: false,
        dismissOnMouseDown: true
      };
      cancelDismissOnEndTimeout.start(1000, () => {
        if (touchStateRef.current) {
          touchStateRef.current.dismissOnTouchEnd = false;
          touchStateRef.current.dismissOnMouseDown = false;
        }
      });
    }
  });
  const handleTouchStartCapture = (0, _useStableCallback.useStableCallback)(event => {
    const target = (0, _utils.getTarget)(event);
    function callback() {
      handleTouchStart(event);
      target?.removeEventListener(event.type, callback);
    }
    target?.addEventListener(event.type, callback);
  });
  const closeOnPressOutsideCapture = (0, _useStableCallback.useStableCallback)(event => {
    // When click outside is lazy (`up` event), handle dragging.
    // Don't close if:
    // - The click started inside the floating element.
    // - The click ended inside the floating element.
    const endedOrStartedInside = endedOrStartedInsideRef.current;
    endedOrStartedInsideRef.current = false;
    cancelDismissOnEndTimeout.clear();
    if (event.type === 'mousedown' && touchStateRef.current && !touchStateRef.current.dismissOnMouseDown) {
      return;
    }
    const target = (0, _utils.getTarget)(event);
    function callback() {
      if (event.type === 'pointerdown') {
        handlePointerDown(event);
      } else {
        closeOnPressOutside(event, endedOrStartedInside);
      }
      target?.removeEventListener(event.type, callback);
    }
    target?.addEventListener(event.type, callback);
  });
  const handleTouchMove = (0, _useStableCallback.useStableCallback)(event => {
    if (getOutsidePressEvent() !== 'sloppy' || !touchStateRef.current || (0, _utils.isEventTargetWithin)(event, store.select('floatingElement')) || (0, _utils.isEventTargetWithin)(event, store.select('domReferenceElement'))) {
      return;
    }
    const touch = event.touches[0];
    if (!touch) {
      return;
    }
    const deltaX = Math.abs(touch.clientX - touchStateRef.current.startX);
    const deltaY = Math.abs(touch.clientY - touchStateRef.current.startY);
    const distance = Math.sqrt(deltaX * deltaX + deltaY * deltaY);
    if (distance > 5) {
      touchStateRef.current.dismissOnTouchEnd = true;
    }
    if (distance > 10) {
      closeOnPressOutside(event);
      cancelDismissOnEndTimeout.clear();
      touchStateRef.current = null;
    }
  });
  const handleTouchMoveCapture = (0, _useStableCallback.useStableCallback)(event => {
    const target = (0, _utils.getTarget)(event);
    function callback() {
      handleTouchMove(event);
      target?.removeEventListener(event.type, callback);
    }
    target?.addEventListener(event.type, callback);
  });
  const handleTouchEnd = (0, _useStableCallback.useStableCallback)(event => {
    if (getOutsidePressEvent() !== 'sloppy' || !touchStateRef.current || (0, _utils.isEventTargetWithin)(event, store.select('floatingElement')) || (0, _utils.isEventTargetWithin)(event, store.select('domReferenceElement'))) {
      return;
    }
    if (touchStateRef.current.dismissOnTouchEnd) {
      closeOnPressOutside(event);
    }
    cancelDismissOnEndTimeout.clear();
    touchStateRef.current = null;
  });
  const handleTouchEndCapture = (0, _useStableCallback.useStableCallback)(event => {
    const target = (0, _utils.getTarget)(event);
    function callback() {
      handleTouchEnd(event);
      target?.removeEventListener(event.type, callback);
    }
    target?.addEventListener(event.type, callback);
  });
  React.useEffect(() => {
    if (!open || !enabled) {
      return undefined;
    }
    dataRef.current.__escapeKeyBubbles = escapeKeyBubbles;
    dataRef.current.__outsidePressBubbles = outsidePressBubbles;
    const compositionTimeout = new _useTimeout.Timeout();
    function onScroll(event) {
      store.setOpen(false, (0, _createBaseUIEventDetails.createChangeEventDetails)(_reasons.REASONS.none, event));
    }
    function handleCompositionStart() {
      compositionTimeout.clear();
      isComposingRef.current = true;
    }
    function handleCompositionEnd() {
      // Safari fires `compositionend` before `keydown`, so we need to wait
      // until the next tick to set `isComposing` to `false`.
      // https://bugs.webkit.org/show_bug.cgi?id=165004
      compositionTimeout.start(
      // 0ms or 1ms don't work in Safari. 5ms appears to consistently work.
      // Only apply to WebKit for the test to remain 0ms.
      (0, _dom.isWebKit)() ? 5 : 0, () => {
        isComposingRef.current = false;
      });
    }
    const doc = (0, _owner.ownerDocument)(floatingElement);
    doc.addEventListener('pointerdown', trackPointerType, true);
    if (escapeKey) {
      doc.addEventListener('keydown', closeOnEscapeKeyDown);
      doc.addEventListener('compositionstart', handleCompositionStart);
      doc.addEventListener('compositionend', handleCompositionEnd);
    }
    if (outsidePress) {
      doc.addEventListener('click', closeOnPressOutsideCapture, true);
      doc.addEventListener('pointerdown', closeOnPressOutsideCapture, true);
      doc.addEventListener('touchstart', handleTouchStartCapture, true);
      doc.addEventListener('touchmove', handleTouchMoveCapture, true);
      doc.addEventListener('touchend', handleTouchEndCapture, true);
      doc.addEventListener('mousedown', closeOnPressOutsideCapture, true);
    }
    let ancestors = [];
    if (ancestorScroll) {
      if ((0, _dom.isElement)(domReferenceElement)) {
        ancestors = (0, _reactDom.getOverflowAncestors)(domReferenceElement);
      }
      if ((0, _dom.isElement)(floatingElement)) {
        ancestors = ancestors.concat((0, _reactDom.getOverflowAncestors)(floatingElement));
      }
      if (!(0, _dom.isElement)(referenceElement) && referenceElement && referenceElement.contextElement) {
        ancestors = ancestors.concat((0, _reactDom.getOverflowAncestors)(referenceElement.contextElement));
      }
    }

    // Ignore the visual viewport for scrolling dismissal (allow pinch-zoom)
    ancestors = ancestors.filter(ancestor => ancestor !== doc.defaultView?.visualViewport);
    ancestors.forEach(ancestor => {
      ancestor.addEventListener('scroll', onScroll, {
        passive: true
      });
    });
    return () => {
      doc.removeEventListener('pointerdown', trackPointerType, true);
      if (escapeKey) {
        doc.removeEventListener('keydown', closeOnEscapeKeyDown);
        doc.removeEventListener('compositionstart', handleCompositionStart);
        doc.removeEventListener('compositionend', handleCompositionEnd);
      }
      if (outsidePress) {
        doc.removeEventListener('click', closeOnPressOutsideCapture, true);
        doc.removeEventListener('pointerdown', closeOnPressOutsideCapture, true);
        doc.removeEventListener('touchstart', handleTouchStartCapture, true);
        doc.removeEventListener('touchmove', handleTouchMoveCapture, true);
        doc.removeEventListener('touchend', handleTouchEndCapture, true);
        doc.removeEventListener('mousedown', closeOnPressOutsideCapture, true);
      }
      ancestors.forEach(ancestor => {
        ancestor.removeEventListener('scroll', onScroll);
      });
      compositionTimeout.clear();
      endedOrStartedInsideRef.current = false;
    };
  }, [dataRef, floatingElement, referenceElement, domReferenceElement, escapeKey, outsidePress, open, onOpenChange, ancestorScroll, enabled, escapeKeyBubbles, outsidePressBubbles, closeOnEscapeKeyDown, closeOnPressOutside, closeOnPressOutsideCapture, handlePointerDown, handleTouchStartCapture, handleTouchMoveCapture, handleTouchEndCapture, trackPointerType, store]);
  React.useEffect(clearInsideReactTree, [outsidePress, clearInsideReactTree]);
  const reference = React.useMemo(() => ({
    onKeyDown: closeOnEscapeKeyDown,
    ...(referencePress && {
      [bubbleHandlerKeys[referencePressEvent]]: event => {
        store.setOpen(false, (0, _createBaseUIEventDetails.createChangeEventDetails)(_reasons.REASONS.triggerPress, event.nativeEvent));
      },
      ...(referencePressEvent !== 'intentional' && {
        onClick(event) {
          store.setOpen(false, (0, _createBaseUIEventDetails.createChangeEventDetails)(_reasons.REASONS.triggerPress, event.nativeEvent));
        }
      })
    })
  }), [closeOnEscapeKeyDown, store, referencePress, referencePressEvent]);
  const handlePressedInside = (0, _useStableCallback.useStableCallback)(event => {
    const target = (0, _utils.getTarget)(event.nativeEvent);
    if (!(0, _utils.contains)(store.select('floatingElement'), target) || event.button !== 0) {
      return;
    }
    endedOrStartedInsideRef.current = true;
  });
  const markPressStartedInsideReactTree = (0, _useStableCallback.useStableCallback)(event => {
    if (!open || !enabled || event.button !== 0) {
      return;
    }
    endedOrStartedInsideRef.current = true;
  });
  const floating = React.useMemo(() => ({
    onKeyDown: closeOnEscapeKeyDown,
    // `onMouseDown` may be blocked if `event.preventDefault()` is called in
    // `onPointerDown`, such as with <NumberField.ScrubArea>.
    // See https://github.com/mui/base-ui/pull/3379
    onPointerDown: handlePressedInside,
    onMouseDown: handlePressedInside,
    onMouseUp: handlePressedInside,
    onClickCapture: markInsideReactTree,
    onMouseDownCapture(event) {
      markInsideReactTree();
      markPressStartedInsideReactTree(event);
    },
    onPointerDownCapture(event) {
      markInsideReactTree();
      markPressStartedInsideReactTree(event);
    },
    onMouseUpCapture: markInsideReactTree,
    onTouchEndCapture: markInsideReactTree,
    onTouchMoveCapture: markInsideReactTree
  }), [closeOnEscapeKeyDown, handlePressedInside, markInsideReactTree, markPressStartedInsideReactTree]);
  return React.useMemo(() => enabled ? {
    reference,
    floating,
    trigger: reference
  } : {}, [enabled, reference, floating]);
}