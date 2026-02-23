import { isElement } from '@floating-ui/utils/dom';
import { Timeout } from '@base-ui/utils/useTimeout';
import { contains, getTarget } from "./utils/element.js";
import { getNodeChildren } from "./utils/nodes.js";

/* eslint-disable no-nested-ternary */

function isPointInPolygon(point, polygon) {
  const [x, y] = point;
  let isInsideValue = false;
  const length = polygon.length;
  // eslint-disable-next-line no-plusplus
  for (let i = 0, j = length - 1; i < length; j = i++) {
    const [xi, yi] = polygon[i] || [0, 0];
    const [xj, yj] = polygon[j] || [0, 0];
    const intersect = yi >= y !== yj >= y && x <= (xj - xi) * (y - yi) / (yj - yi) + xi;
    if (intersect) {
      isInsideValue = !isInsideValue;
    }
  }
  return isInsideValue;
}
function isInside(point, rect) {
  return point[0] >= rect.x && point[0] <= rect.x + rect.width && point[1] >= rect.y && point[1] <= rect.y + rect.height;
}
/**
 * Generates a safe polygon area that the user can traverse without closing the
 * floating element once leaving the reference element.
 * @see https://floating-ui.com/docs/useHover#safepolygon
 */
export function safePolygon(options = {}) {
  const {
    buffer = 0.5,
    blockPointerEvents = false,
    requireIntent = true
  } = options;
  const timeout = new Timeout();
  let hasLanded = false;
  let lastX = null;
  let lastY = null;
  let lastCursorTime = typeof performance !== 'undefined' ? performance.now() : 0;
  function getCursorSpeed(x, y) {
    const currentTime = performance.now();
    const elapsedTime = currentTime - lastCursorTime;
    if (lastX === null || lastY === null || elapsedTime === 0) {
      lastX = x;
      lastY = y;
      lastCursorTime = currentTime;
      return null;
    }
    const deltaX = x - lastX;
    const deltaY = y - lastY;
    const distance = Math.sqrt(deltaX * deltaX + deltaY * deltaY);
    const speed = distance / elapsedTime; // px / ms

    lastX = x;
    lastY = y;
    lastCursorTime = currentTime;
    return speed;
  }
  const fn = ({
    x,
    y,
    placement,
    elements,
    onClose,
    nodeId,
    tree
  }) => {
    return function onMouseMove(event) {
      function close() {
        timeout.clear();
        onClose();
      }
      timeout.clear();
      if (!elements.domReference || !elements.floating || placement == null || x == null || y == null) {
        return undefined;
      }
      const {
        clientX,
        clientY
      } = event;
      const clientPoint = [clientX, clientY];
      const target = getTarget(event);
      const isLeave = event.type === 'mouseleave';
      const isOverFloatingEl = contains(elements.floating, target);
      const isOverReferenceEl = contains(elements.domReference, target);
      const refRect = elements.domReference.getBoundingClientRect();
      const rect = elements.floating.getBoundingClientRect();
      const side = placement.split('-')[0];
      const cursorLeaveFromRight = x > rect.right - rect.width / 2;
      const cursorLeaveFromBottom = y > rect.bottom - rect.height / 2;
      const isOverReferenceRect = isInside(clientPoint, refRect);
      const isFloatingWider = rect.width > refRect.width;
      const isFloatingTaller = rect.height > refRect.height;
      const left = (isFloatingWider ? refRect : rect).left;
      const right = (isFloatingWider ? refRect : rect).right;
      const top = (isFloatingTaller ? refRect : rect).top;
      const bottom = (isFloatingTaller ? refRect : rect).bottom;
      if (isOverFloatingEl) {
        hasLanded = true;
        if (!isLeave) {
          return undefined;
        }
      }
      if (isOverReferenceEl) {
        hasLanded = false;
      }
      if (isOverReferenceEl && !isLeave) {
        hasLanded = true;
        return undefined;
      }

      // Prevent overlapping floating element from being stuck in an open-close
      // loop: https://github.com/floating-ui/floating-ui/issues/1910
      if (isLeave && isElement(event.relatedTarget) && contains(elements.floating, event.relatedTarget)) {
        return undefined;
      }

      // If any nested child is open, abort.
      if (tree && getNodeChildren(tree.nodesRef.current, nodeId).some(({
        context
      }) => context?.open)) {
        return undefined;
      }

      // If the pointer is leaving from the opposite side, the "buffer" logic
      // creates a point where the floating element remains open, but should be
      // ignored.
      // A constant of 1 handles floating point rounding errors.
      if (side === 'top' && y >= refRect.bottom - 1 || side === 'bottom' && y <= refRect.top + 1 || side === 'left' && x >= refRect.right - 1 || side === 'right' && x <= refRect.left + 1) {
        return close();
      }

      // Ignore when the cursor is within the rectangular trough between the
      // two elements. Since the triangle is created from the cursor point,
      // which can start beyond the ref element's edge, traversing back and
      // forth from the ref to the floating element can cause it to close. This
      // ensures it always remains open in that case.
      let rectPoly = [];
      switch (side) {
        case 'top':
          rectPoly = [[left, refRect.top + 1], [left, rect.bottom - 1], [right, rect.bottom - 1], [right, refRect.top + 1]];
          break;
        case 'bottom':
          rectPoly = [[left, rect.top + 1], [left, refRect.bottom - 1], [right, refRect.bottom - 1], [right, rect.top + 1]];
          break;
        case 'left':
          rectPoly = [[rect.right - 1, bottom], [rect.right - 1, top], [refRect.left + 1, top], [refRect.left + 1, bottom]];
          break;
        case 'right':
          rectPoly = [[refRect.right - 1, bottom], [refRect.right - 1, top], [rect.left + 1, top], [rect.left + 1, bottom]];
          break;
        default:
      }
      function getPolygon([px, py]) {
        switch (side) {
          case 'top':
            {
              const cursorPointOne = [isFloatingWider ? px + buffer / 2 : cursorLeaveFromRight ? px + buffer * 4 : px - buffer * 4, py + buffer + 1];
              const cursorPointTwo = [isFloatingWider ? px - buffer / 2 : cursorLeaveFromRight ? px + buffer * 4 : px - buffer * 4, py + buffer + 1];
              const commonPoints = [[rect.left, cursorLeaveFromRight ? rect.bottom - buffer : isFloatingWider ? rect.bottom - buffer : rect.top], [rect.right, cursorLeaveFromRight ? isFloatingWider ? rect.bottom - buffer : rect.top : rect.bottom - buffer]];
              return [cursorPointOne, cursorPointTwo, ...commonPoints];
            }
          case 'bottom':
            {
              const cursorPointOne = [isFloatingWider ? px + buffer / 2 : cursorLeaveFromRight ? px + buffer * 4 : px - buffer * 4, py - buffer];
              const cursorPointTwo = [isFloatingWider ? px - buffer / 2 : cursorLeaveFromRight ? px + buffer * 4 : px - buffer * 4, py - buffer];
              const commonPoints = [[rect.left, cursorLeaveFromRight ? rect.top + buffer : isFloatingWider ? rect.top + buffer : rect.bottom], [rect.right, cursorLeaveFromRight ? isFloatingWider ? rect.top + buffer : rect.bottom : rect.top + buffer]];
              return [cursorPointOne, cursorPointTwo, ...commonPoints];
            }
          case 'left':
            {
              const cursorPointOne = [px + buffer + 1, isFloatingTaller ? py + buffer / 2 : cursorLeaveFromBottom ? py + buffer * 4 : py - buffer * 4];
              const cursorPointTwo = [px + buffer + 1, isFloatingTaller ? py - buffer / 2 : cursorLeaveFromBottom ? py + buffer * 4 : py - buffer * 4];
              const commonPoints = [[cursorLeaveFromBottom ? rect.right - buffer : isFloatingTaller ? rect.right - buffer : rect.left, rect.top], [cursorLeaveFromBottom ? isFloatingTaller ? rect.right - buffer : rect.left : rect.right - buffer, rect.bottom]];
              return [...commonPoints, cursorPointOne, cursorPointTwo];
            }
          case 'right':
            {
              const cursorPointOne = [px - buffer, isFloatingTaller ? py + buffer / 2 : cursorLeaveFromBottom ? py + buffer * 4 : py - buffer * 4];
              const cursorPointTwo = [px - buffer, isFloatingTaller ? py - buffer / 2 : cursorLeaveFromBottom ? py + buffer * 4 : py - buffer * 4];
              const commonPoints = [[cursorLeaveFromBottom ? rect.left + buffer : isFloatingTaller ? rect.left + buffer : rect.right, rect.top], [cursorLeaveFromBottom ? isFloatingTaller ? rect.left + buffer : rect.right : rect.left + buffer, rect.bottom]];
              return [cursorPointOne, cursorPointTwo, ...commonPoints];
            }
          default:
            return [];
        }
      }
      if (isPointInPolygon([clientX, clientY], rectPoly)) {
        return undefined;
      }
      if (hasLanded && !isOverReferenceRect) {
        return close();
      }
      if (!isLeave && requireIntent) {
        const cursorSpeed = getCursorSpeed(event.clientX, event.clientY);
        const cursorSpeedThreshold = 0.1;
        if (cursorSpeed !== null && cursorSpeed < cursorSpeedThreshold) {
          return close();
        }
      }
      if (!isPointInPolygon([clientX, clientY], getPolygon([x, y]))) {
        close();
      } else if (!hasLanded && requireIntent) {
        timeout.start(40, close);
      }
      return undefined;
    };
  };

  // eslint-disable-next-line no-underscore-dangle
  fn.__options = {
    blockPointerEvents
  };
  return fn;
}