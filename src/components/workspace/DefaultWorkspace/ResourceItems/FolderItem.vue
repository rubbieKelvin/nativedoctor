<script setup lang="ts">
import { ref, computed } from "vue";
import type { FolderResource, Resource } from "@/shared/types/resources";
import { Folder, ChevronRight, ChevronDown } from "lucide-vue-next";
import { FolderContextMenu } from "./menus";
import ResourceItem from "./ResourceItem.vue";

defineOptions({ name: "FolderItem" });

const props = withDefaults(
    defineProps<{
        resource: FolderResource;
        depth?: number;
    }>(),
    { depth: 0 },
);

defineEmits<{
    (e: "select", id: string): void;
}>();

const isExpanded = ref(false);

const children = computed<Resource[]>(() => {
    return props.resource.children;
});

function toggleExpand() {
    isExpanded.value = !isExpanded.value;
}
</script>

<template>
    <div>
        <FolderContextMenu :folder-id="resource.id">
            <button
                class="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-sm text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
                :style="{ paddingLeft: `${depth * 12 + 8}px` }"
                @click="toggleExpand"
            >
                <component
                    :is="isExpanded ? ChevronDown : ChevronRight"
                    class="size-4 shrink-0 text-muted-foreground transition-transform"
                />
                <Folder class="size-4 shrink-0 text-amber-500" />
                <span class="truncate">{{
                    resource.name.trim() || "Untitled folder"
                }}</span>
                <span
                    v-if="children.length > 0"
                    class="ml-auto text-xs text-muted-foreground"
                >
                    {{ children.length }}
                </span>
            </button>
        </FolderContextMenu>
        <div v-if="isExpanded && children.length > 0">
            <ResourceItem
                v-for="child in children"
                :key="child.id"
                :resource="child"
                :depth="depth + 1"
                @select="$emit('select', $event)"
            />
        </div>
    </div>
</template>
