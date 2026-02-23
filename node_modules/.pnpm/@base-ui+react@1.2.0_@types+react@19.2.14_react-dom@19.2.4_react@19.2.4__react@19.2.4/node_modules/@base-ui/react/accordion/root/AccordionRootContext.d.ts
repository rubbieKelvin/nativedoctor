import * as React from 'react';
import type { Orientation } from "../../utils/types.js";
import type { TextDirection } from "../../direction-provider/index.js";
import type { AccordionRoot, AccordionValue } from "./AccordionRoot.js";
export interface AccordionRootContext {
  accordionItemRefs: React.RefObject<(HTMLElement | null)[]>;
  direction: TextDirection;
  disabled: boolean;
  handleValueChange: (newValue: number | string, nextOpen: boolean) => void;
  hiddenUntilFound: boolean;
  keepMounted: boolean;
  loopFocus: boolean;
  orientation: Orientation;
  state: AccordionRoot.State;
  value: AccordionValue;
}
export declare const AccordionRootContext: React.Context<AccordionRootContext | undefined>;
export declare function useAccordionRootContext(): AccordionRootContext;