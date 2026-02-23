"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.DrawerPopupDataAttributes = void 0;
let DrawerPopupDataAttributes = exports.DrawerPopupDataAttributes = /*#__PURE__*/function (DrawerPopupDataAttributes) {
  /**
   * Present when the drawer is at the expanded (full-height) snap point.
   */
  DrawerPopupDataAttributes["expanded"] = "data-expanded";
  /**
   * Present when a nested drawer is open.
   */
  DrawerPopupDataAttributes["nestedDrawerOpen"] = "data-nested-drawer-open";
  /**
   * Present when a nested drawer is being swiped.
   */
  DrawerPopupDataAttributes["nestedDrawerSwiping"] = "data-nested-drawer-swiping";
  /**
   * Present when the drawer is dismissed by swiping.
   */
  DrawerPopupDataAttributes["swipeDismiss"] = "data-swipe-dismiss";
  /**
   * Indicates the swipe direction.
   * @type {'up' | 'down' | 'left' | 'right'}
   */
  DrawerPopupDataAttributes["swipeDirection"] = "data-swipe-direction";
  /**
   * Present when the drawer is being swiped.
   */
  DrawerPopupDataAttributes["swiping"] = "data-swiping";
  return DrawerPopupDataAttributes;
}({});