'use client';

import * as React from 'react';
import { useTimeout } from '@base-ui/utils/useTimeout';
import { useRenderElement } from "../../utils/useRenderElement.js";
import { useAvatarRootContext } from "../root/AvatarRootContext.js";
import { avatarStateAttributesMapping } from "../root/stateAttributesMapping.js";
import { useOpenChangeComplete } from "../../utils/useOpenChangeComplete.js";
import { transitionStatusMapping } from "../../utils/stateAttributesMapping.js";
import { useTransitionStatus } from "../../utils/useTransitionStatus.js";
const stateAttributesMapping = {
  ...avatarStateAttributesMapping,
  ...transitionStatusMapping
};

/**
 * Rendered when the image fails to load or when no image is provided.
 * Renders a `<span>` element.
 *
 * Documentation: [Base UI Avatar](https://base-ui.com/react/components/avatar)
 */
export const AvatarFallback = /*#__PURE__*/React.forwardRef(function AvatarFallback(componentProps, forwardedRef) {
  const {
    className,
    render,
    delay,
    ...elementProps
  } = componentProps;
  const {
    imageLoadingStatus
  } = useAvatarRootContext();
  const [delayPassed, setDelayPassed] = React.useState(delay === undefined);
  const timeout = useTimeout();
  const visible = imageLoadingStatus !== 'loaded' && delayPassed;
  const {
    mounted,
    transitionStatus,
    setMounted
  } = useTransitionStatus(visible);
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
  useOpenChangeComplete({
    open: visible,
    ref: fallbackRef,
    onComplete() {
      if (!visible) {
        setMounted(false);
      }
    }
  });
  const element = useRenderElement('span', componentProps, {
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