"use strict";
'use client';

var _interopRequireWildcard = require("@babel/runtime/helpers/interopRequireWildcard").default;
Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.FieldLabel = void 0;
var React = _interopRequireWildcard(require("react"));
var _error = require("@base-ui/utils/error");
var _safeReact = require("@base-ui/utils/safeReact");
var _dom = require("@floating-ui/utils/dom");
var _useIsoLayoutEffect = require("@base-ui/utils/useIsoLayoutEffect");
var _owner = require("@base-ui/utils/owner");
var _useStableCallback = require("@base-ui/utils/useStableCallback");
var _utils = require("../../floating-ui-react/utils");
var _FieldRootContext = require("../root/FieldRootContext");
var _LabelableContext = require("../../labelable-provider/LabelableContext");
var _constants = require("../utils/constants");
var _useBaseUiId = require("../../utils/useBaseUiId");
var _useRenderElement = require("../../utils/useRenderElement");
/**
 * An accessible label that is automatically associated with the field control.
 * Renders a `<label>` element.
 *
 * Documentation: [Base UI Field](https://base-ui.com/react/components/field)
 */
const FieldLabel = exports.FieldLabel = /*#__PURE__*/React.forwardRef(function FieldLabel(componentProps, forwardedRef) {
  const {
    render,
    className,
    id: idProp,
    nativeLabel = true,
    ...elementProps
  } = componentProps;
  const fieldRootContext = (0, _FieldRootContext.useFieldRootContext)(false);
  const {
    controlId,
    setLabelId,
    labelId
  } = (0, _LabelableContext.useLabelableContext)();
  const id = (0, _useBaseUiId.useBaseUiId)(idProp);
  const labelRef = React.useRef(null);
  const handleInteraction = (0, _useStableCallback.useStableCallback)(event => {
    const target = (0, _utils.getTarget)(event.nativeEvent);
    if (target?.closest('button,input,select,textarea')) {
      return;
    }

    // Prevent text selection when double clicking label.
    if (!event.defaultPrevented && event.detail > 1) {
      event.preventDefault();
    }
    if (nativeLabel || !controlId) {
      return;
    }
    const controlElement = (0, _owner.ownerDocument)(event.currentTarget).getElementById(controlId);
    if ((0, _dom.isHTMLElement)(controlElement)) {
      controlElement.focus({
        // Available from Chrome 144+ (January 2026).
        // Safari and Firefox already support it.
        // @ts-expect-error not available in types yet
        focusVisible: true
      });
    }
  });
  if (process.env.NODE_ENV !== 'production') {
    // eslint-disable-next-line react-hooks/rules-of-hooks
    React.useEffect(() => {
      if (!labelRef.current) {
        return;
      }
      const isLabelTag = labelRef.current.tagName === 'LABEL';
      if (nativeLabel) {
        if (!isLabelTag) {
          const ownerStackMessage = _safeReact.SafeReact.captureOwnerStack?.() || '';
          const message = '<Field.Label> expected a <label> element because the `nativeLabel` prop is true. ' + 'Rendering a non-<label> disables native label association, so `htmlFor` will not ' + 'work. Use a real <label> in the `render` prop, or set `nativeLabel` to `false`.';
          (0, _error.error)(`${message}${ownerStackMessage}`);
        }
      } else if (isLabelTag) {
        const ownerStackMessage = _safeReact.SafeReact.captureOwnerStack?.() || '';
        const message = '<Field.Label> expected a non-<label> element because the `nativeLabel` prop is false. ' + 'Rendering a <label> assumes native label behavior while Base UI treats it as ' + 'non-native, which can cause unexpected pointer behavior. Use a non-<label> in the ' + '`render` prop, or set `nativeLabel` to `true`.';
        (0, _error.error)(`${message}${ownerStackMessage}`);
      }
    }, [nativeLabel]);
  }
  (0, _useIsoLayoutEffect.useIsoLayoutEffect)(() => {
    if (id) {
      setLabelId(id);
    }
    return () => {
      setLabelId(undefined);
    };
  }, [id, setLabelId]);
  const element = (0, _useRenderElement.useRenderElement)('label', componentProps, {
    ref: [forwardedRef, labelRef],
    state: fieldRootContext.state,
    props: [{
      id: labelId
    }, nativeLabel ? {
      htmlFor: controlId ?? undefined,
      onMouseDown: handleInteraction
    } : {
      onClick: handleInteraction,
      onPointerDown(event) {
        event.preventDefault();
      }
    }, elementProps],
    stateAttributesMapping: _constants.fieldValidityMapping
  });
  return element;
});
if (process.env.NODE_ENV !== "production") FieldLabel.displayName = "FieldLabel";