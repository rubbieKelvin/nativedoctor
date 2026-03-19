<script setup lang="ts">
import { ref, watch } from "vue";
import { Textarea } from "@/components/ui/textarea";

const props = defineProps<{
  modelValue: string;
  disabled?: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: string): void;
}>();

const error = ref<string | null>(null);

watch(
  () => props.modelValue,
  (val) => {
    if (!val?.trim()) {
      error.value = null;
      return;
    }
    try {
      JSON.parse(val);
      error.value = null;
    } catch {
      error.value = "Invalid JSON";
    }
  },
  { immediate: true },
);
</script>

<template>
  <div class="space-y-2">
    <Textarea
      :model-value="modelValue"
      :disabled="disabled"
      placeholder='{"key": "value"}'
      class="min-h-30 font-mono text-sm"
      @update:model-value="emit('update:modelValue', String($event ?? ''))"
    />
    <p v-if="error" class="text-destructive text-sm">{{ error }}</p>
  </div>
</template>
