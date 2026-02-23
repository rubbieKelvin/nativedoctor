import * as React from 'react';
import { BaseUIComponentProps, NativeButtonProps } from "../../utils/types.js";
import type { AccordionItem } from "../item/AccordionItem.js";
/**
 * A button that opens and closes the corresponding panel.
 * Renders a `<button>` element.
 *
 * Documentation: [Base UI Accordion](https://base-ui.com/react/components/accordion)
 */
export declare const AccordionTrigger: React.ForwardRefExoticComponent<Omit<AccordionTriggerProps, "ref"> & React.RefAttributes<HTMLElement>>;
export interface AccordionTriggerProps extends NativeButtonProps, BaseUIComponentProps<'button', AccordionItem.State> {}
export declare namespace AccordionTrigger {
  type Props = AccordionTriggerProps;
}