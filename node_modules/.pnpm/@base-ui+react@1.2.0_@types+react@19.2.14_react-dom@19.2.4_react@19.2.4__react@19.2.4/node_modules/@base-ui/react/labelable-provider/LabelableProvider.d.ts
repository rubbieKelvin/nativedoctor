import * as React from 'react';
/**
 * @internal
 */
export declare const LabelableProvider: React.FC<LabelableProvider.Props>;
export interface LabelableProviderProps {
  initialControlId?: string | null | undefined;
  children?: React.ReactNode;
}
export declare namespace LabelableProvider {
  type Props = LabelableProviderProps;
}