<script setup lang="ts">
import type { KeyValuePair } from "@/shared/types/resources";
import { KeyValueTable } from "@/components/ui/kvtable";

defineProps<{
  modelValue: KeyValuePair[];
  disabled?: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: KeyValuePair[]): void;
}>();
</script>

<template>
  <KeyValueTable
    :model-value="modelValue"
    @update:model-value="emit('update:modelValue', $event)"
  />
</template>
