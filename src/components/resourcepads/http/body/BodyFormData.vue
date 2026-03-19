<script setup lang="ts">
import type { KeyValuePair } from "@/shared/types/resources";
import { KeyValueTable } from "@/components/ui/kvtable";
import { computed } from "vue";

type FormDataField = KeyValuePair & { isFile?: boolean };

const props = defineProps<{
  modelValue: FormDataField[];
  disabled?: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: FormDataField[]): void;
}>();

const fieldsAsKv = computed({
  get: () =>
    props.modelValue.map(({ isFile: _f, ...rest }) => ({ ...rest })),
  set: (val: KeyValuePair[]) => {
    const current = props.modelValue;
    const next: FormDataField[] = val.map((kv, i) => ({
      ...kv,
      isFile: current[i]?.isFile ?? false,
    }));
    emit("update:modelValue", next);
  },
});
</script>

<template>
  <div class="space-y-2">
    <KeyValueTable
      :model-value="fieldsAsKv"
      @update:model-value="fieldsAsKv = $event"
    />
    <p class="text-muted-foreground text-xs">
      Multipart form-data (file upload per row: backend support pending).
    </p>
  </div>
</template>
