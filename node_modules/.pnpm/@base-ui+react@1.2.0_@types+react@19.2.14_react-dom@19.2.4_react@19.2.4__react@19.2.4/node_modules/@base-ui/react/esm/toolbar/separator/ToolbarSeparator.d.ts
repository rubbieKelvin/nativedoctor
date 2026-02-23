import * as React from 'react';
import type { BaseUIComponentProps } from "../../utils/types.js";
import { Separator } from "../../separator/index.js";
/**
 * A separator element accessible to screen readers.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Toolbar](https://base-ui.com/react/components/toolbar)
 */
export declare const ToolbarSeparator: React.ForwardRefExoticComponent<Omit<ToolbarSeparatorProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface ToolbarSeparatorProps extends BaseUIComponentProps<'div', Separator.State>, Separator.Props {}
export declare namespace ToolbarSeparator {
  type Props = ToolbarSeparatorProps;
}