<script setup lang="ts">
import type { SequenceResource } from "@/shared/types/resources";

defineProps<{
    resource: SequenceResource;
}>();
</script>

<template>
    <div class="flex h-full items-center justify-center p-8 text-muted-foreground">
        <p>Sequence editor for "{{ resource.name || 'Untitled sequence' }}" — coming soon.</p>
    </div>
</template>
