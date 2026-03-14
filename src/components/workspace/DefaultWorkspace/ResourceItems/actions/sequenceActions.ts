import { useResources } from "@/store/resources";

export function useSequenceActions(resourceId: string) {
  const store = useResources();

  function renameSequence() {
    // Defer so the context menu closes and restores focus first; then the input can receive focus without an immediate blur.
    setTimeout(() => store.startRenaming(resourceId), 0);
  }

  function deleteSequence() {
    store.deleteResource(resourceId);
  }

  function duplicateSequence() {
    store.duplicateResource(resourceId);
  }

  function runSequence() {
    // TODO: Implement sequence execution
    console.log("Running sequence:", resourceId);
  }

  return {
    renameSequence,
    deleteSequence,
    duplicateSequence,
    runSequence,
  };
}
