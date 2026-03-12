<script setup lang="ts">
import SideBar from "./SideBar.vue";
import HttpResourcePad from "@/components/resourcepads/http/HttpResourcePad.vue";

import {
    ResizableHandle,
    ResizablePanel,
    ResizablePanelGroup,
} from "@/components/ui/resizable";

defineProps<{
    projectPath: string;
}>();
</script>

<template>
    <ResizablePanelGroup
        direction="horizontal"
        class="max-w-md rounded-lg border md:min-w-112.5"
    >
        <ResizablePanel :default-size="50">
            <SideBar />
        </ResizablePanel>

        <ResizableHandle />

        <ResizablePanel :default-size="50">
            <HttpResourcePad />
        </ResizablePanel>
    </ResizablePanelGroup>
</template>
