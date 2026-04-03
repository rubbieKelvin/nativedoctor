<script setup lang="ts">
import { computed, ref } from "vue";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table";

const props = defineProps<{
    logs: { level: string; message: string; elapsed_ms: number }[];
    error: string | null;
}>();

const filterText = ref("");
const levelFilter = ref<"all" | string>("all");

const filtered = computed(() => {
    let rows = props.logs;
    const q = filterText.value.trim().toLowerCase();
    if (q) {
        rows = rows.filter(
            (l) =>
                l.message.toLowerCase().includes(q) ||
                l.level.toLowerCase().includes(q),
        );
    }
    if (levelFilter.value !== "all") {
        rows = rows.filter(
            (l) =>
                l.level.toLowerCase() === levelFilter.value.toLowerCase(),
        );
    }
    return rows;
});

const levels = computed(() => {
    const s = new Set(props.logs.map((l) => l.level));
    return Array.from(s).sort();
});

function badgeVariant(
    level: string,
): "default" | "secondary" | "destructive" | "outline" {
    const u = level.toLowerCase();
    if (u === "error" || u === "fatal") return "destructive";
    if (u === "warn" || u === "warning") return "secondary";
    return "outline";
}

function copyAll() {
    const text = filtered.value
        .map(
            (l) =>
                `${l.elapsed_ms}ms [${l.level}] ${l.message}`,
        )
        .join("\n");
    void navigator.clipboard.writeText(text);
}
</script>

<template>
    <div
        class="flex min-h-[10rem] max-h-[42vh] flex-col border-t border-border bg-muted/30"
    >
        <div
            class="flex flex-wrap items-center gap-2 border-b border-border px-2 py-1.5"
        >
            <span class="text-xs font-semibold">Script output</span>
            <Input
                v-model="filterText"
                class="h-7 max-w-xs font-mono text-xs"
                placeholder="Filter messages…"
            />
            <Select v-model="levelFilter">
                <SelectTrigger class="h-7 w-[7.5rem] text-xs">
                    <SelectValue placeholder="Level" />
                </SelectTrigger>
                <SelectContent>
                    <SelectItem value="all">All levels</SelectItem>
                    <SelectItem
                        v-for="lv in levels"
                        :key="lv"
                        :value="lv"
                    >
                        {{ lv }}
                    </SelectItem>
                </SelectContent>
            </Select>
            <Button
                size="sm"
                variant="secondary"
                class="h-7 text-xs"
                :disabled="filtered.length === 0"
                @click="copyAll"
            >
                Copy all
            </Button>
            <Badge v-if="error" variant="destructive" class="text-[10px]">{{
                error
            }}</Badge>
        </div>
        <ScrollArea class="min-h-0 flex-1">
            <Table>
                <TableHeader>
                    <TableRow>
                        <TableHead class="w-16 font-mono text-[10px]"
                            >ms</TableHead
                        >
                        <TableHead class="w-24 font-mono text-[10px]"
                            >Level</TableHead
                        >
                        <TableHead class="font-mono text-[10px]"
                            >Message</TableHead
                        >
                    </TableRow>
                </TableHeader>
                <TableBody>
                    <TableRow
                        v-for="(row, i) in filtered"
                        :key="i"
                        class="font-mono text-[11px]"
                    >
                        <TableCell class="text-muted-foreground align-top">{{
                            row.elapsed_ms
                        }}</TableCell>
                        <TableCell class="align-top">
                            <Badge
                                :variant="badgeVariant(row.level)"
                                class="px-1.5 py-0 text-[10px] uppercase"
                                >{{ row.level }}</Badge
                            >
                        </TableCell>
                        <TableCell class="whitespace-pre-wrap break-all align-top">{{
                            row.message
                        }}</TableCell>
                    </TableRow>
                </TableBody>
            </Table>
            <p
                v-if="filtered.length === 0"
                class="text-muted-foreground p-3 text-xs"
            >
                No log lines match.
            </p>
        </ScrollArea>
    </div>
</template>
