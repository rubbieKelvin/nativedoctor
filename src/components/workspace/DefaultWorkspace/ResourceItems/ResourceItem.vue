<script setup lang="ts">
import type { Resource } from "@/shared/types/resources";
import FolderItem from "./FolderItem.vue";
import HttpItem from "./HttpItem.vue";
import SequenceItem from "./SequenceItem.vue";

defineProps<{
    resource: Resource;
    depth?: number;
}>();

defineEmits<{
    (e: "select", id: string): void;
}>();
</script>

<template>
    <FolderItem
        v-if="resource.type === 'folder'"
        :resource="resource"
        :depth="depth ?? 0"
        @select="$emit('select', $event)"
    />
    <HttpItem
        v-else-if="resource.type === 'http'"
        :resource="resource"
        :depth="depth ?? 0"
        @select="$emit('select', $event)"
    />
    <SequenceItem
        v-else-if="resource.type === 'sequence'"
        :resource="resource"
        :depth="depth ?? 0"
        @select="$emit('select', $event)"
    />
</template>
