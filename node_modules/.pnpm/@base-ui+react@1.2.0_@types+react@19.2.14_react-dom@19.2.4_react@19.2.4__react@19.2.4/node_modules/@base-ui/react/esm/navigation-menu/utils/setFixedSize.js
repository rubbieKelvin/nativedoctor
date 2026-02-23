import { getCssDimensions } from "../../utils/getCssDimensions.js";
import { NavigationMenuPopupCssVars } from "../popup/NavigationMenuPopupCssVars.js";
import { NavigationMenuPositionerCssVars } from "../positioner/NavigationMenuPositionerCssVars.js";
export function setFixedSize(element, type) {
  const {
    width,
    height
  } = getCssDimensions(element);
  element.style.setProperty(type === 'popup' ? NavigationMenuPopupCssVars.popupWidth : NavigationMenuPositionerCssVars.positionerWidth, `${width}px`);
  element.style.setProperty(type === 'popup' ? NavigationMenuPopupCssVars.popupHeight : NavigationMenuPositionerCssVars.positionerHeight, `${height}px`);
}