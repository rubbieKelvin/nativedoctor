<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import RuntimeEnvPanel from "@/components/env/RuntimeEnvPanel.vue";
import type { WorkspaceSnapshot } from "@/api";

defineProps<{
    workspace: WorkspaceSnapshot | null;
    loadErr: string | null;
    activeId: string | null;
}>();

const emit = defineEmits<{
    openFile: [path: string, kind: "request" | "script", title: string];
}>();
</script>

<template>
    <aside
        class="flex w-56 shrink-0 flex-col border-r border-border bg-background"
    >
        <div
            class="flex items-center justify-between gap-1 border-b border-border px-2 py-1.5"
        >
            <span
                class="text-[11px] font-medium uppercase tracking-wide text-muted-foreground"
                >NativeDoctor</span
            >
            <RuntimeEnvPanel />
        </div>
        <div v-if="loadErr" class="p-2 text-sm text-destructive">
            {{ loadErr }}
        </div>
        <ScrollArea class="min-h-0 flex-1">
            <div class="p-1">
                <div
                    class="px-2 py-1 text-[11px] font-semibold text-muted-foreground"
                >
                    Requests
                </div>
                <template
                    v-for="g in workspace?.requests ?? []"
                    :key="'rq-' + g.root_index"
                >
                    <div
                        v-if="(workspace?.roots.length ?? 0) > 1"
                        class="px-2 py-0.5 text-[10px] text-muted-foreground/80"
                    >
                        {{ g.root_label }}
                    </div>
                    <Button
                        v-for="e in g.entries"
                        :key="e.path"
                        variant="ghost"
                        size="sm"
                        class="h-7 w-full justify-start px-2 font-normal"
                        :class="activeId === e.path ? 'bg-accent' : ''"
                        :title="e.path"
                        @click="emit('openFile', e.path, 'request', e.name)"
                    >
                        <span class="truncate">{{ e.name }}</span>
                    </Button>
                </template>

                <Separator class="my-2" />

                <div
                    class="px-2 py-1 text-[11px] font-semibold text-muted-foreground"
                >
                    Scripts
                </div>
                <template
                    v-for="g in workspace?.scripts ?? []"
                    :key="'sc-' + g.root_index"
                >
                    <div
                        v-if="(workspace?.roots.length ?? 0) > 1"
                        class="px-2 py-0.5 text-[10px] text-muted-foreground/80"
                    >
                        {{ g.root_label }}
                    </div>
                    <Button
                        v-for="e in g.entries"
                        :key="e.path"
                        variant="ghost"
                        size="sm"
                        class="h-7 w-full justify-start px-2 font-normal"
                        :class="activeId === e.path ? 'bg-accent' : ''"
                        :title="e.path"
                        @click="emit('openFile', e.path, 'script', e.name)"
                    >
                        <span class="truncate">{{ e.name }}</span>
                    </Button>
                </template>
            </div>
        </ScrollArea>
    </aside>
</template>
