<script setup lang="ts">
import { ref, watch } from "vue";
import type { KeyValuePair } from "./types";
import KeyValueInput from "./KeyValueInput.vue";

const modelValue = defineModel<KeyValuePair[]>({ required: true });

const rows = ref<KeyValuePair[]>([]);
const isSyncing = ref(false);

function emptyRow(): KeyValuePair {
    return { key: "", value: "", enabled: false, description: "" };
}

function normalizeRows(inRows: KeyValuePair[] | undefined): KeyValuePair[] {
    const committed = (inRows ?? []).filter(
        (r) => String(r.key ?? "").trim() !== "",
    );
    return [...committed, emptyRow()];
}

// Build initial UI state from v-model, and keep it in sync when the parent replaces the list.
watch(
    modelValue,
    (val) => {
        if (isSyncing.value) return;
        isSyncing.value = true;
        rows.value = normalizeRows(val);
        isSyncing.value = false;
    },
    { immediate: true, deep: true },
);

// Normalize UI state back into v-model: v-model contains ONLY non-empty-key entries.
watch(
    rows,
    (val) => {
        if (isSyncing.value) return;
        isSyncing.value = true;

        const normalized = normalizeRows(val);
        rows.value = normalized;

        modelValue.value = normalized.filter(
            (r) => String(r.key ?? "").trim() !== "",
        );

        isSyncing.value = false;
    },
    { deep: true },
);
</script>

<template>
    <div class="space-y-2">
        <KeyValueInput
            v-for="(_, index) in rows"
            :key="index"
            v-model="rows[index]"
        />
    </div>
</template>
