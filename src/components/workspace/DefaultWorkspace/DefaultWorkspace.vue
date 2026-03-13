<script setup lang="ts">
import { computed } from "vue";
import SideBar from "./SideBar.vue";
import HttpResourcePad from "@/components/resourcepads/http/HttpResourcePad.vue";
import SequencePad from "@/components/resourcepads/sequence/SequencePad.vue";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/components/ui/tabs";
import { Button } from "@/components/ui/button";
import { Globe, ListOrdered, X } from "lucide-vue-next";
import {
    ResizableHandle,
    ResizablePanel,
    ResizablePanelGroup,
} from "@/components/ui/resizable";
import WmDragHandle from "@/components/WmDragHandle.vue";
import { useCurrentProjectActions } from "@/store/project";
import { useWorkspaceTabs } from "@/store/workspaceTabs";

const project = useCurrentProjectActions();
const workspaceTabs = useWorkspaceTabs();

const tabsModel = computed({
    get: () => workspaceTabs.activeTabId ?? undefined,
    set: (v: string | undefined) => workspaceTabs.setActiveTab(v ?? null),
});

function resourceForId(id: string) {
    return project.getResourceById(id);
}
</script>

<template>
    <div class="w-full h-full">
        <WmDragHandle />
        <ResizablePanelGroup direction="horizontal" class="w-full h-full">
            <ResizablePanel :default-size="25" :min-size="20" :max-size="30">
                <SideBar />
            </ResizablePanel>

            <ResizableHandle />

            <ResizablePanel :default-size="75" class="flex flex-col">
                <div
                    v-if="workspaceTabs.openTabIds.length === 0"
                    class="flex flex-1 items-center justify-center text-muted-foreground"
                >
                    Select a resource to open
                </div>
                <Tabs
                    v-else
                    v-model="tabsModel"
                    class="flex h-full flex-col"
                >
                    <TabsList class="w-full justify-start rounded-none border-b bg-transparent p-0">
                        <TabsTrigger
                            v-for="id in workspaceTabs.openTabIds"
                            :key="id"
                            :value="id"
                            class="relative rounded-none border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:shadow-none"
                        >
                            <component
                                :is="resourceForId(id)?.type === 'sequence' ? ListOrdered : Globe"
                                class="mr-1.5 size-4 shrink-0"
                            />
                            <span class="truncate max-w-[120px]">
                                {{ resourceForId(id)?.name || 'Untitled' }}
                            </span>
                            <Button
                                variant="ghost"
                                size="icon"
                                class="ml-1 size-6 rounded hover:bg-muted"
                                @click.stop="workspaceTabs.closeTab(id)"
                            >
                                <X class="size-3.5" />
                            </Button>
                        </TabsTrigger>
                    </TabsList>
                    <div class="min-h-0 flex-1 overflow-auto">
                        <TabsContent
                            v-for="id in workspaceTabs.openTabIds"
                            :key="id"
                            :value="id"
                            class="mt-0 h-full data-[state=inactive]:hidden"
                        >
                            <div v-if="!resourceForId(id)" class="flex h-full flex-col items-center justify-center gap-2 p-8 text-muted-foreground">
                                <p>Resource not found</p>
                                <Button variant="outline" size="sm" @click="workspaceTabs.closeTab(id)">
                                    Close tab
                                </Button>
                            </div>
                            <HttpResourcePad
                                v-else-if="resourceForId(id)?.type === 'http'"
                                :resource="resourceForId(id) as import('@/shared/types/resources').HttpResource"
                            />
                            <SequencePad
                                v-else-if="resourceForId(id)?.type === 'sequence'"
                                :resource="resourceForId(id)! as import('@/shared/types/resources').SequenceResource"
                            />
                        </TabsContent>
                    </div>
                </Tabs>
            </ResizablePanel>
        </ResizablePanelGroup>
    </div>
</template>
