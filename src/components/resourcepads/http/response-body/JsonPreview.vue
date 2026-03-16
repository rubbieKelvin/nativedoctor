<script setup lang="ts">
import { computed } from "vue";
import { ScrollArea } from "@/components/ui/scroll-area";

const props = defineProps<{
    body: string;
}>();

const parsed = computed(() => {
    const s = props.body?.trim();
    if (!s) return { ok: true, value: null, raw: "" };
    try {
        const value = JSON.parse(s);
        return {
            ok: true,
            value,
            raw: JSON.stringify(value, null, 2),
        };
    } catch {
        return { ok: false, value: null, raw: props.body };
    }
});
</script>

<template>
    <div class="flex-1 space-y-1">
        <p
            v-if="parsed.ok && parsed.raw === ''"
            class="text-muted-foreground text-sm"
        >
            Empty JSON
        </p>
        <p
            v-else-if="!parsed.ok"
            class="text-destructive text-sm"
        >
            Invalid JSON
        </p>
        <ScrollArea
            v-else
            class="h-50 w-full rounded border border-border font-mono text-xs"
        >
            <pre class="whitespace-pre-wrap break-words p-2">{{ parsed.raw }}</pre>
        </ScrollArea>
    </div>
</template>
