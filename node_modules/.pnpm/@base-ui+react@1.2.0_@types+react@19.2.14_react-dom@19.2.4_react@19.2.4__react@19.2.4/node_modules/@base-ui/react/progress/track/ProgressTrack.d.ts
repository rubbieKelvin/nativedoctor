import * as React from 'react';
import type { ProgressRoot } from "../root/ProgressRoot.js";
import type { BaseUIComponentProps } from "../../utils/types.js";
/**
 * Contains the progress bar indicator.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Progress](https://base-ui.com/react/components/progress)
 */
export declare const ProgressTrack: React.ForwardRefExoticComponent<Omit<ProgressTrackProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface ProgressTrackProps extends BaseUIComponentProps<'div', ProgressRoot.State> {}
export declare namespace ProgressTrack {
  type Props = ProgressTrackProps;
}