<script setup lang="ts">
import { ScrollArea } from "@/components/ui/scroll-area";

defineProps<{
    body?: string;
}>();
</script>

<template>
    <div class="flex-1 space-y-1">
        <p class="text-muted-foreground text-xs font-medium uppercase">
            Body
        </p>
        <p
            v-if="body == null || body === ''"
            class="text-muted-foreground text-sm"
        >
            No body
        </p>
        <ScrollArea
            v-else
            class="h-50 w-full rounded border border-border font-mono text-xs"
        >
            <pre class="whitespace-pre-wrap wrap-break-words p-2">{{ body }}</pre>
        </ScrollArea>
    </div>
</template>
