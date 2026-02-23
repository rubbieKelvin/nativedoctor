'use client';

import * as React from 'react';
import { useIsoLayoutEffect } from '@base-ui/utils/useIsoLayoutEffect';
import { useBaseUiId } from "../../utils/useBaseUiId.js";
import { useMeterRootContext } from "../root/MeterRootContext.js";
import { useRenderElement } from "../../utils/useRenderElement.js";

/**
 * An accessible label for the meter.
 * Renders a `<span>` element.
 *
 * Documentation: [Base UI Meter](https://base-ui.com/react/components/meter)
 */
export const MeterLabel = /*#__PURE__*/React.forwardRef(function MeterLabel(componentProps, forwardedRef) {
  const {
    render,
    className,
    id: idProp,
    ...elementProps
  } = componentProps;
  const id = useBaseUiId(idProp);
  const {
    setLabelId
  } = useMeterRootContext();
  useIsoLayoutEffect(() => {
    setLabelId(id);
    return () => setLabelId(undefined);
  }, [id, setLabelId]);
  return useRenderElement('span', componentProps, {
    ref: forwardedRef,
    props: [{
      id
    }, elementProps]
  });
});
if (process.env.NODE_ENV !== "production") MeterLabel.displayName = "MeterLabel";