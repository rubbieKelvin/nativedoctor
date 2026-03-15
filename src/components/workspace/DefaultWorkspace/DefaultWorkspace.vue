<script setup lang="ts">
import { ref, computed, watch, nextTick } from "vue";
import { useResizeObserver } from "@vueuse/core";
import SideBar from "./SideBar.vue";
import HttpResourcePad from "@/components/resourcepads/http/HttpResourcePad.vue";
import SequencePad from "@/components/resourcepads/sequence/SequencePad.vue";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "@/components/ui/tabs";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import {
    Globe,
    ListOrdered,
    X,
    ChevronLeft,
    ChevronRight,
} from "lucide-vue-next";
import {
    ResizableHandle,
    ResizablePanel,
    ResizablePanelGroup,
} from "@/components/ui/resizable";
import WmDragHandle from "@/components/WmDragHandle.vue";
import { useResources } from "@/store/resources";
import { useWorkspaceTabs, useWorkspaceTabsActions } from "@/store/tabs";
import { useCurrentProject } from "@/store/project";

const { name: projectName } = useCurrentProject();
const resourcesStore = useResources();
const { openTabIds, activeTabId } = useWorkspaceTabs();
const { setActiveTab, closeTab } = useWorkspaceTabsActions();

const scrollAreaRef = ref<InstanceType<typeof ScrollArea> | null>(null);
const tabListScrollRef = ref<HTMLElement | null>(null);
const canScrollLeft = ref(false);
const canScrollRight = ref(false);

const THRESHOLD = 2;

function updateTabListScrollVisibility() {
    const el = tabListScrollRef.value;
    if (!el) return;
    const { scrollLeft, clientWidth, scrollWidth } = el;
    canScrollLeft.value = scrollLeft > THRESHOLD;
    canScrollRight.value = scrollLeft + clientWidth < scrollWidth - THRESHOLD;
}

function syncViewportRef() {
    tabListScrollRef.value = scrollAreaRef.value?.getViewportElement() ?? null;
    updateTabListScrollVisibility();
}

function onTabListScroll(e: Event) {
    if (!tabListScrollRef.value && e.target instanceof HTMLElement) {
        tabListScrollRef.value = e.target;
    }
    updateTabListScrollVisibility();
}

useResizeObserver(tabListScrollRef, () => {
    updateTabListScrollVisibility();
});

watch(
    () => openTabIds.value.length,
    () => {
        nextTick(syncViewportRef);
    },
    { immediate: true },
);

const tabsModel = computed({
    get: () => activeTabId.value ?? undefined,
    set: (v: string | undefined) => setActiveTab(v ?? null),
});

function resourceForId(id: string) {
    return resourcesStore.getResourceById(id);
}

function httpResourceForId(id: string) {
    return resourcesStore.getHttpResource(id);
}

function sequenceResourceForId(id: string) {
    return resourcesStore.getSequenceResource(id);
}
</script>

<template>
    <div class="w-full h-full">
        <WmDragHandle :title="projectName" />
        <ResizablePanelGroup direction="horizontal" class="w-full h-full">
            <ResizablePanel :default-size="25" :min-size="20" :max-size="30">
                <SideBar />
            </ResizablePanel>

            <ResizableHandle />

            <ResizablePanel :default-size="75" class="flex flex-col">
                <div
                    v-if="openTabIds.length === 0"
                    class="flex flex-1 items-center justify-center text-muted-foreground"
                >
                    Select a resource to open
                </div>
                <Tabs
                    v-else
                    v-model="tabsModel"
                    class="flex h-full flex-col gap-0"
                >
                    <div
                        class="relative overflow-hidden border-b bg-transparent"
                    >
                        <ScrollArea
                            ref="scrollAreaRef"
                            orientation="horizontal"
                            :scroll-hide-delay="100"
                            type="scroll"
                            @scroll="onTabListScroll"
                        >
                            <TabsList
                                class="inline-flex min-w-0 shrink-0 flex-nowrap justify-start rounded-none border-0 bg-transparent p-0"
                            >
                                <TabsTrigger
                                    v-for="id in openTabIds"
                                    :key="id"
                                    :value="id"
                                    class="relative shrink-0 rounded-none border-b-2 border-transparent data-[state=active]:border-primary data-[state=active]:shadow-none"
                                >
                                    <component
                                        :is="
                                            resourceForId(id)?.type ===
                                            'sequence'
                                                ? ListOrdered
                                                : Globe
                                        "
                                        class="mr-1.5 size-4 shrink-0"
                                    />
                                    <span class="truncate max-w-30">
                                        {{
                                            resourceForId(id)?.name ||
                                            "Untitled"
                                        }}
                                    </span>
                                    <Button
                                        variant="ghost"
                                        size="icon"
                                        class="ml-1 size-6 rounded hover:bg-muted"
                                        @click.stop="closeTab(id)"
                                    >
                                        <X class="size-3.5" />
                                    </Button>
                                </TabsTrigger>
                            </TabsList>
                        </ScrollArea>
                        <div
                            v-show="canScrollLeft"
                            class="pointer-events-none absolute left-0 top-0 bottom-0 flex w-6 items-center bg-linear-to-r from-background to-transparent"
                        >
                            <ChevronLeft
                                class="size-4 shrink-0 text-muted-foreground"
                            />
                        </div>
                        <div
                            v-show="canScrollRight"
                            class="pointer-events-none absolute right-0 top-0 bottom-0 flex w-6 items-center justify-end bg-linear-to-l from-background to-transparent"
                        >
                            <ChevronRight
                                class="size-4 shrink-0 text-muted-foreground"
                            />
                        </div>
                    </div>

                    <div class="min-h-0 flex-1 overflow-auto">
                        <TabsContent
                            v-for="id in openTabIds"
                            :key="id"
                            :value="id"
                            class="mt-0 h-full data-[state=inactive]:hidden"
                        >
                            <div
                                v-if="!resourceForId(id)"
                                class="flex h-full flex-col items-center justify-center gap-2 p-8 text-muted-foreground"
                            >
                                <p>Resource not found</p>
                                <Button
                                    variant="outline"
                                    size="sm"
                                    @click="closeTab(id)"
                                >
                                    Close tab
                                </Button>
                            </div>

                            <HttpResourcePad
                                v-else-if="httpResourceForId(id)"
                                :resource="httpResourceForId(id)!"
                            />
                            <SequencePad
                                v-else-if="sequenceResourceForId(id)"
                                :resource="sequenceResourceForId(id)!"
                            />
                        </TabsContent>
                    </div>
                </Tabs>
            </ResizablePanel>
        </ResizablePanelGroup>
    </div>
</template>
