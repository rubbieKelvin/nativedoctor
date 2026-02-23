'use client';

import * as React from 'react';
import { useMergedRefs } from '@base-ui/utils/useMergedRefs';
import { useButton } from "../../use-button/index.js";
import { mergeProps } from "../../merge-props/index.js";
import { useMenuItemCommonProps } from "./useMenuItemCommonProps.js";
export const REGULAR_ITEM = {
  type: 'regular-item'
};
export function useMenuItem(params) {
  const {
    closeOnClick,
    disabled = false,
    highlighted,
    id,
    store,
    nativeButton,
    itemMetadata,
    nodeId
  } = params;
  const itemRef = React.useRef(null);
  const {
    getButtonProps,
    buttonRef
  } = useButton({
    disabled,
    focusableWhenDisabled: true,
    native: nativeButton
  });
  const commonProps = useMenuItemCommonProps({
    closeOnClick,
    highlighted,
    id,
    nodeId,
    store,
    itemRef,
    itemMetadata
  });
  const getItemProps = React.useCallback(externalProps => {
    return mergeProps(commonProps, {
      onMouseEnter() {
        if (itemMetadata.type !== 'submenu-trigger') {
          return;
        }
        itemMetadata.setActive();
      },
      onKeyUp(event) {
        if (event.key === ' ' && store.context.typingRef.current) {
          event.preventBaseUIHandler();
        }
      }
    }, externalProps, getButtonProps);
  }, [commonProps, getButtonProps, store, itemMetadata]);
  const mergedRef = useMergedRefs(itemRef, buttonRef);
  return React.useMemo(() => ({
    getItemProps,
    itemRef: mergedRef
  }), [getItemProps, mergedRef]);
}