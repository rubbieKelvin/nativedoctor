<script setup lang="ts">
import type { AppModel } from "@/composables/useAppModel";
import { Button } from "@/components/ui/button";
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";
import { ChevronDown } from "lucide-vue-next";

defineProps<{
    app: AppModel;
}>();
</script>

<template>
    <div
        class="flex flex-wrap items-center gap-2 border-b border-border bg-background px-2 py-1.5"
    >
        <DropdownMenu v-if="app.requestSpec">
            <DropdownMenuTrigger as-child>
                <Button
                    variant="outline"
                    size="sm"
                    class="h-9 min-w-[6rem] justify-between gap-1 font-mono text-xs"
                >
                    {{ String(app.requestSpec.method ?? "GET") }}
                    <ChevronDown class="h-3.5 w-3.5 opacity-60" />
                </Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent align="start" class="min-w-[8rem]">
                <DropdownMenuItem
                    v-for="m in app.HTTP_METHODS"
                    :key="m"
                    class="cursor-pointer font-mono text-xs"
                    @click="app.setMethod(m)"
                >
                    {{ m }}
                </DropdownMenuItem>
            </DropdownMenuContent>
        </DropdownMenu>
        <Input
            v-if="app.requestSpec"
            class="min-w-0 flex-1 font-mono text-xs"
            :model-value="String(app.requestSpec.url ?? '')"
            placeholder="URL"
            spellcheck="false"
            @update:model-value="app.setUrl(String($event))"
        />
        <Button size="sm" :disabled="app.sending" @click="app.doSend">
            {{ app.sending ? "Sending…" : "Send" }}
        </Button>
    </div>
</template>
