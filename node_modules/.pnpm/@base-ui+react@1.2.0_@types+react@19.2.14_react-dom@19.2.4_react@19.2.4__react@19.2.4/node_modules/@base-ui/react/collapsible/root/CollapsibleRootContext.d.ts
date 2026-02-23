import * as React from 'react';
import type { useCollapsibleRoot } from "./useCollapsibleRoot.js";
import type { CollapsibleRoot } from "./CollapsibleRoot.js";
import type { TransitionStatus } from "../../utils/useTransitionStatus.js";
export interface CollapsibleRootContext extends useCollapsibleRoot.ReturnValue {
  onOpenChange: (open: boolean, eventDetails: CollapsibleRoot.ChangeEventDetails) => void;
  state: CollapsibleRoot.State;
  transitionStatus: TransitionStatus;
}
export declare const CollapsibleRootContext: React.Context<CollapsibleRootContext | undefined>;
export declare function useCollapsibleRootContext(): CollapsibleRootContext;