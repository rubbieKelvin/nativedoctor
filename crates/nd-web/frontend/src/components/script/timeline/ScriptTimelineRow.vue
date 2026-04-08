<script setup lang="ts">
import { computed } from "vue";
import type { TimelineRow } from "@/utils/streamTimeline";

const props = defineProps<{
    row: TimelineRow;
    tMaxMs: number;
    sending: boolean;
    lastElapsedMs: number;
}>();

const emit = defineEmits<{
    select: [id: string];
}>();

const tMax = computed(() => Math.max(props.tMaxMs, 1));

const startPct = computed(() => {
    if (props.row.kind === "instant") {
        return (props.row.tMs / tMax.value) * 100;
    }
    return (props.row.startMs / tMax.value) * 100;
});

const spanEndMs = computed(() => {
    if (props.row.kind !== "span") return 0;
    if (props.row.endMs != null) return props.row.endMs;
    if (props.sending) return tMax.value;
    return Math.max(props.row.startMs, props.lastElapsedMs);
});

const endPct = computed(() => {
    if (props.row.kind === "instant") return startPct.value;
    return (spanEndMs.value / tMax.value) * 100;
});

const barWidthPct = computed(() =>
    Math.max(0.5, endPct.value - startPct.value),
);

const barClass = computed(() => {
    if (props.row.kind !== "span") return "";
    switch (props.row.status) {
        case "ok":
            return "bg-emerald-600/90 shadow-sm dark:bg-emerald-500/85";
        case "error":
            return "bg-destructive/95 shadow-sm";
        case "running":
            return "animate-pulse bg-primary/85 shadow-sm";
        case "interrupted":
            return "bg-amber-600/80 dark:bg-amber-500/75";
        default:
            return "bg-muted-foreground/45";
    }
});

const markerClass = computed(() => {
    if (props.row.kind === "span") {
        if (props.row.status === "error")
            return "border-destructive bg-destructive ring-background ring-2";
        if (props.row.endMs == null && props.sending)
            return "border-primary bg-primary ring-background ring-2";
        return "border-emerald-700 bg-emerald-500 ring-background ring-2 dark:border-emerald-300 dark:bg-emerald-400";
    }
    return "border-muted-foreground/70 bg-background ring-border ring-2";
});
</script>

<template>
    <div
        class="relative h-10 w-full min-w-[12rem] cursor-pointer"
        role="button"
        tabindex="0"
        @click="emit('select', row.id)"
        @keydown.enter.prevent="emit('select', row.id)"
        @keydown.space.prevent="emit('select', row.id)"
    >
        <template v-if="row.kind === 'instant'">
            <div
                class="bg-primary/35 pointer-events-none absolute top-0.5 bottom-0.5 w-px -translate-x-1/2"
                :style="{ left: `${startPct}%` }"
            />
            <div
                class="border-border bg-background hover:bg-muted/60 absolute top-1/2 z-[2] size-3 -translate-x-1/2 -translate-y-1/2 rounded-full border-2 transition-colors"
                :class="markerClass"
                :style="{ left: `${startPct}%` }"
            />
        </template>
        <template v-else>
            <div
                class="pointer-events-none absolute top-1/2 z-[1] size-2.5 -translate-x-1/2 -translate-y-1/2 rounded-full border-2"
                :class="markerClass"
                :style="{ left: `${startPct}%` }"
            />
            <div
                class="pointer-events-none absolute top-1/2 z-0 h-3 min-w-1 -translate-y-1/2 rounded-full"
                :class="barClass"
                :style="{
                    left: `${startPct}%`,
                    width: `${barWidthPct}%`,
                }"
            />
            <div
                v-if="row.endMs != null || !sending"
                class="pointer-events-none absolute top-1/2 z-[1] size-2.5 -translate-x-1/2 -translate-y-1/2 rounded-full border-2"
                :class="markerClass"
                :style="{ left: `${endPct}%` }"
            />
            <span
                v-if="barWidthPct > 14"
                class="text-background pointer-events-none absolute top-1/2 z-[2] max-w-[min(42%,10rem)] -translate-y-1/2 truncate px-2 text-[10px] font-medium leading-none drop-shadow-sm"
                :style="{ left: `calc(${startPct}% + 8px)` }"
                >{{ row.label }}</span
            >
        </template>
    </div>
</template>
