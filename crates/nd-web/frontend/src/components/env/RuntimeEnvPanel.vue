<script setup lang="ts">
import { computed, ref, watch } from "vue";
import { fetchRuntimeEnv, type RuntimeEnvEntry } from "@/api";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
    Sheet,
    SheetContent,
    SheetDescription,
    SheetHeader,
    SheetTitle,
    SheetTrigger,
} from "@/components/ui/sheet";
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table";
const open = ref(false);
const entries = ref<RuntimeEnvEntry[]>([]);
const err = ref<string | null>(null);
const loading = ref(false);
const filter = ref("");

const filtered = computed(() => {
    const q = filter.value.trim().toLowerCase();
    if (!q) return entries.value;
    return entries.value.filter(
        (e) =>
            e.key.toLowerCase().includes(q) ||
            e.value.toLowerCase().includes(q),
    );
});

async function load() {
    loading.value = true;
    err.value = null;
    try {
        const r = await fetchRuntimeEnv();
        entries.value = r.entries;
    } catch (e) {
        err.value = e instanceof Error ? e.message : String(e);
        entries.value = [];
    } finally {
        loading.value = false;
    }
}

watch(open, (v) => {
    if (v) void load();
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
    <Sheet v-model:open="open">
        <SheetTrigger as-child>
            <Button
                size="sm"
                variant="outline"
                class="h-7 text-xs"
                title="Inspect runtime variables (.env merge, persistence, Rhai set)"
            >
                Env
            </Button>
        </SheetTrigger>
        <SheetContent class="flex w-full flex-col sm:max-w-lg">
            <SheetHeader>
                <SheetTitle>Runtime environment</SheetTitle>
                <SheetDescription>
                    In-memory map used for URL/header template expansion and
                    Rhai <code>env()</code>. Values may be sensitive.
                </SheetDescription>
            </SheetHeader>
                <div class="flex shrink-0 gap-2 py-2">
                    <Input
                        v-model="filter"
                        class="h-8 font-mono text-xs"
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
                    <Button
                        size="sm"
                        variant="ghost"
                        class="shrink-0"
                        :disabled="loading"
                        @click="load"
                    >
                        Refresh
                    </Button>
                </div>
                <p v-if="err" class="text-sm text-destructive">{{ err }}</p>
                <p v-else-if="loading" class="text-muted-foreground text-xs">
                    Loading…
                </p>
                <ScrollArea class="min-h-0 flex-1 rounded-md border">
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead class="w-[28%] font-mono text-xs"
                                    >Key</TableHead
                                >
                                <TableHead class="font-mono text-xs"
                                    >Value</TableHead
                                >
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
                <p class="text-muted-foreground pt-2 text-[11px]">
                    {{ filtered.length }} / {{ entries.length }} shown
                </p>
        </SheetContent>
    </Sheet>
</template>
