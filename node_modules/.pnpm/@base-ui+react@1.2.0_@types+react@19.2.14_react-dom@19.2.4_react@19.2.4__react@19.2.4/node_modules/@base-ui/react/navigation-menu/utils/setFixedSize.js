"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.setFixedSize = setFixedSize;
var _getCssDimensions = require("../../utils/getCssDimensions");
var _NavigationMenuPopupCssVars = require("../popup/NavigationMenuPopupCssVars");
var _NavigationMenuPositionerCssVars = require("../positioner/NavigationMenuPositionerCssVars");
function setFixedSize(element, type) {
  const {
    width,
    height
  } = (0, _getCssDimensions.getCssDimensions)(element);
  element.style.setProperty(type === 'popup' ? _NavigationMenuPopupCssVars.NavigationMenuPopupCssVars.popupWidth : _NavigationMenuPositionerCssVars.NavigationMenuPositionerCssVars.positionerWidth, `${width}px`);
  element.style.setProperty(type === 'popup' ? _NavigationMenuPopupCssVars.NavigationMenuPopupCssVars.popupHeight : _NavigationMenuPositionerCssVars.NavigationMenuPositionerCssVars.positionerHeight, `${height}px`);
}