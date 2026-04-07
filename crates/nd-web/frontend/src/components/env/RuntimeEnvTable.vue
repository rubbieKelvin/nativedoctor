<script setup lang="ts">
import { computed, ref } from "vue";
import type { RuntimeEnvEntry } from "@/api";
import { useExecutionStore } from "@/stores/execution";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table";
import { storeToRefs } from "pinia";

const execution = useExecutionStore();
const { runtimeEnvEntries } = storeToRefs(execution);

const filter = ref("");

const filtered = computed(() => {
    const q = filter.value.trim().toLowerCase();
    const rows = runtimeEnvEntries.value;
    if (!q) return rows;
    return rows.filter(
        (e) =>
            e.key.toLowerCase().includes(q) ||
            e.value.toLowerCase().includes(q),
    );
});

function copyRow(e: RuntimeEnvEntry) {
    void navigator.clipboard.writeText(`${e.key}=${e.value}`);
}

function copyAll() {
    const text = filtered.value.map((e) => `${e.key}=${e.value}`).join("\n");
    void navigator.clipboard.writeText(text);
}
</script>

<template>
    <div class="flex min-h-0 flex-1 flex-col gap-2 p-2">
        <p class="text-muted-foreground text-[11px] leading-snug">
            Built from the last run’s streamed events (<code class="rounded bg-muted px-1"
                >RuntimeVariablesInitialized</code
            >
            /
            <code class="rounded bg-muted px-1">RuntimeVariablePushed</code>). Used
            for URL/header templates and Rhai
            <code class="rounded bg-muted px-1">env()</code>. Values may be sensitive.
        </p>
        <div class="flex shrink-0 flex-wrap gap-2">
            <Input
                v-model="filter"
                class="h-8 min-w-0 flex-1 font-mono text-xs"
                placeholder="Filter keys or values…"
                spellcheck="false"
            />
            <Button
                size="sm"
                variant="secondary"
                class="shrink-0"
                :disabled="filtered.length === 0"
                @click="copyAll"
            >
                Copy all
            </Button>
        </div>
        <ScrollArea class="min-h-0 flex-1 rounded-md border border-border">
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead class="w-[28%] font-mono text-xs"
                            >Key</TableHead
                        >
                        <TableHead class="font-mono text-xs">Value</TableHead>
                        <TableHead class="w-14 text-xs" />
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <TableRow
                        v-for="row in filtered"
                        :key="row.key"
                        class="font-mono text-[11px]"
                    >
                        <TableCell class="align-top break-all">{{
                            row.key
                        }}</TableCell>
                        <TableCell class="align-top break-all">{{
                            row.value
                        }}</TableCell>
                        <TableCell>
                            <Button
                                size="sm"
                                variant="ghost"
                                class="h-7 px-2"
                                @click="copyRow(row)"
                            >
                                Copy
                            </Button>
                        </TableCell>
                    </TableRow>
                </TableBody>
            </Table>
        </ScrollArea>
        <p class="text-muted-foreground text-[11px]">
            {{ filtered.length }} / {{ runtimeEnvEntries.length }} shown
        </p>
    </div>
</template>
