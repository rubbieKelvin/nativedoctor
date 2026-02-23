import * as React from 'react';
import type { ProgressRoot } from "../root/ProgressRoot.js";
import type { BaseUIComponentProps } from "../../utils/types.js";
/**
 * An accessible label for the progress bar.
 * Renders a `<span>` element.
 *
 * Documentation: [Base UI Progress](https://base-ui.com/react/components/progress)
 */
export declare const ProgressLabel: React.ForwardRefExoticComponent<Omit<ProgressLabelProps, "ref"> & React.RefAttributes<HTMLSpanElement>>;
export interface ProgressLabelProps extends BaseUIComponentProps<'span', ProgressRoot.State> {}
export declare namespace ProgressLabel {
  type Props = ProgressLabelProps;
}