import * as React from 'react';
import type { SwitchRoot } from "../root/SwitchRoot.js";
import type { BaseUIComponentProps } from "../../utils/types.js";
/**
 * The movable part of the switch that indicates whether the switch is on or off.
 * Renders a `<span>`.
 *
 * Documentation: [Base UI Switch](https://base-ui.com/react/components/switch)
 */
export declare const SwitchThumb: React.ForwardRefExoticComponent<Omit<SwitchThumbProps, "ref"> & React.RefAttributes<HTMLSpanElement>>;
export interface SwitchThumbProps extends BaseUIComponentProps<'span', SwitchThumb.State> {}
export interface SwitchThumbState extends SwitchRoot.State {}
export declare namespace SwitchThumb {
  type Props = SwitchThumbProps;
  type State = SwitchThumbState;
}