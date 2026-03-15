<script setup lang="ts">
defineProps<{
    headers: [string, string][];
}>();
</script>

<template>
    <div class="space-y-1">
        <p class="text-muted-foreground text-xs font-medium uppercase">
            Headers
        </p>
        <p
            v-if="!headers?.length"
            class="text-muted-foreground text-sm"
        >
            No headers
        </p>
        <ul v-else class="space-y-0.5 font-mono text-xs">
            <li
                v-for="([name, val], i) in headers"
                :key="i"
                class="flex gap-2 break-all"
            >
                <span class="shrink-0 text-muted-foreground">{{ name }}:</span>
                <span>{{ val }}</span>
            </li>
        </ul>
    </div>
</template>
