<script setup lang="ts">
import { Input } from "@/components/ui/input";

defineProps<{
  modelValue: string;
  disabled?: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void;
}>();
</script>

<template>
  <div class="space-y-2">
    <Input
      :model-value="modelValue"
      :disabled="disabled"
      placeholder="Path to file"
      class="font-mono text-sm"
      @update:model-value="emit('update:modelValue', String($event ?? ''))"
    />
    <p class="text-muted-foreground text-xs">
      Binary body: select a file (backend support pending).
    </p>
  </div>
</template>
