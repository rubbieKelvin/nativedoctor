<script setup lang="ts">
import type { WorkspaceSnapshot } from "@/api";
import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarGroup,
    SidebarGroupLabel,
    SidebarHeader,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    SidebarSeparator,
} from "@/components/ui/sidebar";
import { IconInnerShadowTop } from "@tabler/icons-vue";
import { computed } from "vue";

const props = defineProps<{
    workspace: WorkspaceSnapshot | null;
    loadErr: string | null;
    activeId: string | null;
}>();

const emit = defineEmits<{
    openFile: [path: string, kind: "request" | "script", title: string];
}>();

const multiRoot = computed(() => (props.workspace?.roots.length ?? 0) > 1);
</script>

<template>
    <Sidebar collapsible="none">
        <SidebarHeader>
            <SidebarMenu>
                <SidebarMenuItem>
                    <SidebarMenuButton
                        class="data-[slot=sidebar-menu-button]:p-1.5!"
                    >
                        <IconInnerShadowTop class="size-5 shrink-0" />
                        <span class="text-sm">NativeDoctor</span>
                    </SidebarMenuButton>
                </SidebarMenuItem>
            </SidebarMenu>
        </SidebarHeader>

        <SidebarContent>
            <div
                v-if="loadErr"
                class="text-destructive px-2 text-sm"
            >
                {{ loadErr }}
            </div>

            <SidebarGroup>
                <SidebarGroupLabel>Requests</SidebarGroupLabel>
                <SidebarMenu>
                    <template
                        v-for="g in workspace?.requests ?? []"
                        :key="'rq-' + g.root_index"
                    >
                        <SidebarMenuItem
                            v-if="multiRoot"
                            class="pointer-events-none"
                        >
                            <span
                                class="text-sidebar-foreground/70 px-2 py-0.5 text-[10px]"
                                >{{ g.root_label }}</span
                            >
                        </SidebarMenuItem>
                        <SidebarMenuItem
                            v-for="e in g.entries"
                            :key="e.path"
                        >
                            <SidebarMenuButton
                                :is-active="activeId === e.path"
                                :tooltip="e.path"
                                class="h-8 text-xs font-normal"
                                @click="
                                    emit('openFile', e.path, 'request', e.name)
                                "
                            >
                                <span class="truncate">{{ e.name }}</span>
                            </SidebarMenuButton>
                        </SidebarMenuItem>
                    </template>
                </SidebarMenu>
            </SidebarGroup>

            <SidebarSeparator />

            <SidebarGroup>
                <SidebarGroupLabel>Scripts</SidebarGroupLabel>
                <SidebarMenu>
                    <template
                        v-for="g in workspace?.scripts ?? []"
                        :key="'sc-' + g.root_index"
                    >
                        <SidebarMenuItem
                            v-if="multiRoot"
                            class="pointer-events-none"
                        >
                            <span
                                class="text-sidebar-foreground/70 px-2 py-0.5 text-[10px]"
                                >{{ g.root_label }}</span
                            >
                        </SidebarMenuItem>
                        <SidebarMenuItem
                            v-for="e in g.entries"
                            :key="e.path"
                        >
                            <SidebarMenuButton
                                :is-active="activeId === e.path"
                                :tooltip="e.path"
                                class="h-8 text-xs font-normal"
                                @click="
                                    emit('openFile', e.path, 'script', e.name)
                                "
                            >
                                <span class="truncate">{{ e.name }}</span>
                            </SidebarMenuButton>
                        </SidebarMenuItem>
                    </template>
                </SidebarMenu>
            </SidebarGroup>
        </SidebarContent>

        <SidebarFooter>
            <p class="text-sm text-muted-foreground">v0.0.0</p>
        </SidebarFooter>
    </Sidebar>
</template>
