<script setup lang="ts">
import { computed, ref, watch, nextTick } from "vue";
import type { FolderResource, Resource } from "@/shared/types/resources";
import { Folder, ChevronRight, ChevronDown } from "lucide-vue-next";
import { FolderContextMenu } from "./menus";
import ResourceItem from "./ResourceItem.vue";
import { useFolders } from "@/store/folders";
import { sortedResources } from "@/shared/resources";
import { useResourcesState, useResources } from "@/store/resources";

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

const { renamingResourceId } = useResourcesState();
const store = useResources();

/** Ignore blur for this long after entering rename (context menu close steals focus). */
const BLUR_GRACE_MS = 200;

const isRenaming = computed(
    () => renamingResourceId.value === props.resource.id,
);
const tempName = ref(props.resource.name);
const inputRef = ref<HTMLInputElement | null>(null);
const renameStartedAt = ref(0);

watch(isRenaming, (val) => {
    if (val) {
        renameStartedAt.value = Date.now();
        tempName.value = props.resource.name;
        nextTick(() => {
            setTimeout(() => {
                inputRef.value?.focus();
                inputRef.value?.select();
            }, 0);
        });
    }
});

function handleRename() {
    if (!isRenaming.value) return;
    if (Date.now() - renameStartedAt.value < BLUR_GRACE_MS) {
        inputRef.value?.focus();
        return;
    }
    if (tempName.value.trim() && tempName.value !== props.resource.name) {
        store.renameResource(props.resource.id, tempName.value.trim());
    }
    store.stopRenaming();
}

function cancelRename() {
    store.stopRenaming();
}

const folders = useFolders();
const isExpanded = computed(() => folders.isOpen(props.resource.id));
const children = computed<Resource[]>(() =>
    sortedResources(props.resource.children),
);

function toggleExpand() {
    folders.toggle(props.resource.id);
}
</script>

<template>
    <div>
        <FolderContextMenu :folder-id="resource.id">
            <button
                class="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-sm text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
                :style="{ paddingLeft: `${depth * 12 + 8}px` }"
                :disabled="isRenaming"
                @click="toggleExpand"
            >
                <component
                    :is="isExpanded ? ChevronDown : ChevronRight"
                    class="size-4 shrink-0 text-muted-foreground transition-transform"
                />
                <Folder class="size-4 shrink-0 text-amber-500" />
                <span v-if="!isRenaming" class="truncate">{{
                    resource.name.trim() || "Untitled folder"
                }}</span>
                <input
                    v-else
                    ref="inputRef"
                    v-model="tempName"
                    class="h-5 w-full min-w-0 flex-1 rounded-sm border-none bg-sidebar-accent-foreground/10 px-1 text-sm outline-none ring-1 ring-ring"
                    autocorrect="off"
                    autocomplete="off"
                    autocapitalize="none"
                    spellcheck="false"
                    @blur="handleRename"
                    @keydown.enter="handleRename"
                    @keydown.esc="cancelRename"
                    @click.stop
                />
                <span
                    v-if="!isRenaming && children.length > 0"
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
