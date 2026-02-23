'use client';

import * as React from 'react';
import { useDialogRootContext } from "../../dialog/root/DialogRootContext.js";
import { useRenderElement } from "../../utils/useRenderElement.js";

/**
 * A container for the drawer contents.
 * Renders a `<div>` element.
 *
 * Documentation: [Base UI Drawer](https://base-ui.com/react/components/drawer)
 */
export const DrawerContent = /*#__PURE__*/React.forwardRef(function DrawerContent(componentProps, forwardedRef) {
  const {
    render,
    className,
    ...elementProps
  } = componentProps;
  useDialogRootContext();
  return useRenderElement('div', componentProps, {
    ref: forwardedRef,
    props: [{
      ['data-swipe-ignore']: ''
    }, elementProps]
  });
});
if (process.env.NODE_ENV !== "production") DrawerContent.displayName = "DrawerContent";