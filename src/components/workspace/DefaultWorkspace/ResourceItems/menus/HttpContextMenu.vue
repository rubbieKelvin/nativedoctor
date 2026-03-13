<script setup lang="ts">
import {
    ContextMenu,
    ContextMenuContent,
    ContextMenuItem,
    ContextMenuSeparator,
    ContextMenuTrigger,
} from "@/components/ui/context-menu";
import { Copy, ClipboardCopy, Pencil, Trash2 } from "lucide-vue-next";
import { useHttpActions } from "../actions";

const props = defineProps<{
    resourceId: string;
}>();

const { duplicateHttp, copyAsCurl, renameHttp, deleteHttp } = useHttpActions(
    props.resourceId
);
</script>

<template>
    <ContextMenu>
        <ContextMenuTrigger as-child>
            <slot />
        </ContextMenuTrigger>
        <ContextMenuContent class="w-48">
            <ContextMenuItem @click="duplicateHttp">
                <Copy class="mr-2 size-4" />
                Duplicate
            </ContextMenuItem>
            <ContextMenuItem @click="copyAsCurl">
                <ClipboardCopy class="mr-2 size-4" />
                Copy as cURL
            </ContextMenuItem>
            <ContextMenuSeparator />
            <ContextMenuItem @click="renameHttp">
                <Pencil class="mr-2 size-4" />
                Rename
            </ContextMenuItem>
            <ContextMenuItem
                class="text-destructive focus:text-destructive"
                @click="deleteHttp"
            >
                <Trash2 class="mr-2 size-4" />
                Delete
            </ContextMenuItem>
        </ContextMenuContent>
    </ContextMenu>
</template>
