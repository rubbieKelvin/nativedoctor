"use strict";
'use client';

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard").default;
Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.ProgressLabel = void 0;
var React = _interopRequireWildcard(require("react"));
var _useIsoLayoutEffect = require("@base-ui/utils/useIsoLayoutEffect");
var _useBaseUiId = require("../../utils/useBaseUiId");
var _useRenderElement = require("../../utils/useRenderElement");
var _ProgressRootContext = require("../root/ProgressRootContext");
var _stateAttributesMapping = require("../root/stateAttributesMapping");
/**
 * An accessible label for the progress bar.
 * Renders a `<span>` element.
 *
 * Documentation: [Base UI Progress](https://base-ui.com/react/components/progress)
 */
const ProgressLabel = exports.ProgressLabel = /*#__PURE__*/React.forwardRef(function ProgressLabel(componentProps, forwardedRef) {
  const {
    render,
    className,
    id: idProp,
    ...elementProps
  } = componentProps;
  const id = (0, _useBaseUiId.useBaseUiId)(idProp);
  const {
    setLabelId,
    state
  } = (0, _ProgressRootContext.useProgressRootContext)();
  (0, _useIsoLayoutEffect.useIsoLayoutEffect)(() => {
    setLabelId(id);
    return () => setLabelId(undefined);
  }, [id, setLabelId]);
  const element = (0, _useRenderElement.useRenderElement)('span', componentProps, {
    state,
    ref: forwardedRef,
    props: [{
      id
    }, elementProps],
    stateAttributesMapping: _stateAttributesMapping.progressStateAttributesMapping
  });
  return element;
});
if (process.env.NODE_ENV !== "production") ProgressLabel.displayName = "ProgressLabel";