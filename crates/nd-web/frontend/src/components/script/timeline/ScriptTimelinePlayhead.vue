<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
    tMs: number;
    tMaxMs: number;
    visible: boolean;
}>();

const leftPct = computed(() => {
    const m = Math.max(props.tMaxMs, 1);
    return Math.min(100, Math.max(0, (props.tMs / m) * 100));
});
</script>

<template>
    <div
        v-if="visible"
        class="bg-foreground/55 pointer-events-none absolute inset-y-0 z-[5] w-px opacity-90 dark:bg-foreground/45"
        :style="{ left: `${leftPct}%` }"
    />
</template>
