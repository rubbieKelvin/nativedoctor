<script setup lang="ts">
import {
    ContextMenu,
    ContextMenuContent,
    ContextMenuItem,
    ContextMenuSeparator,
    ContextMenuTrigger,
} from "@/components/ui/context-menu";
import { Globe, ListOrdered, FolderPlus, Pencil, Trash2 } from "lucide-vue-next";
import { useFolderActions } from "../actions";

const props = defineProps<{
    folderId: string;
}>();

const {
    addHttpToFolder,
    addSequenceToFolder,
    addSubfolder,
    renameFolder,
    deleteFolder,
} = useFolderActions(props.folderId);
</script>

<template>
    <ContextMenu>
        <ContextMenuTrigger as-child>
            <slot />
        </ContextMenuTrigger>
        <ContextMenuContent class="w-48">
            <ContextMenuItem @click="addHttpToFolder">
                <Globe class="mr-2 size-4" />
                Add HTTP
            </ContextMenuItem>
            <ContextMenuItem @click="addSequenceToFolder">
                <ListOrdered class="mr-2 size-4" />
                Add Sequence
            </ContextMenuItem>
            <ContextMenuItem @click="addSubfolder">
                <FolderPlus class="mr-2 size-4" />
                Add Subfolder
            </ContextMenuItem>
            <ContextMenuSeparator />
            <ContextMenuItem @click="renameFolder">
                <Pencil class="mr-2 size-4" />
                Rename
            </ContextMenuItem>
            <ContextMenuItem
                class="text-destructive focus:text-destructive"
                @click="deleteFolder"
            >
                <Trash2 class="mr-2 size-4" />
                Delete
            </ContextMenuItem>
        </ContextMenuContent>
    </ContextMenu>
</template>
