<script setup lang="ts">
import { useEditorStore } from "@/stores/editor";
import { useExecutionStore } from "@/stores/execution";
import { Button } from "@/components/ui/button";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";
import { storeToRefs } from "pinia";
import { ChevronDown } from "lucide-vue-next";

const editor = useEditorStore();
const execution = useExecutionStore();
const { requestSpec } = storeToRefs(editor);
const { sending } = storeToRefs(execution);
</script>

<template>
    <div
        class="flex flex-wrap items-center gap-2 border-b border-border bg-background px-2 py-1.5"
    >
        <DropdownMenu v-if="requestSpec">
            <DropdownMenuTrigger as-child>
                <Button
                    variant="outline"
                    size="sm"
                    class="h-9 min-w-[6rem] justify-between gap-1 font-mono text-xs"
                >
                    {{ String(requestSpec.method ?? "GET") }}
                    <ChevronDown class="h-3.5 w-3.5 opacity-60" />
                </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="start" class="min-w-[8rem]">
                <DropdownMenuItem
                    v-for="m in editor.HTTP_METHODS"
                    :key="m"
                    class="cursor-pointer font-mono text-xs"
                    @click="editor.setMethod(m)"
                >
                    {{ m }}
                </DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
        <Input
            v-if="requestSpec"
            class="min-w-0 flex-1 font-mono text-xs"
            :model-value="String(requestSpec.url ?? '')"
            placeholder="URL"
            spellcheck="false"
            @update:model-value="editor.setUrl(String($event))"
        />
        <Button size="sm" :disabled="sending" @click="execution.doSend">
            {{ sending ? "Sending…" : "Send" }}
        </Button>
    </div>
</template>
