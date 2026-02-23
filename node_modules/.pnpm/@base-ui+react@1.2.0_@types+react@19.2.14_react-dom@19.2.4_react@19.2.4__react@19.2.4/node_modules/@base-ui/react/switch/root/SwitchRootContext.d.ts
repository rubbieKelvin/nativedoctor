import * as React from 'react';
import type { SwitchRoot } from "./SwitchRoot.js";
export type SwitchRootContext = SwitchRoot.State;
export declare const SwitchRootContext: React.Context<import("./SwitchRoot.js").SwitchRootState | undefined>;
export declare function useSwitchRootContext(): import("./SwitchRoot.js").SwitchRootState;