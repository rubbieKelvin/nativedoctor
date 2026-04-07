<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { useEditorStore } from "@/stores/editor";
import { storeToRefs } from "pinia";
import { X } from "lucide-vue-next";

const editor = useEditorStore();
const { tabs, activeId } = storeToRefs(editor);
</script>

<template>
    <div
        class="flex min-h-0 flex-wrap border-b border-border bg-background"
        role="tablist"
    >
        <Button
            v-for="t in tabs"
            :key="t.id"
            variant="ghost"
            size="sm"
            class="h-8 max-w-52 shrink-0 gap-1 rounded-none border-r border-border px-2"
            :class="
                activeId === t.id
                    ? 'bg-muted text-foreground'
                    : 'text-muted-foreground'
            "
            @click="editor.activeId = t.id"
        >
            <span class="min-w-0 truncate text-xs font-medium">{{
                t.title
            }}</span>
            <span
                class="inline-flex shrink-0 rounded-sm p-0.5 hover:bg-background/80 hover:text-foreground"
                title="Close tab"
                @click.stop="editor.closeTab(t.id, $event)"
            >
                <X class="h-3 w-3" aria-hidden="true" />
            </span>
        </Button>
    </div>
</template>
