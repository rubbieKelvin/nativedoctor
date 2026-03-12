<script setup lang="ts">
import SideBar from "./SideBar.vue";
import HttpResourcePad from "@/components/resourcepads/http/HttpResourcePad.vue";

import {
    ResizableHandle,
    ResizablePanel,
    ResizablePanelGroup,
} from "@/components/ui/resizable";
import WmDragHandle from "@/components/WmDragHandle.vue";

defineProps<{
    projectPath: string;
}>();
</script>

<template>
    <div class="w-full h-full">
        <WmDragHandle />
        <ResizablePanelGroup direction="horizontal" class="w-full h-full">
            <ResizablePanel :default-size="25" :min-size="20" :max-size="30">
                <SideBar :project-path="projectPath" />
            </ResizablePanel>

            <ResizableHandle />

            <ResizablePanel :default-size="75">
                <HttpResourcePad />
            </ResizablePanel>
        </ResizablePanelGroup>
    </div>
</template>
