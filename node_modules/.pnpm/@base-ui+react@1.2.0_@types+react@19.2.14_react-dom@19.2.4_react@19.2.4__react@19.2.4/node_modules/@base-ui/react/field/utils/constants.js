"use strict";

Object.defineProperty(exports, "__esModule", {
  value: true
});
exports.fieldValidityMapping = exports.DEFAULT_VALIDITY_STATE = void 0;
var _FieldControlDataAttributes = require("../control/FieldControlDataAttributes");
const DEFAULT_VALIDITY_STATE = exports.DEFAULT_VALIDITY_STATE = {
  badInput: false,
  customError: false,
  patternMismatch: false,
  rangeOverflow: false,
  rangeUnderflow: false,
  stepMismatch: false,
  tooLong: false,
  tooShort: false,
  typeMismatch: false,
  valid: null,
  valueMissing: false
};
const fieldValidityMapping = exports.fieldValidityMapping = {
  valid(value) {
    if (value === null) {
      return null;
    }
    if (value) {
      return {
        [_FieldControlDataAttributes.FieldControlDataAttributes.valid]: ''
      };
    }
    return {
      [_FieldControlDataAttributes.FieldControlDataAttributes.invalid]: ''
    };
  }
};