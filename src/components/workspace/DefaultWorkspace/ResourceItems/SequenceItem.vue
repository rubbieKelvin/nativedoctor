<script setup lang="ts">
import type { SequenceResource } from "@/shared/types/resources";
import { ListOrdered } from "lucide-vue-next";
import { SequenceContextMenu } from "./menus";

defineProps<{
    resource: SequenceResource;
}>();

defineEmits<{
    (e: "select", id: string): void;
}>();
</script>

<template>
    <SequenceContextMenu :resource-id="resource.id">
        <button
            class="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-sm text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
            @click="$emit('select', resource.id)"
        >
            <ListOrdered class="size-4 shrink-0 text-purple-500" />
            <span class="truncate">{{ resource.name || "Untitled sequence" }}</span>
            <span
                v-if="resource.flow.length > 0"
                class="ml-auto text-xs text-muted-foreground"
            >
                {{ resource.flow.length }} step{{ resource.flow.length === 1 ? "" : "s" }}
            </span>
        </button>
    </SequenceContextMenu>
</template>
