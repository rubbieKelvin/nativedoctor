import * as React from 'react';
import type { BaseUIComponentProps } from "../../utils/types.js";
import type { ProgressRoot } from "../root/ProgressRoot.js";
/**
 * A text label displaying the current value.
 * Renders a `<span>` element.
 *
 * Documentation: [Base UI Progress](https://base-ui.com/react/components/progress)
 */
export declare const ProgressValue: React.ForwardRefExoticComponent<Omit<ProgressValueProps, "ref"> & React.RefAttributes<HTMLSpanElement>>;
export interface ProgressValueProps extends Omit<BaseUIComponentProps<'span', ProgressRoot.State>, 'children'> {
  children?: (null | ((formattedValue: string | null, value: number | null) => React.ReactNode)) | undefined;
}
export declare namespace ProgressValue {
  type Props = ProgressValueProps;
}