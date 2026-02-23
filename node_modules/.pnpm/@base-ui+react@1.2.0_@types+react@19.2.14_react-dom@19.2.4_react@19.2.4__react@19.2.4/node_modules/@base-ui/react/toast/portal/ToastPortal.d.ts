import { FloatingPortalLite } from "../../utils/FloatingPortalLite.js";
/**
 * A portal element that moves the viewport to a different part of the DOM.
 * By default, the portal element is appended to `<body>`.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Toast](https://base-ui.com/react/components/toast)
 */
export declare const ToastPortal: import("react").ForwardRefExoticComponent<Omit<FloatingPortalLite.Props<any>, "ref"> & import("react").RefAttributes<HTMLDivElement>>;
export declare namespace ToastPortal {
  interface State {}
}
export interface ToastPortalProps extends FloatingPortalLite.Props<ToastPortal.State> {}
export declare namespace ToastPortal {
  type Props = ToastPortalProps;
}