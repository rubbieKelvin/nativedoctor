"use strict";
'use client';

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard").default;
Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.AvatarFallback = void 0;
var React = _interopRequireWildcard(require("react"));
var _useTimeout = require("@base-ui/utils/useTimeout");
var _useRenderElement = require("../../utils/useRenderElement");
var _AvatarRootContext = require("../root/AvatarRootContext");
var _stateAttributesMapping = require("../root/stateAttributesMapping");
var _useOpenChangeComplete = require("../../utils/useOpenChangeComplete");
var _stateAttributesMapping2 = require("../../utils/stateAttributesMapping");
var _useTransitionStatus = require("../../utils/useTransitionStatus");
const stateAttributesMapping = {
  ..._stateAttributesMapping.avatarStateAttributesMapping,
  ..._stateAttributesMapping2.transitionStatusMapping
};

/**
 * Rendered when the image fails to load or when no image is provided.
 * Renders a `<span>` element.
 *
 * Documentation: [Base UI Avatar](https://base-ui.com/react/components/avatar)
 */
const AvatarFallback = exports.AvatarFallback = /*#__PURE__*/React.forwardRef(function AvatarFallback(componentProps, forwardedRef) {
  const {
    className,
    render,
    delay,
    ...elementProps
  } = componentProps;
  const {
    imageLoadingStatus
  } = (0, _AvatarRootContext.useAvatarRootContext)();
  const [delayPassed, setDelayPassed] = React.useState(delay === undefined);
  const timeout = (0, _useTimeout.useTimeout)();
  const visible = imageLoadingStatus !== 'loaded' && delayPassed;
  const {
    mounted,
    transitionStatus,
    setMounted
  } = (0, _useTransitionStatus.useTransitionStatus)(visible);
  const fallbackRef = React.useRef(null);
  React.useEffect(() => {
    if (delay !== undefined) {
      timeout.start(delay, () => setDelayPassed(true));
    }
    return timeout.clear;
  }, [timeout, delay]);
  const state = {
    imageLoadingStatus,
    transitionStatus
  };
  (0, _useOpenChangeComplete.useOpenChangeComplete)({
    open: visible,
    ref: fallbackRef,
    onComplete() {
      if (!visible) {
        setMounted(false);
      }
    }
  });
  const element = (0, _useRenderElement.useRenderElement)('span', componentProps, {
    state,
    ref: [forwardedRef, fallbackRef],
    props: elementProps,
    stateAttributesMapping,
    enabled: mounted
  });
  if (!mounted) {
    return null;
  }
  return element;
});
if (process.env.NODE_ENV !== "production") AvatarFallback.displayName = "AvatarFallback";