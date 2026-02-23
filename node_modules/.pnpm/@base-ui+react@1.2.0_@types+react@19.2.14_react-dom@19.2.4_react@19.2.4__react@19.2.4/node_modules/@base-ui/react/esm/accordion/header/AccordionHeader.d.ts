import * as React from 'react';
import { BaseUIComponentProps } from "../../utils/types.js";
import type { AccordionItem } from "../item/AccordionItem.js";
/**
 * A heading that labels the corresponding panel.
 * Renders an `<h3>` element.
 *
 * Documentation: [Base UI Accordion](https://base-ui.com/react/components/accordion)
 */
export declare const AccordionHeader: React.ForwardRefExoticComponent<Omit<AccordionHeaderProps, "ref"> & React.RefAttributes<HTMLHeadingElement>>;
export interface AccordionHeaderProps extends BaseUIComponentProps<'h3', AccordionItem.State> {}
export declare namespace AccordionHeader {
  type Props = AccordionHeaderProps;
}