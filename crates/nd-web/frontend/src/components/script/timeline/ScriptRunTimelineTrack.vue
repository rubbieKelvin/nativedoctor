<script setup lang="ts">
import type { TimelineReducerState } from "@/utils/streamTimeline";
import { cn } from "@/lib/utils";
import { ScrollArea } from "@/components/ui/scroll-area";
import ScriptTimelineAxis from "./ScriptTimelineAxis.vue";
import ScriptTimelineGrid from "./ScriptTimelineGrid.vue";
import ScriptTimelinePlayhead from "./ScriptTimelinePlayhead.vue";
import ScriptTimelineRow from "./ScriptTimelineRow.vue";
import type { TimelineTick } from "./timelineScale";

const LABEL_COL = "8.5rem";

defineProps<{
    timeline: TimelineReducerState;
    ticks: TimelineTick[];
    tMaxMs: number;
    sending: boolean;
    liveElapsedMs: number;
    showPlayhead: boolean;
    selectedId: string | null;
}>();

const emit = defineEmits<{
    selectRow: [id: string];
}>();

function onSelectRow(id: string) {
    emit("selectRow", id);
}
</script>

<template>
    <div class="flex min-h-0 min-w-0 flex-1 flex-col">
        <div
            class="border-border text-muted-foreground bg-muted/15 grid shrink-0 border-b text-xs font-medium tabular-nums"
            :style="{
                gridTemplateColumns: `${LABEL_COL} minmax(12rem,1fr)`,
            }"
        >
            <div class="flex items-center px-2 py-1 font-sans">Event</div>
            <div
                class="border-border flex items-center border-l px-2 py-1 font-sans"
            >
                Time
                <span class="text-muted-foreground/60 ml-1 font-normal"
                    >(session)</span
                >
            </div>
        </div>
        <ScrollArea class="min-h-0 flex-1">
            <div class="relative min-w-0">
                <div
                    class="pointer-events-none absolute top-0 bottom-0 z-0 border-border/60 border-l"
                    :style="{
                        left: LABEL_COL,
                        right: '0',
                    }"
                >
                    <div class="relative h-full w-full">
                        <ScriptTimelineGrid :ticks="ticks" />
                        <ScriptTimelinePlayhead
                            :t-ms="liveElapsedMs"
                            :t-max-ms="tMaxMs"
                            :visible="showPlayhead"
                        />
                    </div>
                </div>
                <div
                    class="relative z-1 grid min-w-0"
                    :style="{
                        gridTemplateColumns: `${LABEL_COL} minmax(12rem,1fr)`,
                    }"
                >
                    <template v-for="(r, idx) in timeline.rows" :key="r.id">
                        <button
                            type="button"
                            :class="
                                cn(
                                    'hover:bg-muted/35 flex min-h-7 min-w-0 items-center border-b border-border/40 px-2 py-1 text-left transition-colors duration-150 ease-out',
                                    idx % 2 === 1 && 'bg-muted/10',
                                    selectedId === r.id &&
                                        'bg-accent/40 ring-ring/30 ring-1 ring-inset',
                                )
                            "
                            @click="onSelectRow(r.id)"
                        >
                            <span
                                class="text-foreground min-w-0 truncate text-xs leading-tight font-medium whitespace-nowrap"
                                :title="
                                    r.variant === 'Log' ||
                                    r.variant === 'AssertCalled'
                                        ? r.variant
                                        : `${r.variant} · ${r.label}`
                                "
                            >
                                <span>{{ r.variant }}</span>
                                <template
                                    v-if="
                                        r.variant !== 'Log' &&
                                        r.variant !== 'AssertCalled' &&
                                        r.label
                                    "
                                >
                                    <span
                                        class="text-muted-foreground font-normal"
                                    >
                                        ·
                                    </span>
                                    <span
                                        class="text-muted-foreground font-normal"
                                        >{{ r.label }}</span
                                    >
                                </template>
                            </span>
                        </button>
                        <div
                            :class="
                                cn(
                                    'flex min-h-7 items-stretch border-b border-l border-border/40',
                                    idx % 2 === 1 && 'bg-muted/5',
                                    selectedId === r.id && 'bg-accent/25',
                                )
                            "
                        >
                            <ScriptTimelineRow
                                :row="r"
                                :t-max-ms="tMaxMs"
                                :sending="sending"
                                :live-elapsed-ms="liveElapsedMs"
                                @select="onSelectRow"
                            />
                        </div>
                    </template>
                </div>
            </div>
        </ScrollArea>
        <div
            class="grid shrink-0"
            :style="{
                gridTemplateColumns: `${LABEL_COL} minmax(12rem,1fr)`,
            }"
        >
            <div class="border-border bg-muted/15 border-t" />
            <div class="border-border border-l">
                <ScriptTimelineAxis :ticks="ticks" />
            </div>
        </div>
    </div>
</template>
