<script setup lang="ts">
import { computed, nextTick, ref, watch } from "vue";
import type { TimelineRow } from "@/utils/streamTimeline";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { X } from "lucide-vue-next";

const props = defineProps<{
    row: TimelineRow | null;
}>();

const emit = defineEmits<{
    close: [];
}>();

const rootRef = ref<HTMLElement | null>(null);

watch(
    () => props.row,
    (r) => {
        if (r) {
            void nextTick(() => {
                rootRef.value?.focus({ preventScroll: true });
            });
        }
    },
    { immediate: true },
);

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
        ref="rootRef"
        class="border-border flex h-full min-h-0 min-w-0 flex-col border-t bg-muted/10 outline-none"
        tabindex="-1"
        @keydown.escape.prevent="emit('close')"
    >
        <div
            class="border-border flex shrink-0 items-center justify-between gap-2 border-b px-2 py-1.5"
        >
            <div
                class="text-muted-foreground min-w-0 truncate text-xs font-semibold tracking-wide uppercase"
            >
                Event detail
            </div>
            <Button
                type="button"
                variant="ghost"
                size="icon-sm"
                class="shrink-0 text-muted-foreground hover:text-foreground"
                aria-label="Close event detail"
                @click="emit('close')"
            >
                <X class="size-4" />
            </Button>
        </div>
        <ScrollArea class="min-h-0 flex-1">
            <pre
                class="text-muted-foreground font-mono p-3 text-sm leading-relaxed whitespace-pre-wrap break-all"
                >{{ jsonText }}</pre
            >
        </ScrollArea>
    </div>
</template>
