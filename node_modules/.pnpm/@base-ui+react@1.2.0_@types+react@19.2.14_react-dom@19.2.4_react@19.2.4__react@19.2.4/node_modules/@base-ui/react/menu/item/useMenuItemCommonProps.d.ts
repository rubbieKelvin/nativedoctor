import * as React from 'react';
import { HTMLProps } from "../../utils/types.js";
import { MenuStore } from "../store/MenuStore.js";
import type { UseMenuItemMetadata } from "./useMenuItem.js";
export interface UseMenuItemCommonPropsParameters {
  /**
   * Whether to close the menu when the item is clicked.
   */
  closeOnClick: boolean;
  /**
   * Determines if the menu item is highlighted.
   */
  highlighted: boolean;
  /**
   * The id of the menu item.
   */
  id: string | undefined;
  /**
   * The node id of the menu positioner.
   */
  nodeId: string | undefined;
  /**
   * The menu store.
   */
  store: MenuStore<any>;
  /**
   * Ref to the item element.
   */
  itemRef: React.RefObject<HTMLElement | null>;
  /**
   * Optional metadata for checking item type before triggering click.
   * If provided, click will only be triggered for 'regular-item' type.
   */
  itemMetadata?: UseMenuItemMetadata | undefined;
}
/**
 * Returns common props shared by all menu item types.
 * This hook extracts the shared logic for id, role, tabIndex, onMouseMove, onClick, and onMouseUp handlers.
 */
export declare function useMenuItemCommonProps(params: UseMenuItemCommonPropsParameters): HTMLProps;
export declare namespace useMenuItemCommonProps {
  type Parameters = UseMenuItemCommonPropsParameters;
}