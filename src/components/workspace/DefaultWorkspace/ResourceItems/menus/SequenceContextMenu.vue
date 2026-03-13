<script setup lang="ts">
import {
    ContextMenu,
    ContextMenuContent,
    ContextMenuItem,
    ContextMenuSeparator,
    ContextMenuTrigger,
} from "@/components/ui/context-menu";
import { Play, Copy, Pencil, Trash2 } from "lucide-vue-next";
import { useSequenceActions } from "../actions";

const props = defineProps<{
    resourceId: string;
}>();

const { runSequence, duplicateSequence, renameSequence, deleteSequence } =
    useSequenceActions(props.resourceId);
</script>

<template>
    <ContextMenu>
        <ContextMenuTrigger as-child>
            <slot />
        </ContextMenuTrigger>
        <ContextMenuContent class="w-48">
            <ContextMenuItem @click="runSequence">
                <Play class="mr-2 size-4" />
                Run
            </ContextMenuItem>
            <ContextMenuItem @click="duplicateSequence">
                <Copy class="mr-2 size-4" />
                Duplicate
            </ContextMenuItem>
            <ContextMenuSeparator />
            <ContextMenuItem @click="renameSequence">
                <Pencil class="mr-2 size-4" />
                Rename
            </ContextMenuItem>
            <ContextMenuItem
                class="text-destructive focus:text-destructive"
                @click="deleteSequence"
            >
                <Trash2 class="mr-2 size-4" />
                Delete
            </ContextMenuItem>
        </ContextMenuContent>
    </ContextMenu>
</template>
