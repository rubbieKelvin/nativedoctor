import * as React from 'react';
import type { CheckboxRoot } from "./CheckboxRoot.js";
export type CheckboxRootContext = CheckboxRoot.State;
export declare const CheckboxRootContext: React.Context<import("./CheckboxRoot.js").CheckboxRootState | undefined>;
export declare function useCheckboxRootContext(): import("./CheckboxRoot.js").CheckboxRootState;