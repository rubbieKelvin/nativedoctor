'use client';

import * as React from 'react';
import { useIsoLayoutEffect } from '@base-ui/utils/useIsoLayoutEffect';
import { useBaseUiId } from "../../utils/useBaseUiId.js";
import { useRenderElement } from "../../utils/useRenderElement.js";
import { useProgressRootContext } from "../root/ProgressRootContext.js";
import { progressStateAttributesMapping } from "../root/stateAttributesMapping.js";
/**
 * An accessible label for the progress bar.
 * Renders a `<span>` element.
 *
 * Documentation: [Base UI Progress](https://base-ui.com/react/components/progress)
 */
export const ProgressLabel = /*#__PURE__*/React.forwardRef(function ProgressLabel(componentProps, forwardedRef) {
  const {
    render,
    className,
    id: idProp,
    ...elementProps
  } = componentProps;
  const id = useBaseUiId(idProp);
  const {
    setLabelId,
    state
  } = useProgressRootContext();
  useIsoLayoutEffect(() => {
    setLabelId(id);
    return () => setLabelId(undefined);
  }, [id, setLabelId]);
  const element = useRenderElement('span', componentProps, {
    state,
    ref: forwardedRef,
    props: [{
      id
    }, elementProps],
    stateAttributesMapping: progressStateAttributesMapping
  });
  return element;
});
if (process.env.NODE_ENV !== "production") ProgressLabel.displayName = "ProgressLabel";