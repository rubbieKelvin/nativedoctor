import * as React from 'react';
import { BaseUIComponentProps } from "../../utils/types.js";
import { useCollapsibleRoot } from "../../collapsible/root/useCollapsibleRoot.js";
import type { AccordionRoot } from "../root/AccordionRoot.js";
import { type BaseUIChangeEventDetails } from "../../utils/createBaseUIEventDetails.js";
import { REASONS } from "../../utils/reasons.js";
/**
 * Groups an accordion header with the corresponding panel.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Accordion](https://base-ui.com/react/components/accordion)
 */
export declare const AccordionItem: React.ForwardRefExoticComponent<Omit<AccordionItemProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface AccordionItemState extends AccordionRoot.State {
  index: number;
  open: boolean;
}
export interface AccordionItemProps extends BaseUIComponentProps<'div', AccordionItem.State>, Partial<Pick<useCollapsibleRoot.Parameters, 'disabled'>> {
  /**
   * A unique value that identifies this accordion item.
   * If no value is provided, a unique ID will be generated automatically.
   * Use when controlling the accordion programmatically, or to set an initial
   * open state.
   * @example
   * ```tsx
   * <Accordion.Root value={['a']}>
   *   <Accordion.Item value="a" /> // initially open
   *   <Accordion.Item value="b" /> // initially closed
   * </Accordion.Root>
   * ```
   */
  value?: any;
  /**
   * Event handler called when the panel is opened or closed.
   */
  onOpenChange?: ((open: boolean, eventDetails: AccordionItem.ChangeEventDetails) => void) | undefined;
}
export type AccordionItemChangeEventReason = typeof REASONS.triggerPress | typeof REASONS.none;
export type AccordionItemChangeEventDetails = BaseUIChangeEventDetails<AccordionItem.ChangeEventReason>;
export declare namespace AccordionItem {
  type State = AccordionItemState;
  type Props = AccordionItemProps;
  type ChangeEventReason = AccordionItemChangeEventReason;
  type ChangeEventDetails = AccordionItemChangeEventDetails;
}