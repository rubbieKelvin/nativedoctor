<script setup lang="ts">
import type { FolderResource } from "@/shared/types/resources";
import { Folder, ChevronRight } from "lucide-vue-next";
import { FolderContextMenu } from "./menus";

defineProps<{
    resource: FolderResource;
}>();

defineEmits<{
    (e: "select", id: string): void;
}>();
</script>

<template>
    <FolderContextMenu :folder-id="resource.id">
        <button
            class="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-sm text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
            @click="$emit('select', resource.id)"
        >
            <ChevronRight class="size-4 shrink-0 text-muted-foreground" />
            <Folder class="size-4 shrink-0 text-amber-500" />
            <span class="truncate">{{ resource.name || "Untitled folder" }}</span>
        </button>
    </FolderContextMenu>
</template>
