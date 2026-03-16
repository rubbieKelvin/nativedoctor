<script setup lang="ts">
import { computed } from "vue";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
    getPreviewComponent,
    normalizeContentType,
} from "./response-body/preview-map";

const props = withDefaults(
    defineProps<{
        body?: string;
        contentType?: string;
    }>(),
    { contentType: "" },
);

const normalizedType = computed(() =>
    normalizeContentType(props.contentType || ""),
);

const previewComponent = computed(() =>
    getPreviewComponent(props.contentType || ""),
);

const hasPreview = computed(() => !!previewComponent.value);
</script>

<template>
    <div class="flex-1 flex flex-col min-h-0 space-y-1">
        <Tabs default-value="raw" class="w-full flex-1 flex flex-col">
            <TabsList class="w-fit rounded-md">
                <TabsTrigger value="raw">Raw</TabsTrigger>
                <TabsTrigger value="preview">Preview</TabsTrigger>
            </TabsList>
            <TabsContent
                value="raw"
                class="flex-1 mt-2 flex flex-col min-h-0 data-[state=inactive]:hidden"
            >
                <p
                    v-if="body == null || body === ''"
                    class="text-muted-foreground text-sm"
                >
                    No body
                </p>
                <ScrollArea
                    v-else
                    class="h-full w-full rounded font-mono text-xs"
                >
                    <pre
                        class="whitespace-pre-wrap wrap-break-words p-2 select-auto"
                        >{{ body }}</pre
                    >
                </ScrollArea>
            </TabsContent>
            <TabsContent
                value="preview"
                class="flex-1 mt-2 flex flex-col min-h-0 data-[state=inactive]:hidden"
            >
                <template v-if="body == null || body === ''">
                    <p class="text-muted-foreground text-sm">No body</p>
                </template>
                <template v-else-if="hasPreview">
                    <component :is="previewComponent" :body="body" />
                </template>
                <template v-else>
                    <p class="text-muted-foreground text-sm">
                        No preview for
                        <code class="rounded bg-muted px-1">{{
                            normalizedType || "(unknown type)"
                        }}</code
                        >. Use Raw to view the response.
                    </p>
                </template>
            </TabsContent>
        </Tabs>
    </div>
</template>
