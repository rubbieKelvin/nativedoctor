<script lang="ts" setup>
withDefaults(defineProps<{ showTitle?: boolean }>(), { showTitle: true });
</script>

<template>
    <div
        class="h-8 border-b border-border flex items-center pl-22"
        data-tauri-drag-region
    >
        <p v-if="showTitle" class="text-sm text-gray-800 font-medium">
            NativeDoctor
        </p>
        <slot />
    </div>
</template>
