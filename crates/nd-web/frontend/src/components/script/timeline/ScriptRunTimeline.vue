<script setup lang="ts">
import { computed, ref, watch } from "vue";
import type { TimelineReducerState, TimelineRow } from "@/utils/streamTimeline";
import { cn } from "@/lib/utils";
import { ScrollArea } from "@/components/ui/scroll-area";
import ScriptTimelineAxis from "./ScriptTimelineAxis.vue";
import ScriptTimelineEventDetail from "./ScriptTimelineEventDetail.vue";
import ScriptTimelineGrid from "./ScriptTimelineGrid.vue";
import ScriptTimelinePlayhead from "./ScriptTimelinePlayhead.vue";
import ScriptTimelineRow from "./ScriptTimelineRow.vue";
import { computeTimelineTicks } from "./timelineScale";

const LABEL_COL = "8.5rem";

const props = defineProps<{
    timeline: TimelineReducerState | undefined;
    sending: boolean;
    runError: string | null;
}>();

const selectedId = ref<string | null>(null);

watch(
    () => props.timeline?.rows.length,
    () => {
        if (
            selectedId.value &&
            !props.timeline?.rows.some((r) => r.id === selectedId.value)
        ) {
            selectedId.value = null;
        }
    },
);

const tMaxMs = computed(() => {
    const tl = props.timeline;
    let m = Math.max(tl?.lastElapsedMs ?? 0, 1);
    if (!tl?.rows.length) return m;
    for (const r of tl.rows) {
        if (r.kind === "instant") m = Math.max(m, r.tMs);
        else {
            m = Math.max(m, r.startMs);
            if (r.endMs != null) m = Math.max(m, r.endMs);
        }
    }
    return m;
});

const ticks = computed(() => computeTimelineTicks(tMaxMs.value));

const selectedRow = computed((): TimelineRow | null => {
    const id = selectedId.value;
    if (!id || !props.timeline) return null;
    return props.timeline.rows.find((r) => r.id === id) ?? null;
});

function onSelectRow(id: string) {
    selectedId.value = selectedId.value === id ? null : id;
}

const emptyHint = computed(() => {
    if (props.timeline?.rows.length) return null;
    if (props.sending) return "Waiting for events…";
    return "Run the script to populate the timeline.";
});

const showPlayhead = computed(
    () =>
        props.sending &&
        (props.timeline?.rows.length ?? 0) > 0 &&
        (props.timeline?.lastElapsedMs ?? 0) >= 0,
);
</script>

<template>
    <div class="flex min-h-0 min-w-0 flex-1 flex-col bg-background">
        <div
            v-if="runError"
            class="text-destructive shrink-0 border-b border-border px-2 py-1.5 text-xs"
        >
            {{ runError }}
        </div>
        <div
            v-if="emptyHint"
            class="text-muted-foreground flex shrink-0 items-center px-2 py-3 text-xs"
        >
            {{ emptyHint }}
        </div>
        <template v-if="timeline && timeline.rows.length > 0">
            <div
                class="border-border text-muted-foreground grid shrink-0 border-b bg-muted/20 font-mono text-[10px]"
                :style="{
                    gridTemplateColumns: `${LABEL_COL} minmax(12rem,1fr)`,
                }"
            >
                <div class="flex items-center px-2 py-1.5 font-sans">Event</div>
                <div
                    class="border-border flex items-center border-l px-2 py-1.5 font-sans"
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
                                :t-ms="timeline.lastElapsedMs"
                                :t-max-ms="tMaxMs"
                                :visible="showPlayhead"
                            />
                        </div>
                    </div>
                    <div
                        class="relative z-[1] grid min-w-0"
                        :style="{
                            gridTemplateColumns: `${LABEL_COL} minmax(12rem,1fr)`,
                        }"
                    >
                        <template
                            v-for="(r, idx) in timeline.rows"
                            :key="r.id"
                        >
                            <button
                                type="button"
                                :class="
                                    cn(
                                        'hover:bg-muted/35 flex min-w-0 flex-col justify-center border-b border-border/40 px-2 py-1.5 text-left transition-colors',
                                        idx % 2 === 1 && 'bg-muted/10',
                                        selectedId === r.id &&
                                            'bg-primary/8 ring-primary/25 ring-1 ring-inset',
                                    )
                                "
                                @click="onSelectRow(r.id)"
                            >
                                <span
                                    class="text-foreground text-[11px] font-semibold leading-tight"
                                    >{{ r.variant }}</span
                                >
                                <span
                                    class="text-muted-foreground truncate text-[10px] leading-tight"
                                    :title="r.label"
                                    >{{ r.label }}</span
                                >
                            </button>
                            <div
                                :class="
                                    cn(
                                        'border-border flex min-h-10 items-stretch border-b border-l border-border/40',
                                        idx % 2 === 1 && 'bg-muted/5',
                                        selectedId === r.id && 'bg-primary/5',
                                    )
                                "
                            >
                                <ScriptTimelineRow
                                    :row="r"
                                    :t-max-ms="tMaxMs"
                                    :sending="sending"
                                    :last-elapsed-ms="timeline.lastElapsedMs"
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
                <div class="border-border border-l border-t">
                    <ScriptTimelineAxis :ticks="ticks" />
                </div>
            </div>
        </template>
        <ScriptTimelineEventDetail :row="selectedRow" />
    </div>
</template>
