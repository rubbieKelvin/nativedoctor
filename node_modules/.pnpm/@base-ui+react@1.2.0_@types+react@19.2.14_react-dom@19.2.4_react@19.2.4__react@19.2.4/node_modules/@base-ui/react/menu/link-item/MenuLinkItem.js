"use strict";
'use client';

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard").default;
Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.MenuLinkItem = void 0;
var React = _interopRequireWildcard(require("react"));
var _MenuRootContext = require("../root/MenuRootContext");
var _useRenderElement = require("../../utils/useRenderElement");
var _useBaseUiId = require("../../utils/useBaseUiId");
var _useCompositeListItem = require("../../composite/list/useCompositeListItem");
var _MenuPositionerContext = require("../positioner/MenuPositionerContext");
var _useMenuItemCommonProps = require("../item/useMenuItemCommonProps");
/**
 * A link in the menu that can be used to navigate to a different page or section.
 * Renders an `<a>` element.
 *
 * Documentation: [Base UI Menu](https://base-ui.com/react/components/menu)
 */
const MenuLinkItem = exports.MenuLinkItem = /*#__PURE__*/React.forwardRef(function MenuLinkItem(componentProps, forwardedRef) {
  const {
    render,
    className,
    id: idProp,
    label,
    closeOnClick = false,
    ...elementProps
  } = componentProps;
  const linkRef = React.useRef(null);
  const listItem = (0, _useCompositeListItem.useCompositeListItem)({
    label
  });
  const menuPositionerContext = (0, _MenuPositionerContext.useMenuPositionerContext)(true);
  const nodeId = menuPositionerContext?.nodeId;
  const id = (0, _useBaseUiId.useBaseUiId)(idProp);
  const {
    store
  } = (0, _MenuRootContext.useMenuRootContext)();
  const highlighted = store.useState('isActive', listItem.index);
  const itemProps = store.useState('itemProps');
  const commonProps = (0, _useMenuItemCommonProps.useMenuItemCommonProps)({
    closeOnClick,
    highlighted,
    id,
    nodeId,
    store,
    itemRef: linkRef
  });
  const state = React.useMemo(() => ({
    highlighted
  }), [highlighted]);
  return (0, _useRenderElement.useRenderElement)('a', componentProps, {
    state,
    props: [itemProps, elementProps, commonProps],
    ref: [linkRef, forwardedRef, listItem.ref]
  });
});
if (process.env.NODE_ENV !== "production") MenuLinkItem.displayName = "MenuLinkItem";