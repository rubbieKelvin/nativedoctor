<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
    headers: [string, string][];
}>();

/** Set-Cookie header lines derived from response headers. */
const cookieLines = computed(() => {
    if (!props.headers?.length) return [];
    return props.headers.filter(
        ([name]) => name.toLowerCase() === "set-cookie",
    );
});
</script>

<template>
    <div class="space-y-1">
        <p class="text-muted-foreground text-xs font-medium uppercase">
            Cookies
        </p>
        <p
            v-if="!cookieLines.length"
            class="text-muted-foreground text-sm"
        >
            No cookies
        </p>
        <ul v-else class="space-y-0.5 font-mono text-xs">
            <li
                v-for="([, val], i) in cookieLines"
                :key="i"
                class="flex gap-2 break-all"
            >
                <span>{{ val }}</span>
            </li>
        </ul>
    </div>
</template>
