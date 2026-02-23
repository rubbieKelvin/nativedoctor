'use client';

import { useOnMount } from '@base-ui/utils/useOnMount';
import { useRefWithInit } from '@base-ui/utils/useRefWithInit';
import { Timeout } from '@base-ui/utils/useTimeout';
import { createAttribute } from "../utils/createAttribute.js";
import { TYPEABLE_SELECTOR } from "../utils/constants.js";
export const safePolygonIdentifier = createAttribute('safe-polygon');
const interactiveSelector = `button,a,[role="button"],select,[tabindex]:not([tabindex="-1"]),${TYPEABLE_SELECTOR}`;
export function isInteractiveElement(element) {
  return element ? Boolean(element.closest(interactiveSelector)) : false;
}
export class HoverInteraction {
  constructor() {
    this.pointerType = undefined;
    this.interactedInside = false;
    this.handler = undefined;
    this.blockMouseMove = true;
    this.performedPointerEventsMutation = false;
    this.unbindMouseMove = () => {};
    this.restTimeoutPending = false;
    this.openChangeTimeout = new Timeout();
    this.restTimeout = new Timeout();
    this.handleCloseOptions = undefined;
  }
  static create() {
    return new HoverInteraction();
  }
  dispose = () => {
    this.openChangeTimeout.clear();
    this.restTimeout.clear();
  };
  disposeEffect = () => {
    return this.dispose;
  };
}
export function useHoverInteractionSharedState(store) {
  const instance = useRefWithInit(HoverInteraction.create).current;
  const data = store.context.dataRef.current;
  if (!data.hoverInteractionState) {
    data.hoverInteractionState = instance;
  }
  useOnMount(data.hoverInteractionState.disposeEffect);
  return data.hoverInteractionState;
}