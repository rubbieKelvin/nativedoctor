<script setup lang="ts">
import { computed, ref, watch, nextTick } from "vue";
import type { HttpResource } from "@/shared/types/resources";
import { Globe } from "lucide-vue-next";
import { Badge } from "@/components/ui/badge";
import { HttpContextMenu } from "./menus";
import { useCurrentProject, useCurrentProjectActions } from "@/store/project";

const props = withDefaults(
    defineProps<{
        resource: HttpResource;
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

const methodColor = computed(() => {
    switch (props.resource.method) {
        case "GET":
            return "text-green-600 bg-green-500/10";
        case "POST":
            return "text-blue-600 bg-blue-500/10";
        case "PUT":
            return "text-orange-600 bg-orange-500/10";
        case "PATCH":
            return "text-yellow-600 bg-yellow-500/10";
        case "DELETE":
            return "text-red-600 bg-red-500/10";
        default:
            return "text-muted-foreground bg-muted";
    }
});
</script>

<template>
    <HttpContextMenu :resource-id="resource.id">
        <button
            class="flex w-full items-center gap-2 rounded-md px-2 py-1.5 text-sm text-sidebar-foreground hover:bg-sidebar-accent hover:text-sidebar-accent-foreground"
            :style="{ paddingLeft: `${depth * 12 + 8}px` }"
            :disabled="isRenaming"
            @click="$emit('select', resource.id)"
        >
            <Globe class="size-4 shrink-0 text-muted-foreground" />
            <Badge
                variant="outline"
                :class="['px-1.5 py-0 text-[10px] font-semibold', methodColor]"
            >
                {{ resource.method }}
            </Badge>
            <span v-if="!isRenaming" class="truncate">{{
                resource.name || "Untitled request"
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
        </button>
    </HttpContextMenu>
</template>
