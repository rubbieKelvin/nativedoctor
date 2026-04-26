<script setup lang="ts">
import type { TimelineTick } from "./timelineScale";

defineProps<{
    ticks: TimelineTick[];
}>();
</script>

<template>
    <div
        class="border-border text-muted-foreground relative h-8 shrink-0 border-t bg-background text-xs tabular-nums"
    >
        <div class="absolute inset-x-0 top-0 pt-1.5">
            <template v-for="(tk, i) in ticks" :key="i">
                <div
                    class="absolute flex -translate-x-1/2 flex-col items-center"
                    :style="{ left: `${tk.pct}%` }"
                >
                    <span class="whitespace-nowrap">{{ tk.label }}</span>
                </div>
            </template>
        </div>
    </div>
</template>
