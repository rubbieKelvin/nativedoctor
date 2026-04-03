<script setup lang="ts">
import type { AppModel } from "@/composables/useAppModel";
import { Card, CardContent } from "@/components/ui/card";
import {
    Tabs,
    TabsContent,
    TabsList,
    TabsTrigger,
} from "@/components/ui/tabs";

defineProps<{
    app: AppModel;
}>();
</script>

<template>
    <Tabs v-model="app.reqSubTab" class="flex min-h-0 flex-1 flex-col gap-0">
        <TabsList
            class="h-9 w-full shrink-0 justify-start rounded-none border-b border-border bg-background px-2"
        >
            <TabsTrigger value="params" class="text-xs">Params</TabsTrigger>
            <TabsTrigger value="headers" class="text-xs">Headers</TabsTrigger>
            <TabsTrigger value="body" class="text-xs">Body</TabsTrigger>
            <TabsTrigger value="input" class="text-xs">Input</TabsTrigger>
            <TabsTrigger value="auth" class="text-xs">Auth</TabsTrigger>
        </TabsList>

        <div class="min-h-0 flex-1 overflow-auto p-2">
            <p
                v-if="app.activeTab?.parseError"
                class="mb-2 text-sm text-destructive"
            >
                {{ app.activeTab.parseError }} — fix document
            </p>

            <TabsContent value="params" class="mt-0">
                <Card>
                    <CardContent class="p-2 pt-3">
                        <textarea
                            v-model="app.queryJson"
                            class="border-input bg-background focus-visible:ring-ring h-40 w-full resize-y rounded-md border p-2 font-mono text-[11px] shadow-sm focus-visible:outline-none focus-visible:ring-1"
                            spellcheck="false"
                        />
                    </CardContent>
                </Card>
            </TabsContent>

            <TabsContent value="headers" class="mt-0">
                <Card>
                    <CardContent class="p-2 pt-3">
                        <textarea
                            v-model="app.headersJson"
                            class="border-input bg-background focus-visible:ring-ring h-40 w-full resize-y rounded-md border p-2 font-mono text-[11px] shadow-sm focus-visible:outline-none focus-visible:ring-1"
                            spellcheck="false"
                        />
                    </CardContent>
                </Card>
            </TabsContent>

            <TabsContent value="body" class="mt-0">
                <Card>
                    <CardContent class="p-2 pt-3">
                        <textarea
                            v-model="app.bodyText"
                            class="border-input bg-background focus-visible:ring-ring h-48 w-full resize-y rounded-md border p-2 font-mono text-[11px] shadow-sm focus-visible:outline-none focus-visible:ring-1"
                            spellcheck="false"
                        />
                    </CardContent>
                </Card>
            </TabsContent>

            <TabsContent value="input" class="mt-0">
                <Card>
                    <CardContent class="space-y-2 p-2 pt-3">
                        <p class="text-muted-foreground text-[11px] leading-snug">
                            JSON object of per-send variable overrides (merged
                            with runtime env; same as CLI). Stringify values;
                            applied to URL, headers, and body template
                            expansion.
                        </p>
                        <p
                            v-if="app.overridesJsonError"
                            class="text-sm text-destructive"
                        >
                            {{ app.overridesJsonError }}
                        </p>
                        <textarea
                            v-model="app.overridesJson"
                            class="border-input bg-background focus-visible:ring-ring h-40 w-full resize-y rounded-md border p-2 font-mono text-[11px] shadow-sm focus-visible:outline-none focus-visible:ring-1"
                            spellcheck="false"
                            placeholder='{ "ID": "42", "TOKEN": "secret" }'
                        />
                    </CardContent>
                </Card>
            </TabsContent>

            <TabsContent value="auth" class="mt-0">
                <p class="text-muted-foreground text-xs">
                    Use <strong>Headers</strong> for
                    <code class="rounded bg-muted px-1">Authorization</code>,
                    or reference runtime variables in the URL/body.
                </p>
            </TabsContent>
        </div>
    </Tabs>
</template>
