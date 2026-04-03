<script setup lang="ts">
import { Button } from "@/components/ui/button";
import type { EditorTab } from "@/types/editor";

defineProps<{
    tabs: EditorTab[];
    activeId: string | null;
}>();

const emit = defineEmits<{
    select: [id: string];
    close: [id: string, ev: MouseEvent];
}>();
</script>

<template>
    <div
        class="flex flex-wrap border-b border-border bg-background"
        role="tablist"
    >
        <Button
            v-for="t in tabs"
            :key="t.id"
            variant="ghost"
            size="sm"
            class="h-8 max-w-48 shrink-0 rounded-none border-r border-border px-2"
            :class="activeId === t.id ? 'bg-muted' : ''"
            @click="emit('select', t.id)"
        >
            <span class="truncate text-xs">{{ t.title }}</span>
            <span
                class="ml-1 text-muted-foreground hover:text-foreground"
                @click="emit('close', t.id, $event)"
                >×</span
            >
        </Button>
    </div>
</template>
