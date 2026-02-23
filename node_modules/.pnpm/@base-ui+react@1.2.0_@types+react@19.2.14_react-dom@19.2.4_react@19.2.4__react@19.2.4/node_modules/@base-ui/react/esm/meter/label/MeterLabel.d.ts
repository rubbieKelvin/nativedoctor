import * as React from 'react';
import type { MeterRoot } from "../root/MeterRoot.js";
import { BaseUIComponentProps } from "../../utils/types.js";
/**
 * An accessible label for the meter.
 * Renders a `<span>` element.
 *
 * Documentation: [Base UI Meter](https://base-ui.com/react/components/meter)
 */
export declare const MeterLabel: React.ForwardRefExoticComponent<Omit<MeterLabelProps, "ref"> & React.RefAttributes<HTMLSpanElement>>;
export interface MeterLabelProps extends BaseUIComponentProps<'span', MeterRoot.State> {}
export declare namespace MeterLabel {
  type Props = MeterLabelProps;
}