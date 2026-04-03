<script setup lang="ts">
import type { AppModel } from "@/composables/useAppModel";
import { Button } from "@/components/ui/button";
import ScriptLogViewer from "@/components/script/ScriptLogViewer.vue";

defineProps<{
    app: AppModel;
}>();
</script>

<template>
    <div class="flex min-h-0 min-w-0 flex-1 flex-col">
        <div
            class="flex shrink-0 flex-wrap items-center gap-2 border-b border-border bg-background px-2 py-1.5"
        >
            <Button
                size="sm"
                :disabled="app.sending"
                @click="app.doRunScript"
            >
                {{ app.sending ? "Running…" : "Run script" }}
            </Button>
            <span v-if="app.sendErr" class="text-xs text-destructive">{{
                app.sendErr
            }}</span>
        </div>
        <textarea
            v-model="app.scriptRaw"
            class="border-input bg-background focus-visible:ring-ring min-h-48 w-full flex-1 resize-y border-0 p-3 font-mono text-xs shadow-none focus-visible:outline-none focus-visible:ring-1"
            spellcheck="false"
        />
        <ScriptLogViewer
            :logs="app.scriptLogs"
            :error="app.scriptRunError"
        />
    </div>
</template>
