"use strict";
'use client';

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.HoverInteraction = void 0;
exports.isInteractiveElement = isInteractiveElement;
exports.safePolygonIdentifier = void 0;
exports.useHoverInteractionSharedState = useHoverInteractionSharedState;
var _useOnMount = require("@base-ui/utils/useOnMount");
var _useRefWithInit = require("@base-ui/utils/useRefWithInit");
var _useTimeout = require("@base-ui/utils/useTimeout");
var _createAttribute = require("../utils/createAttribute");
var _constants = require("../utils/constants");
const safePolygonIdentifier = exports.safePolygonIdentifier = (0, _createAttribute.createAttribute)('safe-polygon');
const interactiveSelector = `button,a,[role="button"],select,[tabindex]:not([tabindex="-1"]),${_constants.TYPEABLE_SELECTOR}`;
function isInteractiveElement(element) {
  return element ? Boolean(element.closest(interactiveSelector)) : false;
}
class HoverInteraction {
  constructor() {
    this.pointerType = undefined;
    this.interactedInside = false;
    this.handler = undefined;
    this.blockMouseMove = true;
    this.performedPointerEventsMutation = false;
    this.unbindMouseMove = () => {};
    this.restTimeoutPending = false;
    this.openChangeTimeout = new _useTimeout.Timeout();
    this.restTimeout = new _useTimeout.Timeout();
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
exports.HoverInteraction = HoverInteraction;
function useHoverInteractionSharedState(store) {
  const instance = (0, _useRefWithInit.useRefWithInit)(HoverInteraction.create).current;
  const data = store.context.dataRef.current;
  if (!data.hoverInteractionState) {
    data.hoverInteractionState = instance;
  }
  (0, _useOnMount.useOnMount)(data.hoverInteractionState.disposeEffect);
  return data.hoverInteractionState;
}