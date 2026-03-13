<script lang="ts" setup>
withDefaults(defineProps<{ title?: string | null }>(), {
    title: "NativeDoctor",
});
</script>

<template>
    <div
        class="h-8 border-b border-border flex items-center pl-22"
        data-tauri-drag-region
    >
        <p
            v-if="title"
            class="text-sm text-gray-800 font-medium pointer-events-none"
        >
            {{ title }}
        </p>
        <slot />
    </div>
</template>
