<script setup lang="ts">
import type { AppModel } from "@/composables/useAppModel";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
    Collapsible,
    CollapsibleContent,
    CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

defineProps<{
    app: AppModel;
}>();
</script>

<template>
    <div
        class="flex max-h-[45vh] min-h-[8rem] shrink-0 flex-col border-t border-border bg-background"
    >
        <div
            class="flex flex-wrap items-center gap-2 border-b border-border px-2 py-1.5"
        >
            <span class="text-xs font-semibold">Response</span>
            <Badge
                v-if="app.response"
                variant="secondary"
                class="font-mono text-[11px]"
            >
                {{ app.response.status }} · {{ app.response.duration_ms }}ms
            </Badge>
            <span v-if="app.sendErr" class="text-xs text-destructive">{{
                app.sendErr
            }}</span>
        </div>

        <Tabs v-model="app.bodyView" class="flex min-h-0 flex-1 flex-col">
            <TabsList
                class="h-8 w-full shrink-0 justify-start rounded-none border-b border-border px-2"
            >
                <TabsTrigger value="pretty" class="text-xs">Pretty</TabsTrigger>
                <TabsTrigger value="raw" class="text-xs">Raw</TabsTrigger>
            </TabsList>

            <ScrollArea v-if="app.response" class="min-h-0 flex-1">
                <div class="p-2">
                    <TabsContent value="pretty" class="mt-0">
                        <pre
                            class="whitespace-pre-wrap break-all font-mono text-[11px] leading-relaxed"
                            >{{ app.prettyResponse }}</pre
                        >
                    </TabsContent>
                    <TabsContent value="raw" class="mt-0">
                        <pre
                            class="whitespace-pre-wrap break-all font-mono text-[11px] leading-relaxed"
                            >{{
                                app.response.body_text ??
                                (app.response.body_base64
                                    ? "[binary base64]"
                                    : "")
                            }}</pre
                        >
                    </TabsContent>

                    <Collapsible class="mt-3">
                        <CollapsibleTrigger as-child>
                            <Button variant="ghost" size="sm" class="h-7 text-xs">
                                Headers ({{ app.response.headers.length }})
                            </Button>
                        </CollapsibleTrigger>
                        <CollapsibleContent>
                            <pre
                                class="bg-muted/50 mt-1 max-h-40 overflow-auto rounded-md p-2 font-mono text-[11px]"
                                >{{
                                    JSON.stringify(app.response.headers, null, 2)
                                }}</pre
                            >
                        </CollapsibleContent>
                    </Collapsible>
                </div>
            </ScrollArea>
        </Tabs>
    </div>
</template>
