import { useResources } from "@/store/resources";

export function useFolderActions(folderId: string) {
  const store = useResources();

  function addHttpToFolder() {
    store.createHttpResource(folderId);
  }

  function addSequenceToFolder() {
    store.createSequenceResource(folderId);
  }

  function addSubfolder() {
    store.createFolderResource(folderId);
  }

  function renameFolder() {
    // Defer so the context menu closes and restores focus first; then the input can receive focus without an immediate blur.
    setTimeout(() => store.startRenaming(folderId), 0);
  }

  function deleteFolder() {
    store.deleteResource(folderId);
  }

  return {
    addHttpToFolder,
    addSequenceToFolder,
    addSubfolder,
    renameFolder,
    deleteFolder,
  };
}
