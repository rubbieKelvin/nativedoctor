"use strict";
'use client';

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard").default;
Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.MeterLabel = void 0;
var React = _interopRequireWildcard(require("react"));
var _useIsoLayoutEffect = require("@base-ui/utils/useIsoLayoutEffect");
var _useBaseUiId = require("../../utils/useBaseUiId");
var _MeterRootContext = require("../root/MeterRootContext");
var _useRenderElement = require("../../utils/useRenderElement");
/**
 * An accessible label for the meter.
 * Renders a `<span>` element.
 *
 * Documentation: [Base UI Meter](https://base-ui.com/react/components/meter)
 */
const MeterLabel = exports.MeterLabel = /*#__PURE__*/React.forwardRef(function MeterLabel(componentProps, forwardedRef) {
  const {
    render,
    className,
    id: idProp,
    ...elementProps
  } = componentProps;
  const id = (0, _useBaseUiId.useBaseUiId)(idProp);
  const {
    setLabelId
  } = (0, _MeterRootContext.useMeterRootContext)();
  (0, _useIsoLayoutEffect.useIsoLayoutEffect)(() => {
    setLabelId(id);
    return () => setLabelId(undefined);
  }, [id, setLabelId]);
  return (0, _useRenderElement.useRenderElement)('span', componentProps, {
    ref: forwardedRef,
    props: [{
      id
    }, elementProps]
  });
});
if (process.env.NODE_ENV !== "production") MeterLabel.displayName = "MeterLabel";