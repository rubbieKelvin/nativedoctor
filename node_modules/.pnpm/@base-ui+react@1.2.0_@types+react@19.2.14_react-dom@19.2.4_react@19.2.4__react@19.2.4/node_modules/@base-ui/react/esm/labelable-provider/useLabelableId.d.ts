import * as React from 'react';
export declare function useLabelableId(params?: useLabelableId.Parameters): string | undefined;
export interface UseLabelableIdParameters {
  id?: string | undefined;
  /**
   * Whether implicit labelling is supported.
   * @default false
   */
  implicit?: boolean | undefined;
  /**
   * A ref to an element that can be implicitly labelled.
   */
  controlRef?: React.RefObject<HTMLElement | null> | undefined;
}
export type UseLabelableIdReturnValue = string;
export declare namespace useLabelableId {
  type Parameters = UseLabelableIdParameters;
  type ReturnValue = UseLabelableIdReturnValue;
}