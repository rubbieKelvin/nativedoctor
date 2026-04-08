<script setup lang="ts">
import type { TimelineTick } from "./timelineScale";

defineProps<{
    ticks: TimelineTick[];
}>();
</script>

<template>
    <div class="pointer-events-none absolute inset-0 overflow-hidden">
        <div
            v-for="(tk, i) in ticks"
            :key="i"
            class="bg-border/25 absolute top-0 bottom-0 w-px"
            :style="{ left: `${tk.pct}%` }"
        />
    </div>
</template>
