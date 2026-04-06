<script setup lang="ts">
import { nextTick, ref, watch } from "vue";
import { Input } from "@/components/ui/input";
import { cn } from "@/lib/utils";

type Row = { key: string; value: string };

const props = withDefaults(
    defineProps<{
        modelValue: Record<string, string>;
        disabled?: boolean;
        class?: string;
    }>(),
    { disabled: false },
);

const emit = defineEmits<{
    "update:modelValue": [value: Record<string, string>];
}>();

/** Build rows from a record: entries in insertion order, then one trailing empty row. */
function recordToRows(rec: Record<string, string>): Row[] {
    const rows: Row[] = Object.entries(rec).map(([key, value]) => ({
        key,
        value: String(value ?? ""),
    }));
    rows.push({ key: "", value: "" });
    return rows;
}

/**
 * Enforce: at least one row; exactly one trailing row with empty key; drop empty-key rows
 * that are not the only row and not last.
 */
function finalizeRows(rows: Row[]): Row[] {
    const trimmed = rows.map((r) => ({
        key: r.key.trim(),
        value: r.value,
    }));
    const result: Row[] = [];
    for (let i = 0; i < trimmed.length; i++) {
        const r = trimmed[i]!;
        const isLast = i === trimmed.length - 1;
        if (r.key === "" && trimmed.length > 1 && !isLast) {
            continue;
        }
        result.push(r);
    }
    if (result.length === 0) {
        return [{ key: "", value: "" }];
    }
    const last = result[result.length - 1]!;
    if (last.key !== "") {
        result.push({ key: "", value: "" });
    }
    return result;
}

/** Last row wins for duplicate keys. */
function rowsToRecord(rows: Row[]): Record<string, string> {
    const out: Record<string, string> = {};
    for (const r of rows) {
        const k = r.key.trim();
        if (k === "") continue;
        out[k] = r.value;
    }
    return out;
}

const localRows = ref<Row[]>([{ key: "", value: "" }]);
let syncingFromParent = false;

function snapshotRecord(rec: Record<string, string>): string {
    const keys = Object.keys(rec).sort();
    return JSON.stringify(
        keys.reduce(
            (acc, k) => {
                acc[k] = rec[k] ?? "";
                return acc;
            },
            {} as Record<string, string>,
        ),
    );
}

watch(
    () => snapshotRecord(props.modelValue),
    (snap) => {
        const localSnap = snapshotRecord(rowsToRecord(localRows.value));
        if (snap === localSnap) return;
        syncingFromParent = true;
        localRows.value = finalizeRows(recordToRows({ ...props.modelValue }));
        void nextTick(() => {
            syncingFromParent = false;
        });
    },
    { immediate: true },
);

function emitRecord() {
    if (syncingFromParent) return;
    emit("update:modelValue", rowsToRecord(localRows.value));
}

function onKeyInput(index: number, v: string | number) {
    const s = String(v);
    const next = localRows.value.map((r, i) =>
        i === index ? { ...r, key: s } : r,
    );
    localRows.value = next;
}

function onKeyBlur(index: number) {
    const next = localRows.value.map((r, i) =>
        i === index ? { ...r, key: r.key.trim() } : r,
    );
    localRows.value = finalizeRows(next);
    emitRecord();
}

function onValueInput(index: number, v: string | number) {
    const s = String(v);
    const next = localRows.value.map((r, i) =>
        i === index ? { ...r, value: s } : r,
    );
    localRows.value = next;
    emitRecord();
}
</script>

<template>
    <div :class="cn('flex flex-col gap-0.5', props.class)">
        <div v-for="(row, index) in localRows" :key="index" class="flex gap-2">
            <Input
                :model-value="row.key"
                class="font-mono text-[11px] h-8 flex-[40%]"
                :disabled="disabled"
                placeholder="Key"
                spellcheck="false"
                @update:model-value="onKeyInput(index, $event)"
                @blur="onKeyBlur(index)"
            />
            <Input
                :model-value="row.value"
                class="font-mono text-[11px] h-8"
                :disabled="disabled"
                placeholder="Value"
                spellcheck="false"
                @update:model-value="onValueInput(index, $event)"
            />
        </div>
    </div>
</template>
