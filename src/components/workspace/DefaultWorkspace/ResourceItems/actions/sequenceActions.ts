import { useResources } from "@/store/resources";

export function useSequenceActions(resourceId: string) {
  const store = useResources();

  function renameSequence() {
    store.startRenaming(resourceId);
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
