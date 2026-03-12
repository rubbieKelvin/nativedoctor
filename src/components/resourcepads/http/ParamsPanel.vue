<script setup lang="ts">
import type { KeyValue } from "./types"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Plus, Trash2 } from "lucide-vue-next"

const props = withDefaults(
  defineProps<{
    modelValue: KeyValue[]
  }>(),
  { modelValue: () => [{ key: "", value: "" }] },
)

const emit = defineEmits<{
  (e: "update:modelValue", value: KeyValue[]): void
}>()

function addRow() {
  emit("update:modelValue", [...props.modelValue, { key: "", value: "" }])
}

function removeRow(index: number) {
  const list = props.modelValue.slice()
  list.splice(index, 1)
  emit("update:modelValue", list.length ? list : [{ key: "", value: "" }])
}

function updateRow(index: number, field: "key" | "value", val: string) {
  const list = props.modelValue.slice()
  if (!list[index]) return
  list[index] = { ...list[index], [field]: val }
  emit("update:modelValue", list)
}
</script>

<template>
  <div class="space-y-2">
    <div
      v-for="(item, index) in modelValue"
      :key="index"
      class="flex items-center gap-2"
    >
      <Input
        :model-value="item?.key"
        placeholder="Key"
        class="flex-1"
        @update:model-value="(v) => updateRow(index, 'key', String(v))"
      />
      <Input
        :model-value="item?.value"
        placeholder="Value"
        class="flex-1"
        @update:model-value="(v) => updateRow(index, 'value', String(v))"
      />
      <Button
        type="button"
        variant="ghost"
        size="icon"
        :aria-label="'Remove row ' + (index + 1)"
        @click="removeRow(index)"
      >
        <Trash2 class="size-4" />
      </Button>
    </div>
    <Button
      type="button"
      variant="outline"
      size="sm"
      @click="addRow"
    >
      <Plus class="size-4 mr-1" />
      Add row
    </Button>
  </div>
</template>
