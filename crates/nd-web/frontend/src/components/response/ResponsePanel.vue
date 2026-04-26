<script setup lang="ts">
import { computed, ref } from "vue";
import { storeToRefs } from "pinia";
import { useEditorStore } from "@/stores/editor";
import { useExecutionStore } from "@/stores/execution";
import RuntimeEnvTable from "@/components/env/RuntimeEnvTable.vue";
import { Badge } from "@/components/ui/badge";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table";

const editor = useEditorStore();
const execution = useExecutionStore();
const { response, bodyView, prettyResponse } = storeToRefs(execution);

const activePath = computed(() => editor.activeTab?.path);

const sendErr = computed(() => {
    const p = activePath.value;
    if (!p) return null;
    return execution.sendErrByPath[p] ?? null;
});

const outputSection = ref<"response" | "headers" | "runtime-env">("response");
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
                class="flex flex-wrap items-center gap-2 border-b border-border px-3 py-2"
            >
                <TabsList class="h-8 bg-transparent p-0">
                    <TabsTrigger value="response" class="text-sm font-medium">
                        Response
                    </TabsTrigger>
                    <TabsTrigger value="headers" class="text-sm font-medium">
                        Headers
                    </TabsTrigger>
                    <TabsTrigger value="runtime-env" class="text-sm font-medium">
                        Runtime env
                    </TabsTrigger>
                </TabsList>
                <template
                    v-if="outputSection === 'response' || outputSection === 'headers'"
                >
                    <Badge
                        v-if="response"
                        variant="secondary"
                        class="text-xs font-medium tabular-nums"
                    >
                        {{ response.status }} · {{ response.duration_ms }}ms
                    </Badge>
                    <span
                        v-if="sendErr"
                        class="text-sm text-destructive"
                        >{{ sendErr }}</span
                    >
                </template>
            </div>

            <TabsContent
                value="response"
                class="mt-0 flex min-h-0 min-w-0 flex-1 flex-col"
            >
                <div
                    v-if="!response"
                    class="text-muted-foreground flex flex-1 items-center justify-center p-6 text-sm"
                >
                    Send a request to see the response body here.
                </div>
                <Tabs
                    v-else
                    v-model="bodyView"
                    class="flex min-h-0 flex-1 flex-col"
                >
                    <TabsList
                        class="h-8 w-full shrink-0 justify-start rounded-none border-b border-border px-3"
                    >
                        <TabsTrigger value="pretty" class="text-xs font-medium">
                            Pretty
                        </TabsTrigger>
                        <TabsTrigger value="raw" class="text-xs font-medium">
                            Raw
                        </TabsTrigger>
                    </TabsList>

                    <ScrollArea class="min-h-0 flex-1">
                        <div class="p-3">
                            <TabsContent value="pretty" class="mt-0">
                                <pre
                                    class="whitespace-pre-wrap break-all font-mono text-[11px] leading-relaxed"
                                    >{{ prettyResponse }}</pre
                                >
                            </TabsContent>
                            <TabsContent value="raw" class="mt-0">
                                <pre
                                    class="whitespace-pre-wrap break-all font-mono text-[11px] leading-relaxed"
                                    >{{
                                        response.body_text ??
                                        (response.body_base64
                                            ? "[binary base64]"
                                            : "")
                                    }}</pre
                                >
                            </TabsContent>
                        </div>
                    </ScrollArea>
                </Tabs>
            </TabsContent>

            <TabsContent
                value="headers"
                class="mt-0 flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden"
            >
                <div
                    v-if="!response"
                    class="text-muted-foreground flex flex-1 items-center justify-center p-6 text-sm"
                >
                    Send a request to see response headers here.
                </div>
                <ScrollArea v-else class="min-h-0 flex-1 rounded-md">
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead
                                    class="text-muted-foreground w-[28%] text-xs font-medium"
                                >
                                    Name
                                </TableHead>
                                <TableHead
                                    class="text-muted-foreground text-xs font-medium"
                                >
                                    Value
                                </TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            <TableRow
                                v-for="(pair, i) in response.headers"
                                :key="i"
                                class="font-mono text-xs leading-relaxed"
                            >
                                <TableCell class="align-top break-all">{{
                                    pair[0]
                                }}</TableCell>
                                <TableCell class="align-top break-all">{{
                                    pair[1]
                                }}</TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                </ScrollArea>
            </TabsContent>

            <TabsContent
                value="runtime-env"
                class="mt-0 flex min-h-0 min-w-0 flex-1 flex-col overflow-hidden"
            >
                <RuntimeEnvTable />
            </TabsContent>
        </Tabs>
    </div>
</template>
