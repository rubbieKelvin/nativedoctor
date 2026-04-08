<script setup lang="ts">
import { computed } from "vue";
import type { TimelineRow } from "@/utils/streamTimeline";
import { ScrollArea } from "@/components/ui/scroll-area";

const props = defineProps<{
    row: TimelineRow | null;
}>();

const jsonText = computed(() => {
    const r = props.row;
    if (!r) return "";
    try {
        if (r.kind === "span") {
            return JSON.stringify(
                {
                    variant: r.variant,
                    label: r.label,
                    startMs: r.startMs,
                    endMs: r.endMs,
                    status: r.status,
                    start: r.rawStart,
                    end: r.rawEnd,
                },
                null,
                2,
            );
        }
        return JSON.stringify(
            { variant: r.variant, label: r.label, tMs: r.tMs, raw: r.raw },
            null,
            2,
        );
    } catch {
        return String(r);
    }
});
</script>

<template>
    <div
        v-if="row"
        class="border-border flex min-h-0 flex-col border-t bg-muted/15"
    >
        <div
            class="text-muted-foreground shrink-0 px-2 py-1.5 text-[10px] font-medium tracking-wide uppercase"
        >
            Event detail
        </div>
        <ScrollArea class="max-h-48 min-h-0">
            <pre
                class="text-muted-foreground font-mono p-2 text-[11px] leading-relaxed whitespace-pre-wrap break-all"
                >{{ jsonText }}</pre
            >
        </ScrollArea>
    </div>
</template>
