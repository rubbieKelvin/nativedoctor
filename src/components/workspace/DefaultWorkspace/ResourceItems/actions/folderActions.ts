import { useCurrentProjectActions } from "@/store/project";

export function useFolderActions(folderId: string) {
  const store = useCurrentProjectActions();

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
    store.startRenaming(folderId);
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
