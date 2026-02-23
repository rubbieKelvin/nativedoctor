"use strict";
'use client';

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard").default;
Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.PopoverClose = void 0;
var React = _interopRequireWildcard(require("react"));
var _PopoverRootContext = require("../root/PopoverRootContext");
var _useRenderElement = require("../../utils/useRenderElement");
var _useButton = require("../../use-button");
var _createBaseUIEventDetails = require("../../utils/createBaseUIEventDetails");
var _reasons = require("../../utils/reasons");
/**
 * A button that closes the popover.
 * Renders a `<button>` element.
 *
 * Documentation: [Base UI Popover](https://base-ui.com/react/components/popover)
 */
const PopoverClose = exports.PopoverClose = /*#__PURE__*/React.forwardRef(function PopoverClose(props, forwardedRef) {
  const {
    render,
    className,
    disabled = false,
    nativeButton = true,
    ...elementProps
  } = props;
  const {
    buttonRef,
    getButtonProps
  } = (0, _useButton.useButton)({
    disabled,
    focusableWhenDisabled: false,
    native: nativeButton
  });
  const {
    store
  } = (0, _PopoverRootContext.usePopoverRootContext)();
  const element = (0, _useRenderElement.useRenderElement)('button', props, {
    ref: [forwardedRef, buttonRef],
    props: [{
      onClick(event) {
        store.setOpen(false, (0, _createBaseUIEventDetails.createChangeEventDetails)(_reasons.REASONS.closePress, event.nativeEvent, event.currentTarget));
      }
    }, elementProps, getButtonProps]
  });
  return element;
});
if (process.env.NODE_ENV !== "production") PopoverClose.displayName = "PopoverClose";