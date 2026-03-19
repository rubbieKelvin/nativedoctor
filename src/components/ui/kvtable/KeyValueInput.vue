<script setup lang="ts">
import { computed, watch } from "vue";
import { Input } from "@/components/ui/input";
import type { KeyValuePair } from "./types";

const modelValue = defineModel<KeyValuePair>({ required: true });

const isKeyEmpty = computed(() => !modelValue.value.key?.trim());

// Keep the placeholder row "empty" (no value/description) when key is cleared.
watch(
    () => modelValue.value.key,
    (key) => {
        if (!key?.trim()) {
            modelValue.value.value = "";
            modelValue.value.description = "";
        }
    },
);
</script>

<template>
    <div class="flex items-center gap-2">
        <input
            type="checkbox"
            v-model="modelValue.enabled"
            :disabled="isKeyEmpty"
            aria-label="Enabled"
            class="h-4 w-4 shrink-0 rounded border-input"
        />

        <Input
            :model-value="modelValue.key"
            placeholder="Key"
            class="flex-1"
            @update:model-value="(v) => (modelValue.key = String(v))"
        />

        <div
            :class="isKeyEmpty ? 'pointer-events-none opacity-50' : ''"
            class="flex-1"
        >
            <Input
                :model-value="modelValue.value"
                placeholder="Value"
                class="w-full"
                @update:model-value="(v) => (modelValue.value = String(v))"
            />
        </div>

        <div
            :class="isKeyEmpty ? 'pointer-events-none opacity-50' : ''"
            class="flex-1"
        >
            <Input
                :model-value="modelValue.description ?? ''"
                placeholder="Description"
                class="w-full"
                @update:model-value="
                    (v) => (modelValue.description = String(v))
                "
            />
        </div>
    </div>
</template>
