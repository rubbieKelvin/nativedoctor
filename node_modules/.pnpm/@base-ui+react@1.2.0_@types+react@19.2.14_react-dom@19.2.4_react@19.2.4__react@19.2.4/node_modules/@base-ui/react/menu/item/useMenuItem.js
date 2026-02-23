"use strict";
'use client';

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard").default;
Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.REGULAR_ITEM = void 0;
exports.useMenuItem = useMenuItem;
var React = _interopRequireWildcard(require("react"));
var _useMergedRefs = require("@base-ui/utils/useMergedRefs");
var _useButton = require("../../use-button");
var _mergeProps = require("../../merge-props");
var _useMenuItemCommonProps = require("./useMenuItemCommonProps");
const REGULAR_ITEM = exports.REGULAR_ITEM = {
  type: 'regular-item'
};
function useMenuItem(params) {
  const {
    closeOnClick,
    disabled = false,
    highlighted,
    id,
    store,
    nativeButton,
    itemMetadata,
    nodeId
  } = params;
  const itemRef = React.useRef(null);
  const {
    getButtonProps,
    buttonRef
  } = (0, _useButton.useButton)({
    disabled,
    focusableWhenDisabled: true,
    native: nativeButton
  });
  const commonProps = (0, _useMenuItemCommonProps.useMenuItemCommonProps)({
    closeOnClick,
    highlighted,
    id,
    nodeId,
    store,
    itemRef,
    itemMetadata
  });
  const getItemProps = React.useCallback(externalProps => {
    return (0, _mergeProps.mergeProps)(commonProps, {
      onMouseEnter() {
        if (itemMetadata.type !== 'submenu-trigger') {
          return;
        }
        itemMetadata.setActive();
      },
      onKeyUp(event) {
        if (event.key === ' ' && store.context.typingRef.current) {
          event.preventBaseUIHandler();
        }
      }
    }, externalProps, getButtonProps);
  }, [commonProps, getButtonProps, store, itemMetadata]);
  const mergedRef = (0, _useMergedRefs.useMergedRefs)(itemRef, buttonRef);
  return React.useMemo(() => ({
    getItemProps,
    itemRef: mergedRef
  }), [getItemProps, mergedRef]);
}