<script setup lang="ts">
import type { AppModel } from "@/composables/useAppModel";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";

defineProps<{
    app: AppModel;
}>();
</script>

<template>
    <div
        class="flex flex-wrap items-center gap-2 border-b border-border bg-background px-2 py-1.5"
    >
        <select
            v-if="app.requestSpec"
            class="border-input bg-background h-9 rounded-md border px-2 font-mono text-xs shadow-sm focus-visible:ring-1 focus-visible:ring-ring"
            :value="String(app.requestSpec.method ?? 'GET')"
            @change="
                app.setMethod(($event.target as HTMLSelectElement).value)
            "
        >
            <option v-for="m in app.HTTP_METHODS" :key="m" :value="m">
                {{ m }}
            </option>
        </select>
        <Input
            v-if="app.requestSpec"
            class="min-w-0 flex-1 font-mono text-xs"
            :model-value="String(app.requestSpec.url ?? '')"
            placeholder="URL"
            spellcheck="false"
            @update:model-value="app.setUrl(String($event))"
        />
        <Button
            size="sm"
            :disabled="app.sending"
            @click="app.doSend"
        >
            {{ app.sending ? "Sending…" : "Send" }}
        </Button>
    </div>
</template>
