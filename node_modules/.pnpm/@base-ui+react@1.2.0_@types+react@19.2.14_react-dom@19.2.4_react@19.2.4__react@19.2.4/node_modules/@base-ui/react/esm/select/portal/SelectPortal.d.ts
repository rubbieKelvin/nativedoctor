import * as React from 'react';
import { FloatingPortal } from "../../floating-ui-react/index.js";
/**
 * A portal element that moves the popup to a different part of the DOM.
 * By default, the portal element is appended to `<body>`.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Select](https://base-ui.com/react/components/select)
 */
export declare const SelectPortal: React.ForwardRefExoticComponent<Omit<SelectPortalProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export declare namespace SelectPortal {
  interface State {}
}
export interface SelectPortalProps extends FloatingPortal.Props<SelectPortal.State> {}
export declare namespace SelectPortal {
  type Props = SelectPortalProps;
}