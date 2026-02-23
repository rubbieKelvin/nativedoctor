import * as React from 'react';
import { BaseUIComponentProps } from "../../utils/types.js";
/**
 * An element placed before <Drawer.Indent> to render a background layer
 * that can be styled based on whether any drawer is open.
 */
export declare const DrawerIndentBackground: React.ForwardRefExoticComponent<Omit<DrawerIndentBackgroundProps, "ref"> & React.RefAttributes<HTMLDivElement>>;
export interface DrawerIndentBackgroundState {
  /**
   * Whether any drawer within the nearest <Drawer.Provider> is open.
   */
  active: boolean;
}
export interface DrawerIndentBackgroundProps extends BaseUIComponentProps<'div', DrawerIndentBackground.State> {}
export declare namespace DrawerIndentBackground {
  type State = DrawerIndentBackgroundState;
  type Props = DrawerIndentBackgroundProps;
}