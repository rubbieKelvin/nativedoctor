import * as React from 'react';
import { type Side, useAnchorPositioning } from "../../utils/useAnchorPositioning.js";
export interface SelectPositionerContext extends Omit<useAnchorPositioning.ReturnValue, 'side'> {
  side: 'none' | Side;
  alignItemWithTriggerActive: boolean;
  setControlledAlignItemWithTrigger: React.Dispatch<React.SetStateAction<boolean>>;
  scrollUpArrowRef: React.RefObject<HTMLDivElement | null>;
  scrollDownArrowRef: React.RefObject<HTMLDivElement | null>;
}
export declare const SelectPositionerContext: React.Context<SelectPositionerContext | undefined>;
export declare function useSelectPositionerContext(): SelectPositionerContext;