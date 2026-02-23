import * as React from 'react';
import type { ProgressRoot } from "../root/ProgressRoot.js";
import type { BaseUIComponentProps } from "../../utils/types.js";
/**
 * Visualizes the completion status of the task.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Progress](https://base-ui.com/react/components/progress)
 */
export declare const ProgressIndicator: React.ForwardRefExoticComponent<Omit<ProgressIndicatorProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface ProgressIndicatorProps extends BaseUIComponentProps<'div', ProgressRoot.State> {}
export declare namespace ProgressIndicator {
  type Props = ProgressIndicatorProps;
}