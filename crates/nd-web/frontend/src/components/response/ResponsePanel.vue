<script setup lang="ts">
import { ref } from "vue";
import type { AppModel } from "@/composables/useAppModel";
import RuntimeEnvTable from "@/components/env/RuntimeEnvTable.vue";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
    Collapsible,
    CollapsibleContent,
    CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

const props = defineProps<{
    app: AppModel;
}>();

const outputSection = ref<"response" | "runtime-env">("response");
</script>

<template>
    <div
        class="flex h-full min-h-0 min-w-0 flex-col border-t border-border bg-background"
    >
        <Tabs
            v-model="outputSection"
            class="flex min-h-0 min-w-0 flex-1 flex-col gap-0 overflow-hidden"
        >
            <div
                class="flex flex-wrap items-center gap-2 border-b border-border px-2 py-1.5"
            >
                <TabsList class="h-8 bg-transparent p-0">
                    <TabsTrigger value="response" class="text-xs"
                        >Response</TabsTrigger
                    >
                    <TabsTrigger value="runtime-env" class="text-xs"
                        >Runtime env</TabsTrigger
                    >
                </TabsList>
                <template v-if="outputSection === 'response'">
                    <Badge
                        v-if="app.response"
                        variant="secondary"
                        class="font-mono text-[11px]"
                    >
                        {{ app.response.status }} ·
                        {{ app.response.duration_ms }}ms
                    </Badge>
                    <span
                        v-if="app.sendErr"
                        class="text-xs text-destructive"
                        >{{ app.sendErr }}</span
                    >
                </template>
            </div>

            <TabsContent
                value="response"
                class="mt-0 flex min-h-0 min-w-0 flex-1 flex-col"
            >
                <div
                    v-if="!app.response"
                    class="text-muted-foreground flex flex-1 items-center justify-center p-4 text-xs"
                >
                    Send a request to see the response body here.
                </div>
                <Tabs
                    v-else
                    v-model="app.bodyView"
                    class="flex min-h-0 flex-1 flex-col"
                >
                    <TabsList
                        class="h-8 w-full shrink-0 justify-start rounded-none border-b border-border px-2"
                    >
                        <TabsTrigger value="pretty" class="text-xs"
                            >Pretty</TabsTrigger
                        >
                        <TabsTrigger value="raw" class="text-xs"
                            >Raw</TabsTrigger
                        >
                    </TabsList>

                    <ScrollArea class="min-h-0 flex-1">
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
                                    <Button
                                        variant="ghost"
                                        size="sm"
                                        class="h-7 text-xs"
                                    >
                                        Headers ({{
                                            app.response.headers.length
                                        }})
                                    </Button>
                                </CollapsibleTrigger>
                                <CollapsibleContent>
                                    <pre
                                        class="mt-1 max-h-40 overflow-auto rounded-md border border-border bg-muted/40 p-2 font-mono text-[11px]"
                                        >{{
                                            JSON.stringify(
                                                app.response.headers,
                                                null,
                                                2,
                                            )
                                        }}</pre
                                    >
                                </CollapsibleContent>
                            </Collapsible>
                        </div>
                    </ScrollArea>
                </Tabs>
            </TabsContent>

            <TabsContent
                value="runtime-env"
                class="mt-0 flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden"
            >
                <RuntimeEnvTable :app="app" />
            </TabsContent>
        </Tabs>
    </div>
</template>
