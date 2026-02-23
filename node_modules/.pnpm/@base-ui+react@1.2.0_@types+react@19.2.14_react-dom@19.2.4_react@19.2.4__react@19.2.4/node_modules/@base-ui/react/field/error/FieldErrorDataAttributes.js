"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.FieldErrorDataAttributes = void 0;
let FieldErrorDataAttributes = exports.FieldErrorDataAttributes = /*#__PURE__*/function (FieldErrorDataAttributes) {
  /**
   * Present when the field is disabled.
   */
  FieldErrorDataAttributes["disabled"] = "data-disabled";
  /**
   * Present when the field is in valid state.
   */
  FieldErrorDataAttributes["valid"] = "data-valid";
  /**
   * Present when the field is in invalid state.
   */
  FieldErrorDataAttributes["invalid"] = "data-invalid";
  /**
   * Present when the field has been touched.
   */
  FieldErrorDataAttributes["touched"] = "data-touched";
  /**
   * Present when the field's value has changed.
   */
  FieldErrorDataAttributes["dirty"] = "data-dirty";
  /**
   * Present when the field is filled.
   */
  FieldErrorDataAttributes["filled"] = "data-filled";
  /**
   * Present when the field control is focused.
   */
  FieldErrorDataAttributes["focused"] = "data-focused";
  return FieldErrorDataAttributes;
}({});