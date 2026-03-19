<script setup lang="ts">
import { Textarea } from "@/components/ui/textarea";

defineProps<{
  modelValue: string;
  disabled?: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void;
}>();
</script>

<template>
  <Textarea
    :model-value="modelValue"
    :disabled="disabled"
    placeholder='<?xml version="1.0"?>'
    class="min-h-30 font-mono text-sm"
    @update:model-value="emit('update:modelValue', String($event ?? ''))"
  />
</template>
