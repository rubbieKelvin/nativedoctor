import * as React from 'react';
import type { BaseUIComponentProps } from "../../utils/types.js";
import type { TabsRoot } from "../root/TabsRoot.js";
import type { TabsTab } from "../tab/TabsTab.js";
/**
 * A visual indicator that can be styled to match the position of the currently active tab.
 * Renders a `<span>` element.
 *
 * Documentation: [Base UI Tabs](https://base-ui.com/react/components/tabs)
 */
export declare const TabsIndicator: React.ForwardRefExoticComponent<Omit<TabsIndicatorProps, "ref"> & React.RefAttributes<HTMLSpanElement>>;
export interface TabsIndicatorState extends TabsRoot.State {
  activeTabPosition: TabsTab.Position | null;
  activeTabSize: TabsTab.Size | null;
  orientation: TabsRoot.Orientation;
}
export interface TabsIndicatorProps extends BaseUIComponentProps<'span', TabsIndicator.State> {
  /**
   * Whether to render itself before React hydrates.
   * This minimizes the time that the indicator isn’t visible after server-side rendering.
   * @default false
   */
  renderBeforeHydration?: boolean | undefined;
}
export declare namespace TabsIndicator {
  type State = TabsIndicatorState;
  type Props = TabsIndicatorProps;
}