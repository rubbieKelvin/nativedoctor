import * as React from 'react';
import { FloatingPortal } from "../../floating-ui-react/index.js";
/**
 * A portal element that moves the popup to a different part of the DOM.
 * By default, the portal element is appended to `<body>`.
 * Renders a `<div>` element.
 */
export declare const ComboboxPortal: React.ForwardRefExoticComponent<Omit<ComboboxPortalProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export declare namespace ComboboxPortal {
  interface State {}
}
export interface ComboboxPortalProps extends FloatingPortal.Props<ComboboxPortal.State> {
  /**
   * Whether to keep the portal mounted in the DOM while the popup is hidden.
   * @default false
   */
  keepMounted?: boolean | undefined;
}
export declare namespace ComboboxPortal {
  type Props = ComboboxPortalProps;
}