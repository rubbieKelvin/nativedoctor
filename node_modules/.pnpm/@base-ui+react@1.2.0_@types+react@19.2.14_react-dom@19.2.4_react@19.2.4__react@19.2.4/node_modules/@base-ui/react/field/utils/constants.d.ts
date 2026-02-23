export declare const DEFAULT_VALIDITY_STATE: {
  badInput: boolean;
  customError: boolean;
  patternMismatch: boolean;
  rangeOverflow: boolean;
  rangeUnderflow: boolean;
  stepMismatch: boolean;
  tooLong: boolean;
  tooShort: boolean;
  typeMismatch: boolean;
  valid: null;
  valueMissing: boolean;
};
export declare const fieldValidityMapping: {
  valid(value: boolean | null): Record<string, string> | null;
};