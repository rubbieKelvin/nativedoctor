<script setup lang="ts">
import { ref, computed } from "vue";
import type { KeyValue } from "./types";
import { KeyValueTable } from "@/components/ui/kvtable";
import { ChevronRight, ChevronDown } from "lucide-vue-next";

const props = withDefaults(
  defineProps<{
    modelValue: KeyValue[];
    computedHeaders?: Record<string, string>;
  }>(),
  { computedHeaders: () => ({}) },
);

const emit = defineEmits<{
  (e: "update:modelValue", value: KeyValue[]): void;
}>();

const computedHeadersOpen = ref(false);

const computedEntries = computed(() =>
  Object.entries(props.computedHeaders).filter(([k]) => k.trim()),
);
</script>

<template>
  <div class="space-y-3">
    <KeyValueTable
      :model-value="props.modelValue"
      @update:model-value="emit('update:modelValue', $event)"
    />
    <div v-if="computedEntries.length > 0" class="rounded-md border border-border">
      <button
        type="button"
        class="flex w-full items-center gap-1.5 px-3 py-2 text-left text-sm font-medium text-muted-foreground hover:bg-muted/50 hover:text-foreground"
        @click="computedHeadersOpen = !computedHeadersOpen"
      >
        <component
          :is="computedHeadersOpen ? ChevronDown : ChevronRight"
          class="size-4 shrink-0"
        />
        Computed headers
      </button>
      <div v-show="computedHeadersOpen" class="border-t border-border">
        <table class="w-full text-sm">
          <tbody>
            <tr
              v-for="[key, value] in computedEntries"
              :key="key"
              class="border-b border-border last:border-b-0"
            >
              <td
                class="w-[40%] border-r border-border bg-muted/30 px-3 py-2 font-medium"
              >
                {{ key }}
              </td>
              <td class="px-3 py-2 text-muted-foreground">{{ value }}</td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>
