<script setup lang="ts">
import { Textarea } from "@/components/ui/textarea";

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
        <Textarea
            :model-value="modelValue"
            :disabled="disabled"
            placeholder='{"key": "value"}'
            class="min-h-30 font-mono text-sm"
            @update:model-value="
                (v) => emit('update:modelValue', String(v ?? ''))
            "
        />
        <p v-if="disabled" class="text-muted-foreground text-sm">
            Body is not sent with GET / HEAD requests.
        </p>
    </div>
</template>
