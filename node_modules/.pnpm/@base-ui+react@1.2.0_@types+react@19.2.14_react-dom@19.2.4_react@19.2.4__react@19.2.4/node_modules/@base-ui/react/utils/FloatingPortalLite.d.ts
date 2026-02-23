import * as React from 'react';
import { type FloatingPortal } from "../floating-ui-react/index.js";
/**
 * `FloatingPortal` includes tabbable logic handling for focus management.
 * For components that don't need tabbable logic, use `FloatingPortalLite`.
 * @internal
 */
export declare const FloatingPortalLite: React.ForwardRefExoticComponent<Omit<FloatingPortalLite.Props<any>, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface FloatingPortalLiteProps<State> extends FloatingPortal.Props<State> {}
export declare namespace FloatingPortalLite {
  type Props<State> = FloatingPortalLiteProps<State>;
}