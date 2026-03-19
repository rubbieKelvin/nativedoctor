<script setup lang="ts">
defineProps<{
  disabled?: boolean;
}>();
</script>

<template>
  <p class="text-muted-foreground text-sm">
    No body
    <span v-if="disabled"> (body is not sent with GET / HEAD requests)</span>
  </p>
</template>
