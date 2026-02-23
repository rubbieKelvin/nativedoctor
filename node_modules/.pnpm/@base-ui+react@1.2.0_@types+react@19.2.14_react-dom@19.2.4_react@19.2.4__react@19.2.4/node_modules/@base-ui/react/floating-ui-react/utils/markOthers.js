"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.markOthers = markOthers;
exports.supportsInert = void 0;
var _dom = require("@floating-ui/utils/dom");
var _owner = require("@base-ui/utils/owner");
// Modified to add conditional `aria-hidden` support:
// https://github.com/theKashey/aria-hidden/blob/9220c8f4a4fd35f63bee5510a9f41a37264382d4/src/index.ts

const counters = {
  inert: new WeakMap(),
  'aria-hidden': new WeakMap(),
  none: new WeakMap()
};
function getCounterMap(control) {
  if (control === 'inert') {
    return counters.inert;
  }
  if (control === 'aria-hidden') {
    return counters['aria-hidden'];
  }
  return counters.none;
}
let uncontrolledElementsSet = new WeakSet();
let markerMap = {};
let lockCount = 0;
const supportsInert = () => typeof HTMLElement !== 'undefined' && 'inert' in HTMLElement.prototype;
exports.supportsInert = supportsInert;
const unwrapHost = node => node && (node.host || unwrapHost(node.parentNode));
const correctElements = (parent, targets) => targets.map(target => {
  if (parent.contains(target)) {
    return target;
  }
  const correctedTarget = unwrapHost(target);
  if (parent.contains(correctedTarget)) {
    return correctedTarget;
  }
  return null;
}).filter(x => x != null);
function applyAttributeToOthers(uncorrectedAvoidElements, body, ariaHidden, inert) {
  const markerName = 'data-base-ui-inert';
  // eslint-disable-next-line no-nested-ternary
  const controlAttribute = inert ? 'inert' : ariaHidden ? 'aria-hidden' : null;
  const avoidElements = correctElements(body, uncorrectedAvoidElements);
  const elementsToKeep = new Set();
  const elementsToStop = new Set(avoidElements);
  const hiddenElements = [];
  if (!markerMap[markerName]) {
    markerMap[markerName] = new WeakMap();
  }
  const markerCounter = markerMap[markerName];
  avoidElements.forEach(keep);
  deep(body);
  elementsToKeep.clear();
  function keep(el) {
    if (!el || elementsToKeep.has(el)) {
      return;
    }
    elementsToKeep.add(el);
    if (el.parentNode) {
      keep(el.parentNode);
    }
  }
  function deep(parent) {
    if (!parent || elementsToStop.has(parent)) {
      return;
    }
    [].forEach.call(parent.children, node => {
      if ((0, _dom.getNodeName)(node) === 'script') {
        return;
      }
      if (elementsToKeep.has(node)) {
        deep(node);
      } else {
        const attr = controlAttribute ? node.getAttribute(controlAttribute) : null;
        const alreadyHidden = attr !== null && attr !== 'false';
        const counterMap = getCounterMap(controlAttribute);
        const counterValue = (counterMap.get(node) || 0) + 1;
        const markerValue = (markerCounter.get(node) || 0) + 1;
        counterMap.set(node, counterValue);
        markerCounter.set(node, markerValue);
        hiddenElements.push(node);
        if (counterValue === 1 && alreadyHidden) {
          uncontrolledElementsSet.add(node);
        }
        if (markerValue === 1) {
          node.setAttribute(markerName, '');
        }
        if (!alreadyHidden && controlAttribute) {
          node.setAttribute(controlAttribute, controlAttribute === 'inert' ? '' : 'true');
        }
      }
    });
  }
  lockCount += 1;
  return () => {
    hiddenElements.forEach(element => {
      const counterMap = getCounterMap(controlAttribute);
      const currentCounterValue = counterMap.get(element) || 0;
      const counterValue = currentCounterValue - 1;
      const markerValue = (markerCounter.get(element) || 0) - 1;
      counterMap.set(element, counterValue);
      markerCounter.set(element, markerValue);
      if (!counterValue) {
        if (!uncontrolledElementsSet.has(element) && controlAttribute) {
          element.removeAttribute(controlAttribute);
        }
        uncontrolledElementsSet.delete(element);
      }
      if (!markerValue) {
        element.removeAttribute(markerName);
      }
    });
    lockCount -= 1;
    if (!lockCount) {
      counters.inert = new WeakMap();
      counters['aria-hidden'] = new WeakMap();
      counters.none = new WeakMap();
      uncontrolledElementsSet = new WeakSet();
      markerMap = {};
    }
  };
}
function markOthers(avoidElements, ariaHidden = false, inert = false) {
  const body = (0, _owner.ownerDocument)(avoidElements[0]).body;
  return applyAttributeToOthers(avoidElements.concat(Array.from(body.querySelectorAll('[aria-live]'))), body, ariaHidden, inert);
}