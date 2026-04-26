<script setup lang="ts">
import { computed, onBeforeUnmount, ref, watch } from "vue";
import type { TimelineReducerState, TimelineRow } from "@/utils/streamTimeline";
import { useExecutionStore } from "@/stores/execution";
import {
    ResizableHandle,
    ResizablePanel,
    ResizablePanelGroup,
} from "@/components/ui/resizable";
import ScriptRunTimelineTrack from "./ScriptRunTimelineTrack.vue";
import ScriptTimelineEventDetail from "./ScriptTimelineEventDetail.vue";
import { computeTimelineTicks } from "./timelineScale";

const props = defineProps<{
    timeline: TimelineReducerState | undefined;
    sending: boolean;
    runError: string | null;
    scriptPath: string | undefined;
}>();

const execution = useExecutionStore();

const selectedId = ref<string | null>(null);

/** Bumps every animation frame while `sending` so `liveElapsedMs` recomputes. */
const rafTick = ref(0);
let rafId = 0;

function rafLoop() {
    rafTick.value++;
    rafId = requestAnimationFrame(rafLoop);
}

watch(
    () => props.sending,
    (sending) => {
        cancelAnimationFrame(rafId);
        if (sending) {
            rafId = requestAnimationFrame(rafLoop);
        }
    },
    { immediate: true },
);

onBeforeUnmount(() => {
    cancelAnimationFrame(rafId);
});

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

const liveElapsedMs = computed(() => {
    rafTick.value;
    const tl = props.timeline;
    if (!tl) return 0;
    if (!props.sending) return tl.lastElapsedMs;
    const path = props.scriptPath;
    const sync = path
        ? execution.scriptTimelineWallSyncByPath[path]
        : undefined;
    const wall = sync?.lastWallMs ?? performance.now();
    return tl.lastElapsedMs + (performance.now() - wall);
});

const structuralTMaxMs = computed(() => {
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

const tMaxMs = computed(() => {
    if (!props.sending) return structuralTMaxMs.value;
    return Math.max(structuralTMaxMs.value, liveElapsedMs.value, 1);
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

function closeDetail() {
    selectedId.value = null;
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
        liveElapsedMs.value >= 0,
);
</script>

<template>
    <div class="flex min-h-0 min-w-0 flex-1 flex-col bg-background">
        <div
            v-if="runError"
            class="text-destructive shrink-0 border-b border-border px-3 py-2 text-sm"
        >
            {{ runError }}
        </div>
        <div
            v-if="emptyHint"
            class="text-muted-foreground flex shrink-0 items-center px-3 py-4 text-sm"
        >
            {{ emptyHint }}
        </div>
        <template v-if="timeline && timeline.rows.length > 0">
            <ResizablePanelGroup
                v-if="selectedRow"
                direction="vertical"
                auto-save-id="nd-script-timeline-detail"
                class="flex min-h-0 min-w-0 flex-1 flex-col"
            >
                <ResizablePanel
                    :default-size="68"
                    :min-size="28"
                    class="flex min-h-0 min-w-0 flex-col"
                >
                    <ScriptRunTimelineTrack
                        :timeline="timeline"
                        :ticks="ticks"
                        :t-max-ms="tMaxMs"
                        :sending="sending"
                        :live-elapsed-ms="liveElapsedMs"
                        :show-playhead="showPlayhead"
                        :selected-id="selectedId"
                        @select-row="onSelectRow"
                    />
                </ResizablePanel>
                <ResizableHandle with-handle />
                <ResizablePanel
                    :default-size="32"
                    :min-size="16"
                    class="flex min-h-0 min-w-0 flex-col"
                >
                    <ScriptTimelineEventDetail
                        :row="selectedRow"
                        @close="closeDetail"
                    />
                </ResizablePanel>
            </ResizablePanelGroup>
            <ScriptRunTimelineTrack
                v-else
                :timeline="timeline"
                :ticks="ticks"
                :t-max-ms="tMaxMs"
                :sending="sending"
                :live-elapsed-ms="liveElapsedMs"
                :show-playhead="showPlayhead"
                :selected-id="selectedId"
                @select-row="onSelectRow"
            />
        </template>
    </div>
</template>
