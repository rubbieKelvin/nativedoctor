<script setup lang="ts">
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select"
import { cn } from "@/lib/utils"

const HTTP_METHODS = [
  "GET",
  "POST",
  "PUT",
  "PATCH",
  "DELETE",
  "HEAD",
  "OPTIONS",
] as const

defineProps<{
  modelValue: string
  method: string
  loading?: boolean
}>()

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void
  (e: "update:method", value: string): void
  (e: "send"): void
}>()

function onSend() {
  emit("send")
}
</script>

<template>
  <div class="flex flex-wrap items-center gap-2">
    <Select :model-value="method" @update:model-value="(v) => emit('update:method', String(v ?? 'GET'))">
      <SelectTrigger :class="cn('w-[120px]')">
        <SelectValue placeholder="Method" />
      </SelectTrigger>
      <SelectContent>
        <SelectItem
          v-for="m in HTTP_METHODS"
          :key="m"
          :value="m"
        >
          {{ m }}
        </SelectItem>
      </SelectContent>
    </Select>
    <Input
      :model-value="modelValue"
      type="url"
      placeholder="https://api.example.com/..."
      class="min-w-[200px] flex-1"
      @update:model-value="(v) => emit('update:modelValue', String(v ?? ''))"
    />
    <Button
      :disabled="loading || !modelValue?.trim()"
      @click="onSend"
    >
      {{ loading ? "Sending…" : "Send" }}
    </Button>
  </div>
</template>
