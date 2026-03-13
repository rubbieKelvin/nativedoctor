<script setup lang="ts">
import { computed, ref, watch, nextTick } from "vue";
import type { SequenceResource } from "@/shared/types/resources";
import { ListOrdered } from "lucide-vue-next";
import { SequenceContextMenu } from "./menus";
import { useCurrentProject, useCurrentProjectActions } from "@/store/project";

const props = withDefaults(
    defineProps<{
        resource: SequenceResource;
        depth?: number;
    }>(),
    { depth: 0 }
);

defineEmits<{
    (e: "select", id: string): void;
}>();

const { renamingResourceId } = useCurrentProject();
const store = useCurrentProjectActions();

const isRenaming = computed(() => renamingResourceId.value === props.resource.id);
const tempName = ref(props.resource.name);
const inputRef = ref<HTMLInputElement | null>(null);

watch(isRenaming, (val) => {
    if (val) {
        tempName.value = props.resource.name;
        nextTick(() => {
            inputRef.value?.focus();
            inputRef.value?.select();
        });
    }
});

function handleRename() {
    if (!isRenaming.value) return;
    if (tempName.value.trim() && tempName.value !== props.resource.name) {
        store.renameResource(props.resource.id, tempName.value.trim());
    }
    store.stopRenaming();
}

function cancelRename() {
    store.stopRenaming();
}
</script>

<template>
    <SequenceContextMenu :resource-id="resource.id">
        <button
            class="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-sm text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
            :style="{ paddingLeft: `${depth * 12 + 8}px` }"
            :disabled="isRenaming"
            @click="$emit('select', resource.id)"
        >
            <ListOrdered class="size-4 shrink-0 text-purple-500" />
            <span v-if="!isRenaming" class="truncate">{{
                resource.name || "Untitled sequence"
            }}</span>
            <input
                v-else
                ref="inputRef"
                v-model="tempName"
                class="h-5 w-full min-w-0 flex-1 rounded-sm border-none bg-sidebar-accent-foreground/10 px-1 text-sm outline-none ring-1 ring-ring"
                @blur="handleRename"
                @keydown.enter="handleRename"
                @keydown.esc="cancelRename"
                @click.stop
            />
            <span
                v-if="!isRenaming && resource.flow.length > 0"
                class="ml-auto text-xs text-muted-foreground"
            >
                {{ resource.flow.length }} step{{
                    resource.flow.length === 1 ? "" : "s"
                }}
            </span>
        </button>
    </SequenceContextMenu>
</template>
