<script setup lang="ts">
import { Badge } from "@/components/ui/badge";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import ResponseBody from "./ResponseBody.vue";
import ResponseCookies from "./ResponseCookies.vue";
import ResponseHeaders from "./ResponseHeaders.vue";

defineProps<{
    status?: number;
    headers?: [string, string][];
    body?: string;
    durationMs?: number;
    error?: string;
}>();

function statusVariant(
    status: number,
): "default" | "secondary" | "destructive" | "outline" {
    if (status >= 200 && status < 300) return "default";
    if (status >= 400) return "destructive";
    return "secondary";
}
</script>

<template>
    <div class="rounded-md border border-border bg-muted/20">
        <div
            class="flex flex-wrap items-center gap-2 border-b border-border px-3 py-2"
        >
            <Badge v-if="status != null" :variant="statusVariant(status)">
                {{ status }}
            </Badge>
            <span
                v-if="durationMs != null"
                class="text-muted-foreground text-sm"
            >
                {{ durationMs }} ms
            </span>
            <span v-if="error" class="text-destructive text-sm">
                {{ error }}
            </span>
            <span
                v-if="status == null && !error"
                class="text-muted-foreground text-sm"
            >
                No response yet
            </span>
        </div>
        <div
            v-if="(headers?.length || body != null) && !error"
            class="flex flex-1 flex-col p-3"
        >
            <Tabs default-value="headers" class="w-full">
                <TabsList
                    class="w-full rounded-none flex items-start justify-baseline"
                >
                    <TabsTrigger class="request-tab-trigger" value="headers">
                        Headers
                    </TabsTrigger>
                    <TabsTrigger class="request-tab-trigger" value="body">
                        Body
                    </TabsTrigger>
                    <TabsTrigger class="request-tab-trigger" value="cookies">
                        Cookies
                    </TabsTrigger>
                </TabsList>
                <TabsContent value="headers" class="mt-2">
                    <ResponseHeaders :headers="headers ?? []" />
                </TabsContent>
                <TabsContent value="body" class="mt-2">
                    <ResponseBody :body="body" />
                </TabsContent>
                <TabsContent value="cookies" class="mt-2">
                    <ResponseCookies :headers="headers ?? []" />
                </TabsContent>
            </Tabs>
        </div>
        <div v-else-if="error" class="p-3">
            <p class="text-destructive text-sm">
                {{ error }}
            </p>
        </div>
    </div>
</template>
