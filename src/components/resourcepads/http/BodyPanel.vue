<script setup lang="ts">
import type { HttpBody } from "@/shared/types/resources";
import { computed } from "vue";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import BodyNone from "./body/BodyNone.vue";
import BodyJson from "./body/BodyJson.vue";
import BodyOther from "./body/BodyOther.vue";
import BodyXml from "./body/BodyXml.vue";
import BodyGraphql from "./body/BodyGraphql.vue";
import BodyFormUrlencoded from "./body/BodyFormUrlencoded.vue";
import BodyFormData from "./body/BodyFormData.vue";
import BodyBinary from "./body/BodyBinary.vue";
import {
  BODY_TYPE_OPTIONS,
  defaultBodyForType,
} from "./body/body-helpers";

const props = defineProps<{
  modelValue: HttpBody;
  disabled?: boolean;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", value: HttpBody): void;
}>();

const body = computed(() => props.modelValue ?? ({ type: "none" } as HttpBody));

function onTypeChange(type: string) {
  const next = defaultBodyForType(type as HttpBody["type"]);
  emit("update:modelValue", next);
}

function bodyType(): string {
  return body.value.type;
}
</script>

<template>
  <div class="space-y-3">
    <div class="flex items-center gap-2">
      <span class="text-muted-foreground shrink-0 text-sm">Body type</span>
      <Select
        :model-value="bodyType()"
        @update:model-value="onTypeChange"
      >
        <SelectTrigger class="w-[180px]" size="sm">
          <SelectValue placeholder="Select type" />
        </SelectTrigger>
        <SelectContent>
          <SelectItem
            v-for="opt in BODY_TYPE_OPTIONS"
            :key="opt.value"
            :value="opt.value"
          >
            {{ opt.label }}
          </SelectItem>
        </SelectContent>
      </Select>
    </div>

    <template v-if="body.type === 'none'">
      <BodyNone :disabled="disabled" />
    </template>
    <template v-else-if="body.type === 'json'">
      <BodyJson
        :model-value="body.content"
        :disabled="disabled"
        @update:model-value="
          emit('update:modelValue', { type: 'json', content: $event })
        "
      />
    </template>
    <template v-else-if="body.type === 'text'">
      <BodyOther
        :model-value="body.content"
        :disabled="disabled"
        @update:model-value="
          emit('update:modelValue', { type: 'text', content: $event })
        "
      />
    </template>
    <template v-else-if="body.type === 'xml'">
      <BodyXml
        :model-value="body.content"
        :disabled="disabled"
        @update:model-value="
          emit('update:modelValue', { type: 'xml', content: $event })
        "
      />
    </template>
    <template v-else-if="body.type === 'other'">
      <BodyOther
        :model-value="body.content"
        :disabled="disabled"
        @update:model-value="
          emit('update:modelValue', { type: 'other', content: $event })
        "
      />
    </template>
    <template v-else-if="body.type === 'graphql'">
      <BodyGraphql
        :query="body.query"
        :variables="body.variables"
        :disabled="disabled"
        @update:query="
          emit('update:modelValue', {
            type: 'graphql',
            query: $event,
            variables: body.variables,
          })
        "
        @update:variables="
          emit('update:modelValue', {
            type: 'graphql',
            query: body.query,
            variables: $event,
          })
        "
      />
    </template>
    <template v-else-if="body.type === 'x-www-form-urlencoded'">
      <BodyFormUrlencoded
        :model-value="body.fields"
        :disabled="disabled"
        @update:model-value="
          emit('update:modelValue', {
            type: 'x-www-form-urlencoded',
            fields: $event,
          })
        "
      />
    </template>
    <template v-else-if="body.type === 'form-data'">
      <BodyFormData
        :model-value="body.fields"
        :disabled="disabled"
        @update:model-value="
          emit('update:modelValue', { type: 'form-data', fields: $event })
        "
      />
    </template>
    <template v-else-if="body.type === 'binary'">
      <BodyBinary
        :model-value="body.file_path ?? ''"
        :disabled="disabled"
        @update:model-value="
          emit('update:modelValue', {
            type: 'binary',
            file_path: $event || undefined,
          })
        "
      />
    </template>
    <template v-else>
      <BodyNone :disabled="disabled" />
    </template>
  </div>
</template>
