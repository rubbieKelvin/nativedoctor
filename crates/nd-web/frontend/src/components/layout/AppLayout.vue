<script setup lang="ts">
import {
    ResizableHandle,
    ResizablePanel,
    ResizablePanelGroup,
} from "@/components/ui/resizable";
import { SidebarProvider } from "@/components/ui/sidebar";
</script>

<template>
    <SidebarProvider
        class="flex h-full min-h-0 font-sans text-sm antialiased"
    >
        <ResizablePanelGroup
            id="nd-app-shell"
            direction="horizontal"
            auto-save-id="nd-app-shell"
            class="h-full min-h-0 w-full"
        >
            <ResizablePanel :default-size="20" :min-size="14" :max-size="45">
                <slot name="sidebar" />
            </ResizablePanel>
            <ResizableHandle with-handle />
            <ResizablePanel :default-size="80" :min-size="40">
                <!-- Panel is a flex item, not a flex container: fill it with h-full so nested flex-1 chains work -->
                <div
                    class="bg-background border-border flex h-full min-h-0 min-w-0 flex-col border-l"
                >
                    <slot />
                </div>
            </ResizablePanel>
        </ResizablePanelGroup>
    </SidebarProvider>
</template>
