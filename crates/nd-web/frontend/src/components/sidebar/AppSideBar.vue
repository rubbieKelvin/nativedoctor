<script setup lang="ts">
import type { GroupedFiles, WorkspaceSnapshot } from "@/api";
import {
    Sidebar,
    SidebarContent,
    SidebarFooter,
    SidebarGroup,
    SidebarGroupLabel,
    SidebarHeader,
    SidebarInput,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
    SidebarSeparator,
} from "@/components/ui/sidebar";
import { IconInnerShadowTop } from "@tabler/icons-vue";
import { Search } from "lucide-vue-next";
import { computed, ref } from "vue";

const VITE_NATIVEDOCTOR_VERSION = import.meta.env.VITE_NATIVEDOCTOR_VERSION;

const props = defineProps<{
    workspace: WorkspaceSnapshot | null;
    loadErr: string | null;
    activeId: string | null;
}>();

const emit = defineEmits<{
    openFile: [path: string, kind: "request" | "script", title: string];
}>();

const multiRoot = computed(() => (props.workspace?.roots.length ?? 0) > 1);

const searchQuery = ref("");

function filterGrouped(groups: GroupedFiles[], qRaw: string): GroupedFiles[] {
    const q = qRaw.trim().toLowerCase();
    if (!q) return groups;
    return groups
        .map((g) => ({
            ...g,
            entries: g.entries.filter(
                (e) =>
                    e.name.toLowerCase().includes(q) ||
                    e.path.toLowerCase().includes(q),
            ),
        }))
        .filter((g) => g.entries.length > 0);
}

const filteredRequests = computed(() =>
    filterGrouped(props.workspace?.requests ?? [], searchQuery.value),
);

const filteredScripts = computed(() =>
    filterGrouped(props.workspace?.scripts ?? [], searchQuery.value),
);
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
            <div
                class="text-sidebar-foreground/50 flex items-center gap-1.5 px-2 pb-2"
            >
                <Search class="size-3.5 shrink-0" aria-hidden="true" />
                <SidebarInput
                    v-model="searchQuery"
                    class="min-w-0 flex-1 font-sans text-xs"
                    placeholder="Search requests & scripts…"
                    type="search"
                    autocomplete="off"
                    spellcheck="false"
                />
            </div>
        </SidebarHeader>

        <SidebarContent>
            <div v-if="loadErr" class="text-destructive px-2 text-sm">
                {{ loadErr }}
            </div>

            <SidebarGroup>
                <SidebarGroupLabel>Requests</SidebarGroupLabel>
                <SidebarMenu>
                    <template
                        v-for="g in filteredRequests"
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
                        <SidebarMenuItem v-for="e in g.entries" :key="e.path">
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
                        v-for="g in filteredScripts"
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
                        <SidebarMenuItem v-for="e in g.entries" :key="e.path">
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
            <p class="text-sm text-muted-foreground">
                v{{ VITE_NATIVEDOCTOR_VERSION }}
            </p>
        </SidebarFooter>
    </Sidebar>
</template>
