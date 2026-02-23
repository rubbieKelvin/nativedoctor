import * as React from 'react';
import { BaseUIComponentProps, Orientation } from "../../utils/types.js";
import { type BaseUIChangeEventDetails } from "../../utils/createBaseUIEventDetails.js";
import { REASONS } from "../../utils/reasons.js";
/**
 * Groups all parts of the accordion.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Accordion](https://base-ui.com/react/components/accordion)
 */
export declare const AccordionRoot: React.ForwardRefExoticComponent<Omit<AccordionRootProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export type AccordionValue = (any | null)[];
export interface AccordionRootState {
  value: AccordionValue;
  /**
   * Whether the component should ignore user interaction.
   */
  disabled: boolean;
  orientation: Orientation;
}
export interface AccordionRootProps extends BaseUIComponentProps<'div', AccordionRoot.State> {
  /**
   * The controlled value of the item(s) that should be expanded.
   *
   * To render an uncontrolled accordion, use the `defaultValue` prop instead.
   */
  value?: AccordionValue | undefined;
  /**
   * The uncontrolled value of the item(s) that should be initially expanded.
   *
   * To render a controlled accordion, use the `value` prop instead.
   */
  defaultValue?: AccordionValue | undefined;
  /**
   * Whether the component should ignore user interaction.
   * @default false
   */
  disabled?: boolean | undefined;
  /**
   * Allows the browser’s built-in page search to find and expand the panel contents.
   *
   * Overrides the `keepMounted` prop and uses `hidden="until-found"`
   * to hide the element without removing it from the DOM.
   * @default false
   */
  hiddenUntilFound?: boolean | undefined;
  /**
   * Whether to keep the element in the DOM while the panel is closed.
   * This prop is ignored when `hiddenUntilFound` is used.
   * @default false
   */
  keepMounted?: boolean | undefined;
  /**
   * Whether to loop keyboard focus back to the first item
   * when the end of the list is reached while using the arrow keys.
   * @default true
   */
  loopFocus?: boolean | undefined;
  /**
   * Event handler called when an accordion item is expanded or collapsed.
   * Provides the new value as an argument.
   */
  onValueChange?: ((value: AccordionValue, eventDetails: AccordionRootChangeEventDetails) => void) | undefined;
  /**
   * Whether multiple items can be open at the same time.
   * @default false
   */
  multiple?: boolean | undefined;
  /**
   * The visual orientation of the accordion.
   * Controls whether roving focus uses left/right or up/down arrow keys.
   * @default 'vertical'
   */
  orientation?: Orientation | undefined;
}
export type AccordionRootChangeEventReason = typeof REASONS.triggerPress | typeof REASONS.none;
export type AccordionRootChangeEventDetails = BaseUIChangeEventDetails<AccordionRoot.ChangeEventReason>;
export declare namespace AccordionRoot {
  type State = AccordionRootState;
  type Props = AccordionRootProps;
  type ChangeEventReason = AccordionRootChangeEventReason;
  type ChangeEventDetails = AccordionRootChangeEventDetails;
}