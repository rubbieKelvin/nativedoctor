import * as React from 'react';
import type { AccordionItem } from "./AccordionItem.js";
export interface AccordionItemContext {
  open: boolean;
  state: AccordionItem.State;
  setTriggerId: (id: string | undefined) => void;
  triggerId?: string | undefined;
}
export declare const AccordionItemContext: React.Context<AccordionItemContext | undefined>;
export declare function useAccordionItemContext(): AccordionItemContext;