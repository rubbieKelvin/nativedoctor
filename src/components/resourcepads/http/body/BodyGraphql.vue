<script setup lang="ts">
import { Textarea } from "@/components/ui/textarea";

defineProps<{
  query: string;
  variables: string;
  disabled?: boolean;
}>();

const emit = defineEmits<{
  (e: "update:query", value: string): void;
  (e: "update:variables", value: string): void;
}>();
</script>

<template>
  <div class="space-y-3">
    <div>
      <label class="text-muted-foreground mb-1 block text-xs font-medium">
        Query
      </label>
      <Textarea
        :model-value="query"
        :disabled="disabled"
        placeholder="query { ... }"
        class="min-h-24 font-mono text-sm"
        @update:model-value="emit('update:query', String($event ?? ''))"
      />
    </div>
    <div>
      <label class="text-muted-foreground mb-1 block text-xs font-medium">
        Variables (JSON)
      </label>
      <Textarea
        :model-value="variables"
        :disabled="disabled"
        placeholder='{"key": "value"}'
        class="min-h-20 font-mono text-sm"
        @update:model-value="emit('update:variables', String($event ?? ''))"
      />
    </div>
  </div>
</template>
