import * as React from 'react';
import { BaseUIComponentProps, NativeButtonProps } from "../../utils/types.js";
import { CollapsibleRoot } from "../root/CollapsibleRoot.js";
/**
 * A button that opens and closes the collapsible panel.
 * Renders a `<button>` element.
 *
 * Documentation: [Base UI Collapsible](https://base-ui.com/react/components/collapsible)
 */
export declare const CollapsibleTrigger: React.ForwardRefExoticComponent<Omit<CollapsibleTriggerProps, "ref"> & React.RefAttributes<HTMLButtonElement>>;
export interface CollapsibleTriggerProps extends NativeButtonProps, BaseUIComponentProps<'button', CollapsibleRoot.State> {}
export declare namespace CollapsibleTrigger {
  type Props = CollapsibleTriggerProps;
}