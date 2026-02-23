import { type DialogRoot } from "./DialogRoot.js";
import { DialogStore } from "../store/DialogStore.js";
export declare function useDialogRoot(params: useDialogRoot.Parameters): useDialogRoot.ReturnValue;
export interface UseDialogRootSharedParameters {}
export interface UseDialogRootParameters {
  store: DialogStore<any>;
  actionsRef?: DialogRoot.Props['actionsRef'] | undefined;
  parentContext?: DialogStore<unknown>['context'] | undefined;
  onOpenChange: DialogRoot.Props['onOpenChange'];
  triggerIdProp?: (string | null) | undefined;
}
export type UseDialogRootReturnValue = void;
export declare namespace useDialogRoot {
  type SharedParameters = UseDialogRootSharedParameters;
  type Parameters = UseDialogRootParameters;
  type ReturnValue = UseDialogRootReturnValue;
}