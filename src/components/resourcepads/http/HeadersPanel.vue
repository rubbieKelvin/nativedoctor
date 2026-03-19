<script setup lang="ts">
import type { KeyValue } from "./types"
import { KeyValueTable } from "@/components/ui/kvtable"

const props = defineProps<{
  modelValue: KeyValue[]
}>()

const emit = defineEmits<{
  (e: "update:modelValue", value: KeyValue[]): void
}>()
</script>

<template>
  <KeyValueTable
    :model-value="props.modelValue"
    @update:model-value="emit('update:modelValue', $event)"
  />
</template>
